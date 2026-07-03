<script setup lang="ts">
import { ref, reactive, watch } from 'vue'
import type { FormInstance, FormRules } from 'element-plus'
import { ElMessage } from 'element-plus'
import { useDeviceStore } from '../stores/device'
import type { Device } from '../types/device'

const props = defineProps<{
  visible: boolean
}>()

const emit = defineEmits<{
  close: []
  connected: [device: Device]
}>()

const deviceStore = useDeviceStore()
const formRef = ref<FormInstance>()
const connecting = ref(false)

const formData = reactive({
  ip: '',
  port: 5555,
})

const rules: FormRules = {
  ip: [
    { required: true, message: '请输入设备 IP 地址', trigger: 'blur' },
    {
      pattern: /^(\d{1,3}\.){3}\d{1,3}$/,
      message: 'IP 地址格式不正确',
      trigger: 'blur',
    },
  ],
  port: [
    { required: true, message: '请输入端口号', trigger: 'blur' },
    {
      type: 'number',
      min: 1,
      max: 65535,
      message: '端口号范围为 1-65535',
      trigger: 'blur',
    },
  ],
}

watch(
  () => props.visible,
  (val) => {
    if (!val) {
      formData.ip = ''
      formData.port = 5555
      connecting.value = false
    }
  }
)

async function handleConnect() {
  if (!formRef.value) return
  await formRef.value.validate(async (valid) => {
    if (!valid) return
    connecting.value = true
    try {
      const device = await deviceStore.addDevice(formData.ip, formData.port)
      emit('connected', device)
      ElMessage.success(`成功连接到设备: ${device.name}`)
      emit('close')
    } catch (err) {
      ElMessage.error(deviceStore.errorMessage || '连接失败')
    } finally {
      connecting.value = false
    }
  })
}

function handleClose() {
  emit('close')
}
</script>

<template>
  <el-dialog
    title="添加设备"
    :model-value="visible"
    width="420px"
    :close-on-click-modal="false"
    @close="handleClose"
  >
    <el-form
      ref="formRef"
      :model="formData"
      :rules="rules"
      label-width="80px"
      label-position="left"
    >
      <el-form-item label="IP 地址" prop="ip">
        <el-input
          v-model="formData.ip"
          placeholder="例如: 192.168.1.100"
          :disabled="connecting"
        />
      </el-form-item>
      <el-form-item label="端口号" prop="port">
        <el-input-number
          v-model="formData.port"
          :min="1"
          :max="65535"
          :disabled="connecting"
          style="width: 100%"
        />
      </el-form-item>
    </el-form>

    <template #footer>
      <el-button @click="handleClose" :disabled="connecting">取消</el-button>
      <el-button
        type="primary"
        :loading="connecting"
        @click="handleConnect"
      >
        {{ connecting ? '连接中...' : '连接设备' }}
      </el-button>
    </template>
  </el-dialog>
</template>
