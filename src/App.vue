<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { ElMessage, ElNotification } from 'element-plus'
import { useDeviceStore } from './stores/device'
import DeviceList from './components/DeviceList.vue'
import ConnectDialog from './components/ConnectDialog.vue'
import type { Device } from './types/device'

const deviceStore = useDeviceStore()
const showConnectDialog = ref(false)

onMounted(async () => {
  try {
    await deviceStore.fetchDevices()
  } catch {
    ElMessage.warning('设备列表加载失败，请检查 HDC 工具是否可用')
  }
  deviceStore.startPolling(3000)
})

onUnmounted(() => {
  deviceStore.stopPolling()
})

watch(
  () => deviceStore.errorMessage,
  (msg) => {
    if (msg) {
      ElMessage.error(msg)
    }
  }
)

watch(
  () => deviceStore.offlineDevices.length,
  (newCount, oldCount) => {
    if (newCount > oldCount) {
      const newlyOffline = deviceStore.offlineDevices[deviceStore.offlineDevices.length - 1]
      if (newlyOffline) {
        ElNotification({
          title: '设备已断开',
          message: `${newlyOffline.name} (${newlyOffline.id}) 连接已断开`,
          type: 'warning',
          duration: 5000,
        })
      }
    }
  }
)

function handleAddDevice() {
  showConnectDialog.value = true
}

function handleDeviceConnected(device: Device) {
  showConnectDialog.value = false
  deviceStore.selectDevice(device.id)
}

async function handleDisconnectDevice(deviceId: string) {
  try {
    await deviceStore.removeDevice(deviceId)
    ElMessage.success('设备已断开连接')
  } catch {
    ElMessage.error(deviceStore.errorMessage || '断开连接失败')
  }
}

async function handleRefreshDevice(deviceId: string) {
  try {
    await deviceStore.refreshDeviceInfo(deviceId)
    ElMessage.success('设备信息已刷新')
  } catch {
    ElMessage.error(deviceStore.errorMessage || '刷新信息失败')
  }
}
</script>

<template>
  <div class="app-container">
    <DeviceList
      @add-device="handleAddDevice"
      @disconnect-device="handleDisconnectDevice"
      @refresh-device="handleRefreshDevice"
    />
    <ConnectDialog
      :visible="showConnectDialog"
      @close="showConnectDialog = false"
      @connected="handleDeviceConnected"
    />
  </div>
</template>

<style scoped>
.app-container {
  width: 100%;
  height: 100vh;
  overflow: hidden;
}
</style>
