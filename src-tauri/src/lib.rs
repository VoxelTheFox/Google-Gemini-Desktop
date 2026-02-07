use tauri::{
    menu::{Menu, MenuItem, CheckMenuItem},
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
                // We only need to tell the App to be an "Accessory" (Hidden from Dock)
                // We DO NOT touch the window decorations anymore.
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

            let menu = Menu::with_items(app, &[&toggle_i, &autostart_i, &quit_i])?;

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
            let hotkey = "Command+G";
            #[cfg(not(target_os = "macos"))]
            let hotkey = "Alt+Space";
            
            app.global_shortcut().register(hotkey)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}