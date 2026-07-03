export type InstallStatus = 'pending' | 'installing' | 'success' | 'failed' | 'cancelled'

export interface InstallTask {
  deviceId: string
  hapPath: string
  status: InstallStatus
  startTime: string | null
  endTime: string | null
  errorMessage: string | null
}
