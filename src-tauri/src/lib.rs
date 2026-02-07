use tauri::{
    menu::{Menu, MenuItem, CheckMenuItem}, // Added CheckMenuItem
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Manager,
    image::Image,
};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_autostart::ManagerExt; // Required to access .autolaunch()
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    const DRAG_SCRIPT: &str = include_str!(".././drag.js");

    tauri::Builder::default()
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
            // --- LOAD TRAY ICONS ---
            #[cfg(target_os = "macos")]
            let tray_icon = Image::from_bytes(include_bytes!("../icons/tray.png")).unwrap();

            #[cfg(not(target_os = "macos"))]
            let tray_icon = Image::from_bytes(include_bytes!("../icons/32x32.png")).unwrap();

            // --- CREATE MENU ITEMS ---
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let toggle_i = MenuItem::with_id(app, "toggle", "Show/Hide", true, None::<&str>)?;
            
            // 1. Create the Checkbox Item
            let autostart_i = CheckMenuItem::with_id(app, "autostart", "Run on Startup", true, false, None::<&str>)?;

            // 2. Check current state and update checkbox
            let autostart_manager = app.autolaunch();
            if autostart_manager.is_enabled().unwrap_or(false) {
                autostart_i.set_checked(true)?;
            }

            // 3. Add to Menu
            let menu = Menu::with_items(app, &[&toggle_i, &autostart_i, &quit_i])?;

            // --- TRAY SETUP ---
            #[allow(unused_mut)]
            let mut tray_builder = TrayIconBuilder::new()
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
                        // 4. Handle the Toggle Logic
                        let manager = app.autolaunch();
                        if manager.is_enabled().unwrap_or(false) {
                            let _ = manager.disable();
                            autostart_i.set_checked(false).unwrap();
                        } else {
                            let _ = manager.enable();
                            autostart_i.set_checked(true).unwrap();
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
            {
                tray_builder = tray_builder.icon_as_template(true);
            }

            tray_builder.build(app)?;

            // --- GLOBAL SHORTCUT ---
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