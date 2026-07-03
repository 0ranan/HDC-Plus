import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Device } from '../types/device'
import {
  invokeListDevices,
  invokeConnectDevice,
  invokeDisconnectDevice,
  invokeGetDeviceInfo,
} from '../utils/tauri'

export const useDeviceStore = defineStore('device', () => {
  const devices = ref<Device[]>([])
  const selectedDeviceId = ref<string | null>(null)
  const loading = ref(false)
  const errorMessage = ref<string | null>(null)
  let pollingTimer: ReturnType<typeof setInterval> | null = null

  const onlineDevices = computed(() =>
    devices.value.filter((d) => d.status === 'online')
  )

  const offlineDevices = computed(() =>
    devices.value.filter((d) => d.status === 'offline')
  )

  const selectedDevice = computed(() => {
    if (!selectedDeviceId.value) return null
    return devices.value.find((d) => d.id === selectedDeviceId.value) ?? null
  })

  const deviceCount = computed(() => devices.value.length)

  async function fetchDevices() {
    errorMessage.value = null
    try {
      const prevIds = new Set(devices.value.filter(d => d.status === 'online').map(d => d.id))
      const result = await invokeListDevices()
      const newIds = new Set(result.filter(d => d.status === 'online').map(d => d.id))

      for (const prevId of prevIds) {
        if (!newIds.has(prevId)) {
          const device = devices.value.find(d => d.id === prevId)
          if (device) {
            device.status = 'offline'
            if (selectedDeviceId.value === prevId) {
              selectedDeviceId.value = null
            }
          }
        }
      }

      for (const newDevice of result) {
        const existing = devices.value.find(d => d.id === newDevice.id)
        if (existing) {
          Object.assign(existing, newDevice)
        } else {
          devices.value.push(newDevice)
        }
      }
    } catch (err) {
      errorMessage.value = err instanceof Error ? err.message : '获取设备列表失败'
      throw err
    }
  }

  async function addDevice(ip: string, port: number) {
    loading.value = true
    errorMessage.value = null
    try {
      const device = await invokeConnectDevice(ip, port)
      const existing = devices.value.find((d) => d.id === device.id)
      if (existing) {
        Object.assign(existing, device)
      } else {
        devices.value.push(device)
      }
      return device
    } catch (err) {
      errorMessage.value = err instanceof Error ? err.message : '添加设备失败'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function removeDevice(id: string) {
    errorMessage.value = null
    try {
      await invokeDisconnectDevice(id)
      devices.value = devices.value.filter((d) => d.id !== id)
      if (selectedDeviceId.value === id) {
        selectedDeviceId.value = null
      }
    } catch (err) {
      errorMessage.value = err instanceof Error ? err.message : '断开设备失败'
      throw err
    }
  }

  async function refreshDeviceInfo(id: string) {
    errorMessage.value = null
    try {
      const info = await invokeGetDeviceInfo(id)
      const existing = devices.value.find((d) => d.id === id)
      if (existing) {
        Object.assign(existing, info)
      }
      return info
    } catch (err) {
      errorMessage.value = err instanceof Error ? err.message : '刷新设备信息失败'
      throw err
    }
  }

  function selectDevice(id: string | null) {
    selectedDeviceId.value = id
  }

  function startPolling(intervalMs: number = 3000) {
    stopPolling()
    pollingTimer = setInterval(() => {
      fetchDevices().catch(() => {})
    }, intervalMs)
  }

  function stopPolling() {
    if (pollingTimer !== null) {
      clearInterval(pollingTimer)
      pollingTimer = null
    }
  }

  return {
    devices,
    selectedDeviceId,
    loading,
    errorMessage,
    onlineDevices,
    offlineDevices,
    selectedDevice,
    deviceCount,
    fetchDevices,
    addDevice,
    removeDevice,
    refreshDeviceInfo,
    selectDevice,
    startPolling,
    stopPolling,
  }
})
