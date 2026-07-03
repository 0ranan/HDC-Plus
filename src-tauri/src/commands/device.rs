use crate::hdc::client::HdcClient;
use crate::models::device::Device;
use tauri::State;

#[tauri::command]
pub fn list_devices(hdc: State<'_, HdcClient>) -> Result<Vec<Device>, String> {
    hdc.list_devices()
}

#[tauri::command]
pub fn connect_device(ip: String, port: u16, hdc: State<'_, HdcClient>) -> Result<Device, String> {
    hdc.connect_device(&ip, port)
}

#[tauri::command]
pub fn disconnect_device(id: String, hdc: State<'_, HdcClient>) -> Result<(), String> {
    hdc.disconnect_device(&id)
}

#[tauri::command]
pub fn get_device_info(id: String, hdc: State<'_, HdcClient>) -> Result<Device, String> {
    hdc.get_device_info(&id)
}
