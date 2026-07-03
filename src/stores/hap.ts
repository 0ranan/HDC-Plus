import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { InstallTask } from '../types/hap'
import { invokeInstallHap, invokeValidateHap } from '../utils/hap'

export const useHapStore = defineStore('hap', () => {
  const selectedHapPath = ref<string | null>(null)
  const targetDeviceId = ref<string | null>(null)
  const currentTask = ref<InstallTask | null>(null)
  const installing = ref(false)
  const errorMessage = ref<string | null>(null)
  const taskHistory = ref<InstallTask[]>([])

  function selectHap(path: string | null) {
    selectedHapPath.value = path
    errorMessage.value = null
  }

  function selectTargetDevice(deviceId: string | null) {
    targetDeviceId.value = deviceId
  }

  async function validateAndSetHap(path: string) {
    errorMessage.value = null
    try {
      await invokeValidateHap(path)
      selectedHapPath.value = path
      return true
    } catch (err) {
      errorMessage.value = err instanceof Error ? err.message : 'HAP 文件校验失败'
      return false
    }
  }

  async function installHap(): Promise<InstallTask> {
    if (!selectedHapPath.value) {
      throw new Error('请先选择 HAP 安装包')
    }
    if (!targetDeviceId.value) {
      throw new Error('请先选择目标设备')
    }

    installing.value = true
    errorMessage.value = null
    currentTask.value = null

    try {
      const task = await invokeInstallHap(targetDeviceId.value, selectedHapPath.value)
      currentTask.value = task
      taskHistory.value.unshift(task)
      return task
    } catch (err) {
      const msg = err instanceof Error ? err.message : 'HAP 安装失败'
      errorMessage.value = msg
      throw err
    } finally {
      installing.value = false
    }
  }

  function clearCurrentTask() {
    currentTask.value = null
  }

  return {
    selectedHapPath,
    targetDeviceId,
    currentTask,
    installing,
    errorMessage,
    taskHistory,
    selectHap,
    selectTargetDevice,
    validateAndSetHap,
    installHap,
    clearCurrentTask,
  }
})
