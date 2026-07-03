mod commands;
mod hdc;
mod models;

use crate::hdc::client::HdcClient;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let hdc_client = HdcClient::new().unwrap_or_else(|_| HdcClient::with_path("hdc".to_string()));

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(hdc_client)
        .invoke_handler(tauri::generate_handler![
            commands::device::list_devices,
            commands::device::connect_device,
            commands::device::disconnect_device,
            commands::device::get_device_info,
            commands::hap::install_hap,
            commands::hap::validate_hap,
        ])
        .run(tauri::generate_context!())
        .expect("启动应用失败");
}
