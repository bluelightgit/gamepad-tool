/**
 * 应用程序管理 Composable
 * 管理应用程序生命周期、设置和主要功能
 */
import { reactive, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface AppSettings {
  frameRate: number
  logSize: number
  selectedGamepadId: number
  isRecordLog: boolean
}

export interface AppState {
  isInitializing: boolean
  hasError: boolean
  errorMessage: string
  isMainThreadRunning: boolean
}

/**
 * 应用程序管理 Hook
 */
export function useAppManager() {
  // 应用状态
  const appState = reactive<AppState>({
    isInitializing: true,
    hasError: false,
    errorMessage: '',
    isMainThreadRunning: false
  })
  
  // 应用设置
  const appSettings = reactive<AppSettings>({
    frameRate: 120,
    logSize: 2000,
    selectedGamepadId: 0,
    isRecordLog: true
  })
  
  // 可用的设置选项
  const frameRateOptions = [30, 60, 120, 180]
  const logSizeOptions = [1000, 2000, 4000, 8000]
  
  // Tauri 命令包装器
  const tauriCommands = {
    async startMainThread(userId: number, frameRate: number, isRecordLog: boolean): Promise<void> {
      try {
        console.log(`Starting main thread with userId: ${userId}, frameRate: ${frameRate}`)
        await invoke<void>("start_update", { userId, frameRate, isRecordLog })
        appState.isMainThreadRunning = true
      } catch (error) {
        console.error("Error starting main thread:", error)
        throw error
      }
    },
    
    async stopMainThread(): Promise<void> {
      try {
        await invoke<void>("stop_update")
        appState.isMainThreadRunning = false
      } catch (error) {
        console.error("Error stopping main thread:", error)
        throw error
      }
    },
    
    async setLogSize(logSize: number): Promise<void> {
      try {
        console.log(`Setting log size to ${logSize}`)
        await invoke<void>("set_log_size", { logSize })
        console.log(`Log size set to ${logSize}`)
      } catch (error) {
        console.error("Error setting log size:", error)
        throw error
      }
    },
    
    async getGamepadIds(): Promise<number[]> {
      try {
        return await invoke<number[]>("get_gamepad_ids")
      } catch (error) {
        console.error("Error getting gamepad IDs:", error)
        return [0]
      }
    },
    
    async cleanLog(): Promise<void> {
      try {
        console.log("Cleaning log data...")
        await invoke<void>("clean_log")
        console.log("Log data cleaned successfully")
      } catch (error) {
        console.error("Error cleaning log:", error)
        throw error
      }
    }
  }
  
  // 应用程序初始化
  const initializeApp = async (): Promise<void> => {
    try {
      appState.isInitializing = true
      appState.hasError = false
      appState.errorMessage = ''
      
      console.log("Initializing application...")
      
      // 1. 设置日志大小
      await tauriCommands.setLogSize(appSettings.logSize)
      
      // 2. 启动主线程
      await tauriCommands.startMainThread(
        appSettings.selectedGamepadId, 
        appSettings.frameRate,
        appSettings.isRecordLog
      )
      
      console.log("Application initialized successfully")
      appState.isInitializing = false
    } catch (error) {
      console.error("Application initialization failed:", error)
      appState.hasError = true
      appState.errorMessage = error instanceof Error ? error.message : "Unknown error"
      appState.isInitializing = false
      throw error
    }
  }
  
  // 重启主线程（当设置改变时）
  const restartMainThread = async (): Promise<void> => {
    try {
      if (appState.isMainThreadRunning) {
        await tauriCommands.stopMainThread()
      }
      
      // 确保应用当前设置
      await tauriCommands.setLogSize(appSettings.logSize)
      await tauriCommands.startMainThread(
        appSettings.selectedGamepadId,
        appSettings.frameRate,
        appSettings.isRecordLog
      )
    } catch (error) {
      console.error("Error restarting main thread:", error)
      appState.hasError = true
      appState.errorMessage = error instanceof Error ? error.message : "Restart failed"
      throw error
    }
  }
  
  // 重试初始化
  const retryInitialization = async (): Promise<void> => {
    return initializeApp()
  }
  
  // 更新帧率
  const updateFrameRate = async (newFrameRate: number): Promise<void> => {
    if (frameRateOptions.includes(newFrameRate) && newFrameRate !== appSettings.frameRate) {
      appSettings.frameRate = newFrameRate
      await restartMainThread()
    }
  }
  
  // 更新日志大小
  const updateLogSize = async (newLogSize: number): Promise<void> => {
    if (logSizeOptions.includes(newLogSize) && newLogSize !== appSettings.logSize) {
      appSettings.logSize = newLogSize
      await restartMainThread()
    }
  }
  
  // 更新选中的手柄ID
  const updateSelectedGamepadId = async (newId: number): Promise<void> => {
    if (newId !== appSettings.selectedGamepadId) {
      appSettings.selectedGamepadId = newId
      await restartMainThread()
    }
  }

  // 是否记录log数据
  const updateIsRecordLog = async (isRecordLog: boolean): Promise<void> => {
    if (isRecordLog !== appSettings.isRecordLog) {
      appSettings.isRecordLog = isRecordLog
      await restartMainThread()
    }
  }
  
  // 监听设置变化，自动重启主线程
  watch(
    () => appSettings.selectedGamepadId,
    async (newId, oldId) => {
      if (newId !== oldId && appState.isMainThreadRunning) {
        try {
          await restartMainThread()
        } catch (error) {
          console.error("Failed to restart main thread after gamepad change:", error)
        }
      }
    }
  )
  
  // 应用关闭时的清理
  const cleanup = async (): Promise<void> => {
    try {
      if (appState.isMainThreadRunning) {
        await tauriCommands.stopMainThread()
      }
    } catch (error) {
      console.error("Error during cleanup:", error)
    }
  }
  
  return {
    // 状态
    appState,
    appSettings,
    
    // 选项
    frameRateOptions,
    logSizeOptions,
    
    // 方法
    initializeApp,
    retryInitialization,
    updateFrameRate,
    updateLogSize,
    updateSelectedGamepadId,
    updateIsRecordLog,
    cleanup,
    
    // Tauri 命令
    tauriCommands
  }
}
