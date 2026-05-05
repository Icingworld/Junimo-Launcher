// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod commands;
mod core;

use std::path::PathBuf;

use core::db::{init::init_db, init_sqlite_pool};
use sqlx::SqlitePool;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

fn setup_error<E: std::error::Error + 'static>(e: E) -> tauri::Error {
    let boxed: Box<dyn std::error::Error> = Box::new(e);
    tauri::Error::Setup(boxed.into())
}

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub app_data_dir: PathBuf,
    pub app_mods_dir: PathBuf,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // 初始化数据库（存放在 app_data_dir）
            let app_data_dir = app
                .path()
                .app_data_dir()
                .map_err(setup_error)?;
            std::fs::create_dir_all(&app_data_dir).map_err(|e| {
                setup_error(e)
            })?;

            let db_path = app_data_dir.join("junimo-launcher.sqlite");
            let app_mods_dir = app_data_dir.join("Mods");
            std::fs::create_dir_all(&app_mods_dir).map_err(|e| {
                setup_error(e)
            })?;

            let pool = tauri::async_runtime::block_on(async {
                let pool = init_sqlite_pool(&db_path, 10).await?;
                init_db(&pool).await?;
                Ok::<_, sqlx::Error>(pool)
            })
            .map_err(setup_error)?;

            app.manage(AppState {
                pool,
                app_data_dir,
                app_mods_dir,
            });

            let show_i = MenuItemBuilder::with_id("show", "显示").build(app)?;
            let hide_i = MenuItemBuilder::with_id("hide", "隐藏").build(app)?;
            let quit_i = MenuItemBuilder::with_id("quit", "退出").build(app)?;

            let menu = MenuBuilder::new(app)
                .items(&[&show_i, &hide_i, &quit_i])
                .build()?;

            let _tray = TrayIconBuilder::new()
                .icon(tauri::include_image!("icons/32x32.png"))
                .menu(&menu)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "quit" => app.exit(0),
                    "hide" => {
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.hide();
                        }
                    }
                    "show" => {
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::game::start_game,
            commands::mods::mods_list,
            commands::mods::mods_add,
            commands::mods::mods_delete,
            commands::mods::mods_set_enabled,
            commands::settings::settings_get_game_path,
            commands::settings::settings_set_game_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
