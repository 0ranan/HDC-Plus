<script setup lang="ts">
import { ref } from 'vue'
import type { Device } from '../types/device'

const props = defineProps<{
  device: Device
  isSelected: boolean
  isDragOver?: boolean
}>()

const emit = defineEmits<{
  select: [deviceId: string]
}>()

const cardDragOver = ref(false)
const statusText = props.device.status === 'online' ? '在线' : '离线'
const connectionText = props.device.connectionType === 'USB' ? 'USB' : 'WiFi'

function handleDragOver(e: DragEvent) {
  if (props.device.status !== 'online') return
  e.preventDefault()
  e.stopPropagation()
  cardDragOver.value = true
}

function handleDragLeave() {
  cardDragOver.value = false
}

function handleDrop(e: DragEvent) {
  e.preventDefault()
  e.stopPropagation()
  cardDragOver.value = false
}
</script>

<template>
  <div
    class="device-card"
    :class="{
      'device-card--selected': isSelected,
      'device-card--online': device.status === 'online',
      'device-card--offline': device.status === 'offline',
      'device-card--drag-over': cardDragOver || isDragOver,
    }"
    :data-device-id="device.id"
    @click="emit('select', device.id)"
    @dragover="handleDragOver"
    @dragleave="handleDragLeave"
    @drop="handleDrop"
  >
    <div class="device-card__header">
      <span class="device-card__status-dot" :class="`device-card__status-dot--${device.status}`"></span>
      <span class="device-card__name">{{ device.name }}</span>
    </div>
    <div class="device-card__info">
      <span class="device-card__model">{{ device.model }}</span>
      <el-tag size="small" :type="device.connectionType === 'USB' ? '' : 'success'">
        {{ connectionText }}
      </el-tag>
    </div>
    <div class="device-card__footer">
      <span class="device-card__status-text" :class="`device-card__status-text--${device.status}`">
        {{ statusText }}
      </span>
    </div>
    <div v-if="cardDragOver && device.status === 'online'" class="device-card__drop-hint">
      释放以安装到此设备
    </div>
  </div>
</template>

<style scoped>
.device-card {
  padding: 12px 16px;
  border: 1px solid var(--el-border-color-light);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s ease;
  background-color: var(--el-bg-color);
  margin-bottom: 8px;
}

.device-card:hover {
  border-color: var(--el-color-primary-light-3);
}

.device-card--selected {
  border-color: var(--el-color-primary);
  box-shadow: 0 0 0 2px var(--el-color-primary-light-8);
}

.device-card--drag-over {
  border-color: var(--el-color-primary);
  box-shadow: 0 0 0 3px var(--el-color-primary-light-5);
  transform: scale(1.02);
}

.device-card--online {
  border-left: 3px solid #67c23a;
}

.device-card--offline {
  opacity: 0.6;
  border-left: 3px solid #c0c4cc;
}

.device-card__header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}

.device-card__status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.device-card__status-dot--online {
  background-color: #67c23a;
}

.device-card__status-dot--offline {
  background-color: #c0c4cc;
}

.device-card__name {
  font-weight: 600;
  font-size: 14px;
  color: var(--el-text-color-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.device-card__info {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 6px;
}

.device-card__model {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.device-card__footer {
  display: flex;
  align-items: center;
}

.device-card__status-text {
  font-size: 12px;
}

.device-card__status-text--online {
  color: #67c23a;
}

.device-card__status-text--offline {
  color: #c0c4cc;
}

.device-card__drop-hint {
  margin-top: 8px;
  padding: 4px 8px;
  background-color: var(--el-color-primary-light-9);
  border-radius: 4px;
  font-size: 12px;
  color: var(--el-color-primary);
  text-align: center;
}
</style>
