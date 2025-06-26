/**
 * 事件监听器管理 Composable
 * 统一管理所有Tauri事件监听器，优化性能
 */
import { onMounted, onBeforeUnmount } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import type { GamepadInfo, PollingRateResult } from './useGamepadState'

export interface OutputLog {
  timestamp: number
  xyxy: [number, number, number, number]
}

export interface JoystickLevelsData {
  leftLevelCount: number
  rightLevelCount: number
  leftValues: number[]
  rightValues: number[]
}

/**
 * 事件监听器管理 Hook
 */
export function useEventListeners() {
  const unlistenFunctions: UnlistenFn[] = []
  
  // 事件回调函数映射
  const eventCallbacks = new Map<string, Function>()
  
  // 注册事件回调
  const registerCallback = (eventName: string, callback: Function) => {
    eventCallbacks.set(eventName, callback)
  }
  
  // 节流函数工厂
  const createThrottle = (fn: Function, delay: number) => {
    let lastCall = 0
    return (...args: any[]) => {
      const now = performance.now()
      if (now - lastCall >= delay) {
        lastCall = now
        fn(...args)
      }
    }
  }
  
  // 初始化所有事件监听器
  const initializeEventListeners = async () => {
    try {
      // 1. 手柄信息事件 - 高频更新，需要节流
      const gamepadInfoThrottled = createThrottle((data: GamepadInfo) => {
        const callback = eventCallbacks.get('gamepads_info')
        if (callback) callback(data)
      }, 8) // ~120fps
      
      const unlistenGamepadInfo = await listen("gamepads_info", (event) => {
        if (event.payload) {
          gamepadInfoThrottled(event.payload as GamepadInfo)
        }
      })
      unlistenFunctions.push(unlistenGamepadInfo)
      
      // 2. 轮询率结果事件 - 中频更新
      const pollingRateThrottled = createThrottle((data: PollingRateResult) => {
        const callback = eventCallbacks.get('polling_rate_result')
        if (callback) callback(data)
      }, 16) // ~60fps
      
      const unlistenPollingRate = await listen("polling_rate_result", (event) => {
        if (event.payload) {
          pollingRateThrottled(event.payload as PollingRateResult)
        }
      })
      unlistenFunctions.push(unlistenPollingRate)
      
      // 3. 轮询率日志事件 - 用于历史轨迹，低频更新
      const pollingRateLogThrottled = createThrottle((data: OutputLog[]) => {
        const callback = eventCallbacks.get('polling_rate_log')
        if (callback) callback(data)
      }, 33) // ~30fps
      
      const unlistenPollingRateLog = await listen("polling_rate_log", (event) => {
        if (event.payload) {
          pollingRateLogThrottled(event.payload as OutputLog[])
        }
      })
      unlistenFunctions.push(unlistenPollingRateLog)
      
      // 4. 摇杆分级数据事件 - 低频更新
      const joystickLevelsThrottled = createThrottle((data: any) => {
        const callback = eventCallbacks.get('joystick_levels')
        if (callback) callback(data)
      }, 100) // ~10fps
      
      const unlistenJoystickLevels = await listen("joystick_levels", (event) => {
        if (event.payload) {
          joystickLevelsThrottled(event.payload)
        }
      })
      unlistenFunctions.push(unlistenJoystickLevels)
      
      console.log("All event listeners initialized successfully")
    } catch (error) {
      console.error("Failed to initialize event listeners:", error)
      throw error
    }
  }
  
  // 清理所有事件监听器
  const cleanupEventListeners = () => {
    unlistenFunctions.forEach(unlisten => {
      try {
        unlisten()
      } catch (error) {
        console.error("Error cleaning up event listener:", error)
      }
    })
    unlistenFunctions.length = 0
    eventCallbacks.clear()
  }
  
  // 自动管理生命周期
  onMounted(initializeEventListeners)
  onBeforeUnmount(cleanupEventListeners)

  return {
    registerCallback,
    cleanup: cleanupEventListeners
  }
}
