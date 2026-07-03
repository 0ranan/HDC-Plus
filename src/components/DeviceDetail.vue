<script setup lang="ts">
import { Refresh } from '@element-plus/icons-vue'
import type { Device } from '../types/device'

const props = defineProps<{
  device: Device | null
}>()

const emit = defineEmits<{
  disconnect: [deviceId: string]
  refresh: [deviceId: string]
}>()

function getStatusText(status: string): string {
  return status === 'online' ? '在线' : '离线'
}

function getConnectionText(type: string): string {
  return type === 'USB' ? 'USB 连接' : 'WiFi 连接'
}
</script>

<template>
  <div class="device-detail">
    <template v-if="device">
      <div class="device-detail__header">
        <h3 class="device-detail__name">{{ device.name }}</h3>
        <el-tag :type="device.status === 'online' ? 'success' : 'info'" size="small">
          {{ getStatusText(device.status) }}
        </el-tag>
      </div>

      <el-descriptions :column="1" border size="small">
        <el-descriptions-item label="设备标识">
          {{ device.id }}
        </el-descriptions-item>
        <el-descriptions-item label="型号">
          {{ device.model }}
        </el-descriptions-item>
        <el-descriptions-item label="系统版本">
          {{ device.osVersion }}
        </el-descriptions-item>
        <el-descriptions-item label="序列号">
          {{ device.sn }}
        </el-descriptions-item>
        <el-descriptions-item label="连接方式">
          {{ getConnectionText(device.connectionType) }}
        </el-descriptions-item>
        <el-descriptions-item v-if="device.batteryLevel !== null" label="电池电量">
          {{ device.batteryLevel }}%
        </el-descriptions-item>
      </el-descriptions>

      <div class="device-detail__actions">
        <el-button
          type="primary"
          size="small"
          :icon="Refresh"
          @click="emit('refresh', device.id)"
        >
          刷新信息
        </el-button>
        <el-button
          v-if="device.connectionType === 'WiFi'"
          type="danger"
          size="small"
          @click="emit('disconnect', device.id)"
        >
          断开连接
        </el-button>
      </div>
    </template>

    <template v-else>
      <div class="device-detail__empty">
        <el-empty description="请选择一个设备查看详情" />
      </div>
    </template>
  </div>
</template>

<style scoped>
.device-detail {
  padding: 16px;
  height: 100%;
  overflow-y: auto;
}

.device-detail__header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 16px;
}

.device-detail__name {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.device-detail__actions {
  display: flex;
  gap: 8px;
  margin-top: 16px;
}

.device-detail__empty {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 200px;
}
</style>
