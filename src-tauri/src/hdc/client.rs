use crate::models::device::{ConnectionType, Device, DeviceStatus};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct HdcClient {
    hdc_path: String,
}

impl HdcClient {
    pub fn new() -> Result<Self, String> {
        let path = find_hdc_binary()?;
        Ok(Self { hdc_path: path })
    }

    pub fn with_path(hdc_path: String) -> Self {
        Self { hdc_path }
    }

    pub fn list_devices(&self) -> Result<Vec<Device>, String> {
        let output = self.execute(&["list", "targets"])?;
        let raw_ids = parse_target_list(&output);

        let mut devices = Vec::new();
        for id in raw_ids {
            let device = self.get_device_detail(&id).unwrap_or_else(|_| Device {
                id: id.clone(),
                name: id.clone(),
                model: String::from("未知"),
                os_version: String::from("未知"),
                sn: String::from("未知"),
                connection_type: detect_connection_type(&id),
                status: DeviceStatus::Online,
                battery_level: None,
            });
            devices.push(device);
        }

        Ok(devices)
    }

    pub fn connect_device(&self, ip: &str, port: u16) -> Result<Device, String> {
        let target = format!("{}:{}", ip, port);
        self.execute(&["tconn", &target])?;

        let device_id = target;
        self.get_device_detail(&device_id)
    }

    pub fn disconnect_device(&self, id: &str) -> Result<(), String> {
        self.execute(&["tdisconn", id])?;
        Ok(())
    }

    pub fn get_device_info(&self, id: &str) -> Result<Device, String> {
        self.get_device_detail(id)
    }

    fn get_device_detail(&self, id: &str) -> Result<Device, String> {
        let output = self.execute(&["-t", id, "shell", "getprop"])?;

        let name = extract_prop(&output, "ro.product.name").unwrap_or_else(|| id.to_string());
        let model =
            extract_prop(&output, "ro.product.model").unwrap_or_else(|| String::from("未知"));
        let os_version = extract_prop(&output, "ro.build.version.release")
            .or_else(|| extract_prop(&output, "hw_sc.build.platform.version"))
            .unwrap_or_else(|| String::from("未知"));
        let sn = extract_prop(&output, "ro.serialno")
            .or_else(|| extract_prop(&output, "ohos.boot.sn"))
            .unwrap_or_else(|| String::from("未知"));

        let battery_str = extract_prop(&output, "hw.battery.capacity");
        let battery_level = battery_str.and_then(|s| s.parse::<u8>().ok());

        Ok(Device {
            id: id.to_string(),
            name,
            model,
            os_version,
            sn,
            connection_type: detect_connection_type(id),
            status: DeviceStatus::Online,
            battery_level,
        })
    }

    fn execute(&self, args: &[&str]) -> Result<String, String> {
        let output = Command::new(&self.hdc_path)
            .args(args)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .output()
            .map_err(|e| format!("执行 HDC 命令失败: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("HDC 命令执行错误: {}", stderr.trim()));
        }

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        if stdout.trim().is_empty() {
            return Err(String::from("HDC 命令返回空结果"));
        }

        Ok(stdout)
    }
}

fn find_hdc_binary() -> Result<String, String> {
    let candidates = ["hdc", "/usr/local/bin/hdc", "/opt/hdc/hdc"];

    for path in &candidates {
        let result = Command::new(path)
            .arg("version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();

        if result.is_ok() {
            return Ok(path.to_string());
        }
    }

    Err(String::from(
        "未检测到 HDC 工具，请确保 HDC 已安装并添加到系统 PATH 环境变量中",
    ))
}

fn parse_target_list(output: &str) -> Vec<String> {
    output
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty() && !line.starts_with('['))
        .collect()
}

fn extract_prop(output: &str, prop_name: &str) -> Option<String> {
    let prefix = format!("[{}]:", prop_name);

    for line in output.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(&prefix) {
            let value_part = trimmed.strip_prefix(&prefix)?;
            let value = value_part.trim();
            let value = value.strip_prefix('[').unwrap_or(value);
            let value = value.strip_suffix(']').unwrap_or(value);
            return Some(value.to_string());
        }
    }

    None
}

fn detect_connection_type(id: &str) -> ConnectionType {
    if id.contains(':') {
        ConnectionType::WiFi
    } else {
        ConnectionType::USB
    }
}
