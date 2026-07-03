import { invoke } from '@tauri-apps/api/core'
import type { InstallTask } from '../types/hap'

export async function invokeInstallHap(deviceId: string, hapPath: string): Promise<InstallTask> {
  try {
    return await invoke<InstallTask>('install_hap', { deviceId, hapPath })
  } catch (err) {
    throw new Error(`HAP 安装失败: ${err}`)
  }
}

export async function invokeValidateHap(hapPath: string): Promise<void> {
  try {
    await invoke<void>('validate_hap', { hapPath })
  } catch (err) {
    throw new Error(`HAP 文件校验失败: ${err}`)
  }
}
