use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Manager,
    image::Image,
};
use tauri_plugin_autostart::MacosLauncher;
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
                // FIX 1: Added underscore to `_shortcut` to fix warning
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

            // FIX 2: Use a PNG for Windows (ICO causes the crash!)
            // We use the 32x32.png that Tauri generates by default
            #[cfg(not(target_os = "macos"))]
            let tray_icon = Image::from_bytes(include_bytes!("../icons/32x32.png")).unwrap();

            // FIX 3: Removed `mut` because Windows doesn't need to modify the builder
            let tray_builder = TrayIconBuilder::new()
                .icon(tray_icon)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
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

            // --- MACOS SPECIFIC ---
            // We have to use a separate block for the mutable logic to avoid the warning on Windows
            #[cfg(target_os = "macos")]
            let tray_builder = tray_builder.icon_as_template(true);

            // --- CREATE MENU ---
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let toggle_i = MenuItem::with_id(app, "toggle", "Show/Hide", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&toggle_i, &quit_i])?;

            // Finish building
            tray_builder
                .menu(&menu)
                .build(app)?;

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