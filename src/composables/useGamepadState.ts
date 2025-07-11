/**
 * 游戏手柄状态管理 Composable
 * 使用Vue 3 Composition API 重构，提升性能
 */
import { ref, computed, reactive, shallowRef, triggerRef, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface AxisData {
  axis: string
  value: number
}

export interface ButtonData {
  button: string
  is_pressed: boolean
  value: number
}

export interface GamepadInfo {
  id: number
  name: string
  vendor_id?: number
  product_id?: number
  guid: string
  power_info: string
  axes: Record<string, AxisData>
  buttons: Record<string, ButtonData>
}

export interface PollingRateResult {
  polling_rate_avg: number
  polling_rate_min: number
  polling_rate_max: number
  avg_interval: number
  drop_rate: number
  avg_error_r: number
  avg_error_l: number
}

export interface HistoryPoint {
  x: number
  y: number
}

// 默认手柄数据工厂函数
function createDefaultGamepad(id: number = 0): GamepadInfo {
  return {
    id,
    name: id === -1 ? "No Gamepad Connected" : "Loading...",
    guid: "",
    power_info: id === -1 ? "N/A" : "Unknown",
    axes: {
      "LeftThumbX": { axis: "LeftThumbX", value: 0 },
      "LeftThumbY": { axis: "LeftThumbY", value: 0 },
      "RightThumbX": { axis: "RightThumbX", value: 0 },
      "RightThumbY": { axis: "RightThumbY", value: 0 }
    },
    buttons: {
      "A": { button: "A", is_pressed: false, value: 0 },
      "B": { button: "B", is_pressed: false, value: 0 },
      "X": { button: "X", is_pressed: false, value: 0 },
      "Y": { button: "Y", is_pressed: false, value: 0 },
      "LeftShoulder": { button: "LeftShoulder", is_pressed: false, value: 0 },
      "RightShoulder": { button: "RightShoulder", is_pressed: false, value: 0 },
      "LeftTrigger": { button: "LeftTrigger", is_pressed: false, value: 0 },
      "RightTrigger": { button: "RightTrigger", is_pressed: false, value: 0 },
      "Back": { button: "Back", is_pressed: false, value: 0 },
      "Start": { button: "Start", is_pressed: false, value: 0 },
      "LeftThumb": { button: "LeftThumb", is_pressed: false, value: 0 },
      "RightThumb": { button: "RightThumb", is_pressed: false, value: 0 },
      "DPadUp": { button: "DPadUp", is_pressed: false, value: 0 },
      "DPadDown": { button: "DPadDown", is_pressed: false, value: 0 },
      "DPadLeft": { button: "DPadLeft", is_pressed: false, value: 0 },
      "DPadRight": { button: "DPadRight", is_pressed: false, value: 0 }
    }
  }
}

function createDefaultPollingRateResult(): PollingRateResult {
  return {
    polling_rate_avg: 0,
    polling_rate_min: 0,
    polling_rate_max: 0,
    avg_interval: 0,
    drop_rate: 0,
    avg_error_l: 0,
    avg_error_r: 0
  }
}

/**
 * 手柄状态管理 Hook
 */
export function useGamepadState() {
  // 基础状态 - 使用-1表示没有选中的手柄
  const selectedGamepadId = ref<number>(-1)
  const gamepadIds = ref<number[]>([])
  
  // 使用 shallowRef 优化大对象性能
  const currentGamepad = shallowRef<GamepadInfo>(createDefaultGamepad(-1))
  const pollingRateData = reactive<Record<string, PollingRateResult>>({})
  
  // 初始化标记，确保只在第一次设置正确的默认值
  const isInitialized = ref(false)
  
  // 应用状态
  const appState = reactive({
    isInitializing: true,
    hasError: false,
    errorMessage: '',
    isConnected: false
  })
  
  // 设置状态
  const settings = reactive({
    frameRate: 120,
    logSize: 2000,
    showHistory: false,
    innerDeadzone: 0.05,
    outerDeadzone: 1.0
  })
  
  // 历史数据 - 使用环形缓冲区优化内存
  const maxHistoryPoints = computed(() => settings.logSize)
  const leftJoystickHistory = ref<HistoryPoint[]>([])
  const rightJoystickHistory = ref<HistoryPoint[]>([])
  
  // 清理历史数据当设置改变时
  const clearHistoryIfNeeded = () => {
    const maxPoints = maxHistoryPoints.value
    if (leftJoystickHistory.value.length > maxPoints) {
      leftJoystickHistory.value = leftJoystickHistory.value.slice(-maxPoints)
    }
    if (rightJoystickHistory.value.length > maxPoints) {
      rightJoystickHistory.value = rightJoystickHistory.value.slice(-maxPoints)
    }
  }
  
  // 清除所有历史记录
  const clearAllHistory = () => {
    leftJoystickHistory.value = []
    rightJoystickHistory.value = []
  }
  
  // 监听 logSize 变化并清理历史记录
  watch(() => settings.logSize, () => {
    clearHistoryIfNeeded()
  })
  
  // 计算属性
  const selectedPollingRateData = computed(() => {
    // 如果没有选中的手柄（-1），返回默认数据
    if (selectedGamepadId.value === -1) {
      return createDefaultPollingRateResult()
    }
    return pollingRateData[selectedGamepadId.value.toString()] || createDefaultPollingRateResult()
  })

  const isGamepadAvailable = (id: number): boolean => {
    // 简化逻辑：只要手柄ID在可用列表中，就认为是可用的
    // 这适用于所有手柄，包括0号手柄（如果它真实存在的话）
    return gamepadIds.value.includes(id)
  }
  
  // 手柄数据获取器 - 优化的访问函数
  const getButtonValue = (buttonKey: string): number => {
    return currentGamepad.value?.buttons[buttonKey]?.value || 0
  }
  
  const getButtonName = (buttonKey: string): string => {
    return currentGamepad.value?.buttons[buttonKey]?.button || buttonKey
  }
  
  const getAxisValue = (axisKey: string): number => {
    return currentGamepad.value?.axes[axisKey]?.value || 0
  }
  
  const formatButtonValue = (buttonKey: string): string => {
    return getButtonValue(buttonKey).toFixed(2)
  }
  
  const formatNumber = (value: number): string => {
    return value.toFixed(2)
  }
  
  // 高效的历史数据更新函数
  let lastHistoryUpdate = 0
  const HISTORY_THROTTLE = 16 // ~60fps for history to capture more detail
  
  const updateJoystickHistory = () => {
    if (!settings.showHistory) return
    
    const now = performance.now()
    if (now - lastHistoryUpdate < HISTORY_THROTTLE) return
    
    lastHistoryUpdate = now
    
    const leftX = getAxisValue('LeftThumbX')
    const leftY = getAxisValue('LeftThumbY')
    const rightX = getAxisValue('RightThumbX')
    const rightY = getAxisValue('RightThumbY')
    
    // 总是添加当前位置（即使是零点），这样可以显示完整的轨迹
    leftJoystickHistory.value.push({ x: leftX, y: leftY })
    rightJoystickHistory.value.push({ x: rightX, y: rightY })
    
    // 保持历史记录在指定大小内
    const maxPoints = maxHistoryPoints.value
    if (leftJoystickHistory.value.length > maxPoints) {
      leftJoystickHistory.value.shift()
    }
    if (rightJoystickHistory.value.length > maxPoints) {
      rightJoystickHistory.value.shift()
    }
  }
  
  // 数据更新函数 - 使用对象引用比较避免不必要的更新，并验证手柄ID
  const updateGamepadData = (newData: GamepadInfo) => {
    if (!newData) return
    
    // 确保只更新当前选中手柄的数据
    if (newData.id !== selectedGamepadId.value) {
      return // 忽略其他手柄的数据
    }
    
    // 浅比较检查是否真的有变化
    const current = currentGamepad.value
    if (current && 
        current.id === newData.id &&
        current.name === newData.name &&
        current.power_info === newData.power_info) {
      
      // 检查buttons和axes是否有实质变化
      let hasButtonChange = false
      let hasAxisChange = false
      
      for (const key in newData.buttons) {
        if (current.buttons[key]?.value !== newData.buttons[key]?.value ||
            current.buttons[key]?.is_pressed !== newData.buttons[key]?.is_pressed) {
          hasButtonChange = true
          break
        }
      }
      
      if (!hasButtonChange) {
        for (const key in newData.axes) {
          if (current.axes[key]?.value !== newData.axes[key]?.value) {
            hasAxisChange = true
            break
          }
        }
      }
      
      if (!hasButtonChange && !hasAxisChange) return
    }
    
    // 直接替换引用，触发响应式更新
    currentGamepad.value = newData
    triggerRef(currentGamepad)
    
    // 更新历史数据
    updateJoystickHistory()
  }
  
  // 轮询率数据更新 - 修复闪烁问题，确保只更新对应手柄的数据
  const updatePollingRateData = (data: PollingRateResult & { gamepad_id?: number }) => {
    // 如果没有选中的手柄，不更新数据
    if (selectedGamepadId.value === -1) {
      return
    }
    
    // 如果数据包含手柄ID，只有当它匹配当前选中的手柄时才更新
    if (data.gamepad_id !== undefined && data.gamepad_id !== selectedGamepadId.value) {
      return // 忽略其他手柄的数据
    }
    
    const userId = selectedGamepadId.value.toString()
    pollingRateData[userId] = {
      polling_rate_avg: data.polling_rate_avg,
      polling_rate_min: data.polling_rate_min,
      polling_rate_max: data.polling_rate_max,
      avg_interval: data.avg_interval,
      drop_rate: data.drop_rate,
      avg_error_l: data.avg_error_l,
      avg_error_r: data.avg_error_r
    }
  }
  
  // 手柄ID更新 - 修复断开连接问题，确保选中的手柄始终可用，特别处理初始化
  const updateGamepadIds = async (): Promise<number[]> => {
    try {
      const ids = await invoke<number[]>("get_gamepad_ids")
      const previousIds = [...gamepadIds.value]
      const currentSelectedId = selectedGamepadId.value
      const wasInitialized = isInitialized.value
      
      // 更新手柄列表
      gamepadIds.value = ids.length > 0 ? ids : []
      
      // 如果没有可用的手柄，保持空状态
      if (gamepadIds.value.length === 0) {
        console.log('No gamepads available, keeping empty state')
        
        // 重置到空状态，但不添加虚拟的0号手柄
        if (selectedGamepadId.value !== -1) {
          selectedGamepadId.value = -1 // 使用-1表示没有选中的手柄
          currentGamepad.value = createDefaultGamepad(-1)
          triggerRef(currentGamepad)
          leftJoystickHistory.value = []
          rightJoystickHistory.value = []
          // 清空所有轮询率数据
          Object.keys(pollingRateData).forEach(key => delete pollingRateData[key])
        }
        
        isInitialized.value = true
        return []
      }
      
      // 初始化时，选择第一个可用的手柄而不是默认的0
      if (!wasInitialized) {
        const firstAvailableId = gamepadIds.value[0]
        console.log(`Initial gamepad setup: selecting gamepad ${firstAvailableId} from available: [${gamepadIds.value.join(', ')}]`)
        
        selectedGamepadId.value = firstAvailableId
        currentGamepad.value = createDefaultGamepad(firstAvailableId)
        triggerRef(currentGamepad)
        leftJoystickHistory.value = []
        rightJoystickHistory.value = []
        
        const userId = firstAvailableId.toString()
        pollingRateData[userId] = createDefaultPollingRateResult()
        
        isInitialized.value = true
        console.log(`Initial setup complete: gamepad ${firstAvailableId} selected`)
        return gamepadIds.value
      }
      
      // 检查当前选中的手柄是否仍然可用
      const isCurrentAvailable = gamepadIds.value.includes(currentSelectedId)
      
      if (!isCurrentAvailable) {
        // 当前选中的手柄不可用，切换到第一个可用的手柄
        const newSelectedId = gamepadIds.value[0]
        console.log(`Gamepad ${currentSelectedId} not available, switching to gamepad ${newSelectedId}`)
        console.log(`Available gamepads: [${gamepadIds.value.join(', ')}]`)
        
        selectedGamepadId.value = newSelectedId
        
        // 重置当前手柄数据为默认数据，避免显示已断开手柄的数据
        currentGamepad.value = createDefaultGamepad(newSelectedId)
        triggerRef(currentGamepad)
        
        // 清空历史数据
        leftJoystickHistory.value = []
        rightJoystickHistory.value = []
        
        // 重置轮询率数据
        const userId = newSelectedId.toString()
        if (!pollingRateData[userId]) {
          pollingRateData[userId] = createDefaultPollingRateResult()
        }
        
        console.log(`Successfully switched to gamepad ${newSelectedId}`)
      } else {
        console.log(`Current gamepad ${currentSelectedId} is still available`)
      }
      
      // 如果手柄列表发生变化，记录日志
      const idsChanged = previousIds.length !== gamepadIds.value.length || 
                         !previousIds.every(id => gamepadIds.value.includes(id))
      
      if (idsChanged) {
        console.log('Gamepad list changed:', {
          previous: previousIds,
          current: gamepadIds.value,
          selectedId: selectedGamepadId.value
        })
      }
      
      return gamepadIds.value
    } catch (error) {
      console.error("Error updating gamepad IDs:", error)
      
      // 发生错误时回退到空状态
      gamepadIds.value = []
      if (selectedGamepadId.value !== -1) {
        selectedGamepadId.value = -1
        currentGamepad.value = createDefaultGamepad(-1)
        triggerRef(currentGamepad)
        leftJoystickHistory.value = []
        rightJoystickHistory.value = []
        // 清空所有轮询率数据
        Object.keys(pollingRateData).forEach(key => delete pollingRateData[key])
      }
      isInitialized.value = true
      
      return []
    }
  }
  
  // 选择手柄
  const selectGamepad = (id: number) => {
    if (selectedGamepadId.value !== id && isGamepadAvailable(id)) {
      const previousId = selectedGamepadId.value
      selectedGamepadId.value = id
      
      // 重置当前手柄数据为新手柄的默认数据
      currentGamepad.value = createDefaultGamepad(id)
      triggerRef(currentGamepad)
      
      // 清空历史数据
      leftJoystickHistory.value = []
      rightJoystickHistory.value = []
      
      // 确保新手柄有默认的轮询率数据
      const userId = id.toString()
      if (!pollingRateData[userId]) {
        pollingRateData[userId] = createDefaultPollingRateResult()
      }
      
      console.log(`Switched from gamepad ${previousId} to ${id}`)
    }
  }
  
  // 切换历史显示
  const toggleHistoryDisplay = () => {
    settings.showHistory = !settings.showHistory
    if (!settings.showHistory) {
      leftJoystickHistory.value = []
      rightJoystickHistory.value = []
    }
  }
  
  // 清空历史数据
  const clearHistory = () => {
    leftJoystickHistory.value = []
    rightJoystickHistory.value = []
  }
  
  // 更新内部deadzone
  const updateInnerDeadzone = (value: number) => {
    settings.innerDeadzone = Math.max(0, Math.min(value, settings.outerDeadzone - 0.01))
  }
  
  // 更新外部deadzone
  const updateOuterDeadzone = (value: number) => {
    settings.outerDeadzone = Math.max(settings.innerDeadzone + 0.01, value)
  }
  
  return {
    // 状态
    selectedGamepadId,
    gamepadIds,
    currentGamepad,
    pollingRateData,
    appState,
    settings,
    isInitialized,
    
    // 历史数据
    leftJoystickHistory,
    rightJoystickHistory,
    maxHistoryPoints,
    
    // 计算属性
    selectedPollingRateData,
    
    // 方法
    isGamepadAvailable,
    getButtonValue,
    getButtonName,
    getAxisValue,
    formatButtonValue,
    formatNumber,
    updateGamepadData,
    updatePollingRateData,
    updateGamepadIds,
    selectGamepad,
    toggleHistoryDisplay,
    clearHistory,
    clearHistoryIfNeeded,
    clearAllHistory,
    updateInnerDeadzone,
    updateOuterDeadzone
  }
}
