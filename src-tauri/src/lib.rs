use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Manager, PhysicalPosition, WindowEvent};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

fn toggle_widget(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("widget") {
        if w.is_visible().unwrap_or(false) { let _ = w.hide(); } else { let _ = w.show(); }
    }
}

#[tauri::command]
fn set_widget(app: AppHandle, visible: bool) {
    if let Some(w) = app.get_webview_window("widget") {
        if visible { let _ = w.show(); } else { let _ = w.hide(); }
    }
}

#[tauri::command]
fn place_widget(app: AppHandle, corner: String) {
    if let Some(w) = app.get_webview_window("widget") {
        if let (Ok(Some(m)), Ok(ws)) = (w.current_monitor(), w.outer_size()) {
            let pad = (16.0 * m.scale_factor()) as i32;
            let bar = (48.0 * m.scale_factor()) as i32;
            let (mw, mh) = (m.size().width as i32, m.size().height as i32);
            let x = if corner.ends_with('l') { m.position().x + pad } else { m.position().x + mw - ws.width as i32 - pad };
            let y = if corner.starts_with('t') { m.position().y + pad } else { m.position().y + mh - ws.height as i32 - pad - bar };
            let _ = w.set_position(PhysicalPosition::new(x, y));
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, None))
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _s, e| { if e.state() == ShortcutState::Pressed { toggle_widget(app); } })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![set_widget, place_widget])
        .setup(|app| {
            let open = MenuItem::with_id(app, "open", "Open Day Planner", true, None::<&str>)?;
            let wid = MenuItem::with_id(app, "wid", "Show or hide widget", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open, &wid, &quit])?;
            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, ev| match ev.id.as_ref() {
                    "open" => { if let Some(w) = app.get_webview_window("main") { let _ = w.show(); let _ = w.set_focus(); } }
                    "wid" => toggle_widget(app),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;
            let _ = app.global_shortcut().register(Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyW));
            let _ = app.autolaunch().enable();
            Ok(())
        })
        .on_window_event(|w, e| {
            if let WindowEvent::CloseRequested { api, .. } = e {
                if w.label() == "main" { let _ = w.hide(); api.prevent_close(); }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running Day Planner");
}
