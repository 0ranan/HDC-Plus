<script setup lang="ts">
import { useDeviceStore } from '../stores/device'
import DeviceCard from './DeviceCard.vue'
import DeviceDetail from './DeviceDetail.vue'

const props = defineProps<{
  showAddButton?: boolean
}>()

const emit = defineEmits<{
  addDevice: []
  disconnectDevice: [deviceId: string]
  refreshDevice: [deviceId: string]
}>()

const deviceStore = useDeviceStore()

function handleSelect(deviceId: string) {
  deviceStore.selectDevice(deviceId)
}

function handleDisconnect(deviceId: string) {
  emit('disconnectDevice', deviceId)
}

function handleRefresh(deviceId: string) {
  emit('refreshDevice', deviceId)
}
</script>

<template>
  <div class="device-list-container">
    <div class="device-list__sidebar">
      <div class="device-list__toolbar">
        <h2 class="device-list__title">
          设备列表
          <span class="device-list__count">({{ deviceStore.deviceCount }})</span>
        </h2>
        <el-button
          v-if="showAddButton !== false"
          type="primary"
          size="small"
          @click="emit('addDevice')"
        >
          添加设备
        </el-button>
      </div>

      <div class="device-list__cards" v-loading="!deviceStore.devices.length && deviceStore.loading">
        <template v-if="deviceStore.devices.length > 0">
          <DeviceCard
            v-for="device in deviceStore.devices"
            :key="device.id"
            :device="device"
            :is-selected="deviceStore.selectedDeviceId === device.id"
            @select="handleSelect"
          />
        </template>

        <template v-else>
          <div class="device-list__empty">
            <el-empty
              description="暂无已连接设备，请通过 USB 连接设备或点击「添加设备」按钮手动连接"
            />
          </div>
        </template>
      </div>
    </div>

    <div class="device-list__detail">
      <DeviceDetail
        :device="deviceStore.selectedDevice"
        @disconnect="handleDisconnect"
        @refresh="handleRefresh"
      />
    </div>
  </div>
</template>

<style scoped>
.device-list-container {
  display: flex;
  height: 100%;
  overflow: hidden;
}

.device-list__sidebar {
  width: 340px;
  min-width: 280px;
  border-right: 1px solid var(--el-border-color-light);
  display: flex;
  flex-direction: column;
  background-color: var(--el-bg-color-page);
}

.device-list__toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px;
  border-bottom: 1px solid var(--el-border-color-light);
}

.device-list__title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.device-list__count {
  font-weight: 400;
  color: var(--el-text-color-secondary);
  font-size: 14px;
}

.device-list__cards {
  flex: 1;
  overflow-y: auto;
  padding: 12px 16px;
}

.device-list__empty {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 300px;
}

.device-list__detail {
  flex: 1;
  overflow: hidden;
}
</style>
