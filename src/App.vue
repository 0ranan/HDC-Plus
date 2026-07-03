<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { ElMessage, ElNotification } from 'element-plus'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useDeviceStore } from './stores/device'
import { useHapStore } from './stores/hap'
import DeviceList from './components/DeviceList.vue'
import ConnectDialog from './components/ConnectDialog.vue'
import HapInstallPanel from './components/HapInstallPanel.vue'
import type { Device } from './types/device'

const deviceStore = useDeviceStore()
const hapStore = useHapStore()
const showConnectDialog = ref(false)
const activeTab = ref('devices')
const isDragOver = ref(false)
let unlistenDragDrop: (() => void) | null = null

onMounted(async () => {
  try {
    await deviceStore.fetchDevices()
  } catch {
    ElMessage.warning('设备列表加载失败，请检查 HDC 工具是否可用')
  }
  deviceStore.startPolling(3000)

  unlistenDragDrop = await getCurrentWindow().onDragDropEvent(async (event) => {
    if (event.payload.type === 'over') {
      const hasHap = event.payload.paths.some((p) => p.toLowerCase().endsWith('.hap'))
      if (hasHap) {
        isDragOver.value = true
      }
    } else if (event.payload.type === 'leave') {
      isDragOver.value = false
    } else if (event.payload.type === 'drop') {
      isDragOver.value = false
      const hapPaths = event.payload.paths.filter((p) => p.toLowerCase().endsWith('.hap'))
      if (hapPaths.length === 0) return

      const hapPath = hapPaths[0]
      const dpr = window.devicePixelRatio
      const x = event.payload.position.x / dpr
      const y = event.payload.position.y / dpr
      const element = document.elementFromPoint(x, y)

      if (element) {
        const card = element.closest('[data-device-id]') as HTMLElement | null
        if (card) {
          const deviceId = card.dataset.deviceId
          if (deviceId) {
            await handleHapDropOnDevice(deviceId, hapPath)
            return
          }
        }
      }

      const valid = await hapStore.validateAndSetHap(hapPath)
      if (valid) {
        activeTab.value = 'hap'
      } else {
        ElMessage.error(hapStore.errorMessage || 'HAP 文件校验失败')
      }
    }
  })
})

onUnmounted(() => {
  deviceStore.stopPolling()
  if (unlistenDragDrop) {
    unlistenDragDrop()
  }
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

async function handleHapDropOnDevice(deviceId: string, hapPath: string) {
  const valid = await hapStore.validateAndSetHap(hapPath)
  if (valid) {
    hapStore.selectTargetDevice(deviceId)
    activeTab.value = 'hap'
  } else {
    ElMessage.error(hapStore.errorMessage || 'HAP 文件校验失败')
  }
}
</script>

<template>
  <div class="app-container">
    <div class="app-tabs">
      <el-tabs v-model="activeTab" class="app-tabs__bar">
        <el-tab-pane label="设备管理" name="devices">
          <DeviceList
            :is-drag-over="isDragOver"
            @add-device="handleAddDevice"
            @disconnect-device="handleDisconnectDevice"
            @refresh-device="handleRefreshDevice"
          />
        </el-tab-pane>
        <el-tab-pane label="HAP 安装" name="hap">
          <HapInstallPanel />
        </el-tab-pane>
      </el-tabs>
    </div>

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

.app-tabs {
  height: 100%;
}

.app-tabs :deep(.el-tabs) {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.app-tabs :deep(.el-tabs__header) {
  margin: 0;
  padding: 0 16px;
  border-bottom: 1px solid var(--el-border-color-light);
  flex-shrink: 0;
}

.app-tabs :deep(.el-tabs__content) {
  flex: 1;
  overflow: hidden;
}

.app-tabs :deep(.el-tab-pane) {
  height: 100%;
}
</style>
