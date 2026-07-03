import { invoke } from '@tauri-apps/api/core'
import type { Device } from '../types/device'

export async function invokeListDevices(): Promise<Device[]> {
  try {
    return await invoke<Device[]>('list_devices')
  } catch (err) {
    throw new Error(`获取设备列表失败: ${err}`)
  }
}

export async function invokeConnectDevice(ip: string, port: number): Promise<Device> {
  try {
    return await invoke<Device>('connect_device', { ip, port })
  } catch (err) {
    throw new Error(`设备连接失败: ${err}`)
  }
}

export async function invokeDisconnectDevice(id: string): Promise<void> {
  try {
    await invoke<void>('disconnect_device', { id })
  } catch (err) {
    throw new Error(`断开连接失败: ${err}`)
  }
}

export async function invokeGetDeviceInfo(id: string): Promise<Device> {
  try {
    return await invoke<Device>('get_device_info', { id })
  } catch (err) {
    throw new Error(`获取设备信息失败: ${err}`)
  }
}
