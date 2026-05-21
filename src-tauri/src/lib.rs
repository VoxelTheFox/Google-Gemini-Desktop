use tauri::{
    menu::{Menu, MenuItem, CheckMenuItem, Submenu},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Manager,
    image::Image,
    command,
    Window,
};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

// --- SAFE IMPORTS ---
#[cfg(target_os = "macos")]
use objc::{msg_send, sel, sel_impl};
#[cfg(target_os = "macos")]
use objc::runtime::{Object, Class};

#[command]
fn start_drag(window: Window) {
    let _ = window.start_dragging();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    const DRAG_SCRIPT: &str = include_str!("../drag.js");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_drag])
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag"]),
        ))
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        let window = app.get_webview_window("main").unwrap();
                        if window.is_visible().unwrap() {
                            window.hide().unwrap();
                        } else {
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                    }
                })
                .build(),
        )
        .on_page_load(|webview, _payload| {
            webview.eval(DRAG_SCRIPT).expect("Failed to inject drag script");
        })
        .setup(|app| {
            // ---------------------------------------------------------
            // 1. MACOS: HIDE FROM DOCK ONLY
            // ---------------------------------------------------------
            #[cfg(target_os = "macos")]
            unsafe {
                let cls = Class::get("NSApplication").unwrap();
                let app_instance: *mut Object = msg_send![cls, sharedApplication];
                let _: () = msg_send![app_instance, setActivationPolicy: 1]; // 1 = Accessory
            }

            // ---------------------------------------------------------
            // 2. TRAY ICONS
            // ---------------------------------------------------------
            #[cfg(target_os = "macos")]
            let tray_icon = Image::from_bytes(include_bytes!("../icons/tray.png")).unwrap();
            #[cfg(not(target_os = "macos"))]
            let tray_icon = Image::from_bytes(include_bytes!("../icons/32x32.png")).unwrap();

            // ---------------------------------------------------------
            // 3. MENU & TRAY
            // ---------------------------------------------------------
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let toggle_i = MenuItem::with_id(app, "toggle", "Show/Hide", true, None::<&str>)?;
            let autostart_i = CheckMenuItem::with_id(app, "autostart", "Run on Startup", true, false, None::<&str>)?;

            let autostart_manager = app.autolaunch();
            if autostart_manager.is_enabled().unwrap_or(false) {
                let _ = autostart_i.set_checked(true);
            }

            #[cfg(target_os = "windows")]
            let current_shortcut = {
                let config_dir = app.path().app_config_dir().unwrap_or_else(|_| std::env::current_dir().unwrap());
                let config_path = config_dir.join("config.json");
                let mut loaded_shortcut = "Alt+Space".to_string();
                if let Ok(content) = std::fs::read_to_string(&config_path) {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                        if let Some(shortcut_val) = json.get("shortcut").and_then(|v| v.as_str()) {
                            loaded_shortcut = shortcut_val.to_string();
                        }
                    }
                }
                loaded_shortcut
            };

            #[cfg(target_os = "windows")]
            let (shortcut_menu, alt_space_i, ctrl_space_i) = {
                let alt_i = CheckMenuItem::with_id(app, "alt_space", "Alt+Space", true, false, None::<&str>)?;
                let ctrl_i = CheckMenuItem::with_id(app, "ctrl_space", "Control+Space", true, false, None::<&str>)?;
                let sub = Submenu::with_items(app, "Activation Shortcut", true, &[&alt_i, &ctrl_i])?;
                (sub, alt_i, ctrl_i)
            };

            #[cfg(target_os = "windows")]
            {
                println!("Loaded activation shortcut: {}", current_shortcut);
                if current_shortcut == "Alt+Space" {
                    let _ = alt_space_i.set_checked(true);
                } else {
                    let _ = ctrl_space_i.set_checked(true);
                }
            }

            #[cfg(target_os = "windows")]
            let menu = Menu::with_items(app, &[&toggle_i, &autostart_i, &shortcut_menu, &quit_i])?;
            #[cfg(not(target_os = "windows"))]
            let menu = Menu::with_items(app, &[&toggle_i, &autostart_i, &quit_i])?;

            #[cfg(target_os = "windows")]
            let alt_space_i_clone = alt_space_i.clone();
            #[cfg(target_os = "windows")]
            let ctrl_space_i_clone = ctrl_space_i.clone();

            let tray_builder = TrayIconBuilder::new()
                .menu(&menu)
                .icon(tray_icon)
                .show_menu_on_left_click(false)
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "quit" => app.exit(0),
                    "toggle" => {
                        let window = app.get_webview_window("main").unwrap();
                        if window.is_visible().unwrap() {
                            window.hide().unwrap();
                        } else {
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                    }
                    "autostart" => {
                         let manager = app.autolaunch();
                         if manager.is_enabled().unwrap_or(false) {
                             let _ = manager.disable();
                             let _ = autostart_i.set_checked(false);
                         } else {
                             let _ = manager.enable();
                             let _ = autostart_i.set_checked(true);
                         }
                    }
                    #[cfg(target_os = "windows")]
                    "alt_space" => {
                        let is_ctrl_checked = ctrl_space_i_clone.is_checked().unwrap_or(false);
                        if is_ctrl_checked {
                            let _ = alt_space_i_clone.set_checked(true);
                            let _ = ctrl_space_i_clone.set_checked(false);
                            
                            if let Ok(ctrl_shortcut) = "Control+Space".parse::<tauri_plugin_global_shortcut::Shortcut>() {
                                let _ = app.global_shortcut().unregister(ctrl_shortcut);
                            }
                            if let Ok(alt_shortcut) = "Alt+Space".parse::<tauri_plugin_global_shortcut::Shortcut>() {
                                let _ = app.global_shortcut().register(alt_shortcut);
                            }
                            
                            let config_dir = app.path().app_config_dir().unwrap_or_else(|_| std::env::current_dir().unwrap());
                            let config_path = config_dir.join("config.json");
                            let _ = std::fs::create_dir_all(&config_dir);
                            if std::fs::write(&config_path, serde_json::json!({ "shortcut": "Alt+Space" }).to_string()).is_ok() {
                                println!("Saved shortcut preference: Alt+Space to {:?}", config_path);
                            }
                        } else {
                            let _ = alt_space_i_clone.set_checked(true);
                        }
                    }
                    #[cfg(target_os = "windows")]
                    "ctrl_space" => {
                        let is_alt_checked = alt_space_i_clone.is_checked().unwrap_or(false);
                        if is_alt_checked {
                            let _ = ctrl_space_i_clone.set_checked(true);
                            let _ = alt_space_i_clone.set_checked(false);
                            
                            if let Ok(alt_shortcut) = "Alt+Space".parse::<tauri_plugin_global_shortcut::Shortcut>() {
                                let _ = app.global_shortcut().unregister(alt_shortcut);
                            }
                            if let Ok(ctrl_shortcut) = "Control+Space".parse::<tauri_plugin_global_shortcut::Shortcut>() {
                                let _ = app.global_shortcut().register(ctrl_shortcut);
                            }
                            
                            let config_dir = app.path().app_config_dir().unwrap_or_else(|_| std::env::current_dir().unwrap());
                            let config_path = config_dir.join("config.json");
                            let _ = std::fs::create_dir_all(&config_dir);
                            if std::fs::write(&config_path, serde_json::json!({ "shortcut": "Control+Space" }).to_string()).is_ok() {
                                println!("Saved shortcut preference: Control+Space to {:?}", config_path);
                            }
                        } else {
                            let _ = ctrl_space_i_clone.set_checked(true);
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click { button: MouseButton::Left, .. } => {
                        let app = tray.app_handle();
                        let window = app.get_webview_window("main").unwrap();
                        if window.is_visible().unwrap() {
                            window.hide().unwrap();
                        } else {
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                    }
                    _ => {}
                });

            #[cfg(target_os = "macos")]
            let tray_builder = tray_builder.icon_as_template(true);

            tray_builder.build(app)?;

            // ---------------------------------------------------------
            // 4. HOTKEYS
            // ---------------------------------------------------------
            #[cfg(target_os = "macos")]
            let hotkey = "Command+G".to_string();
            
            #[cfg(target_os = "windows")]
            let hotkey = current_shortcut;
            
            #[cfg(not(any(target_os = "macos", target_os = "windows")))]
            let hotkey = "Alt+Space".to_string();
            
            let parsed_shortcut: tauri_plugin_global_shortcut::Shortcut = hotkey.parse()?;
            app.global_shortcut().register(parsed_shortcut)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}