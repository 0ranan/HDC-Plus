<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { ElMessage } from 'element-plus'
import { UploadFilled } from '@element-plus/icons-vue'
import { open } from '@tauri-apps/plugin-dialog'
import { useDeviceStore } from '../stores/device'
import { useHapStore } from '../stores/hap'

const deviceStore = useDeviceStore()
const hapStore = useHapStore()
const installing = ref(false)
const dragOver = ref(false)

const onlineDevices = computed(() => deviceStore.onlineDevices)
const selectedFileName = computed(() => {
  if (!hapStore.selectedHapPath) return null
  return hapStore.selectedHapPath.split(/[\\/]/).pop() || hapStore.selectedHapPath
})

async function handleSelectFile() {
  try {
    const selected = await open({
      multiple: false,
      filters: [{ name: 'HAP 安装包', extensions: ['hap'] }],
    })
    if (selected) {
      const path = typeof selected === 'string' ? selected : (Array.isArray(selected) ? selected[0] : null)
      if (path) {
        const valid = await hapStore.validateAndSetHap(path)
        if (!valid) {
          ElMessage.error(hapStore.errorMessage || '文件校验失败')
        }
      }
    }
  } catch (err) {
    ElMessage.error('选择文件失败: ' + (err instanceof Error ? err.message : String(err)))
  }
}

function handleRemoveFile() {
  hapStore.clearCurrentTask()
  hapStore.selectHap(null)
}

function handleDragOver(e: DragEvent) {
  e.preventDefault()
  dragOver.value = true
}

function handleDragLeave() {
  dragOver.value = false
}

function handleDrop(e: DragEvent) {
  e.preventDefault()
  dragOver.value = false
}

async function handleInstall() {
  if (!hapStore.selectedHapPath) {
    ElMessage.warning('请先选择 HAP 安装包')
    return
  }
  if (!hapStore.targetDeviceId) {
    const onlineCount = onlineDevices.value.length
    if (onlineCount === 0) {
      ElMessage.warning('没有可用的在线设备')
      return
    } else if (onlineCount === 1) {
      hapStore.selectTargetDevice(onlineDevices.value[0].id)
    } else {
      ElMessage.warning('请先选择目标设备')
      return
    }
  }

  installing.value = true
  try {
    const task = await hapStore.installHap()
    if (task.status === 'success') {
      ElMessage.success('HAP 安装成功')
    } else {
      ElMessage.error(task.errorMessage || 'HAP 安装失败')
    }
  } catch (err) {
    ElMessage.error(hapStore.errorMessage || 'HAP 安装失败')
  } finally {
    installing.value = false
  }
}

function getStatusText(status: string): string {
  switch (status) {
    case 'pending': return '等待安装'
    case 'installing': return '安装中...'
    case 'success': return '安装成功'
    case 'failed': return '安装失败'
    case 'cancelled': return '已取消'
    default: return status
  }
}

function getStatusType(status: string): string {
  switch (status) {
    case 'success': return 'success'
    case 'failed': return 'danger'
    case 'installing': return 'warning'
    default: return 'info'
  }
}
</script>

<template>
  <div class="hap-install-panel">
    <div class="hap-install__header">
      <h3 class="hap-install__title">HAP 安装</h3>
    </div>

    <div class="hap-install__content">
      <div
        class="hap-install__dropzone"
        :class="{ 'hap-install__dropzone--active': dragOver }"
        @dragover="handleDragOver"
        @dragleave="handleDragLeave"
        @drop="handleDrop"
      >
        <template v-if="!hapStore.selectedHapPath">
          <el-icon :size="48" color="#c0c4cc">
            <UploadFilled />
          </el-icon>
          <p class="hap-install__dropzone-text">
            拖拽 HAP 文件到此处，或点击下方按钮选择文件
          </p>
        </template>
        <template v-else>
          <div class="hap-install__file-info">
            <span class="hap-install__file-name">{{ selectedFileName }}</span>
            <el-button
              type="danger"
              size="small"
              text
              @click="handleRemoveFile"
            >
              移除
            </el-button>
          </div>
        </template>
      </div>

      <div class="hap-install__actions">
        <el-button type="primary" @click="handleSelectFile">
          选择 HAP 文件
        </el-button>
      </div>

      <el-divider />

      <div class="hap-install__device-select">
        <label class="hap-install__label">目标设备</label>
        <el-select
          v-model="hapStore.targetDeviceId"
          placeholder="请选择目标设备"
          style="width: 100%"
          :disabled="installing"
          clearable
        >
          <el-option
            v-for="device in onlineDevices"
            :key="device.id"
            :label="`${device.name} (${device.id})`"
            :value="device.id"
          />
        </el-select>
        <span v-if="onlineDevices.length === 0" class="hap-install__hint">
          暂无在线设备，请先连接设备
        </span>
      </div>

      <el-divider />

      <div class="hap-install__footer">
        <el-button
          type="primary"
          size="large"
          :loading="installing"
          :disabled="!hapStore.selectedHapPath || onlineDevices.length === 0"
          @click="handleInstall"
          style="width: 100%"
        >
          {{ installing ? '安装中...' : '安装 HAP' }}
        </el-button>
      </div>

      <div v-if="hapStore.currentTask" class="hap-install__result">
        <el-divider />
        <el-alert
          :title="getStatusText(hapStore.currentTask.status)"
          :type="getStatusType(hapStore.currentTask.status) as any"
          :closable="false"
          show-icon
        >
          <template v-if="hapStore.currentTask.errorMessage">
            <p>{{ hapStore.currentTask.errorMessage }}</p>
          </template>
        </el-alert>
      </div>
    </div>
  </div>
</template>

<style scoped>
.hap-install-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  background-color: var(--el-bg-color);
}

.hap-install__header {
  padding: 16px 20px;
  border-bottom: 1px solid var(--el-border-color-light);
}

.hap-install__title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.hap-install__content {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
}

.hap-install__dropzone {
  border: 2px dashed var(--el-border-color-light);
  border-radius: 8px;
  padding: 32px 20px;
  text-align: center;
  transition: all 0.3s ease;
  cursor: pointer;
  background-color: var(--el-bg-color-page);
}

.hap-install__dropzone--active {
  border-color: var(--el-color-primary);
  background-color: var(--el-color-primary-light-9);
}

.hap-install__dropzone-text {
  margin-top: 12px;
  font-size: 13px;
  color: var(--el-text-color-secondary);
}

.hap-install__file-info {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 0;
}

.hap-install__file-name {
  font-size: 14px;
  color: var(--el-text-color-primary);
  font-weight: 500;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.hap-install__actions {
  margin-top: 12px;
  text-align: center;
}

.hap-install__device-select {
  margin: 4px 0;
}

.hap-install__label {
  display: block;
  margin-bottom: 8px;
  font-size: 14px;
  font-weight: 500;
  color: var(--el-text-color-primary);
}

.hap-install__hint {
  display: block;
  margin-top: 6px;
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.hap-install__footer {
  margin-top: 4px;
}

.hap-install__result {
  margin-top: 8px;
}
</style>
