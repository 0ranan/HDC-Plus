export type ConnectionType = 'USB' | 'WiFi'

export type DeviceStatus = 'online' | 'offline'

export interface Device {
  id: string
  name: string
  model: string
  osVersion: string
  sn: string
  connectionType: ConnectionType
  status: DeviceStatus
  batteryLevel: number | null
}
