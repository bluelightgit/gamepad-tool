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
    name: "Loading...",
    guid: "",
    power_info: "Unknown",
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
  // 基础状态
  const selectedGamepadId = ref<number>(0)
  const gamepadIds = ref<number[]>([0])
  
  // 使用 shallowRef 优化大对象性能
  const currentGamepad = shallowRef<GamepadInfo>(createDefaultGamepad(0))
  const pollingRateData = reactive<Record<string, PollingRateResult>>({
    '0': createDefaultPollingRateResult()
  })
  
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
  const selectedPollingRateData = computed(() => 
    pollingRateData[selectedGamepadId.value.toString()] || createDefaultPollingRateResult()
  )
  
  const isGamepadAvailable = (id: number): boolean => {
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
  
  // 数据更新函数 - 使用对象引用比较避免不必要的更新
  const updateGamepadData = (newData: GamepadInfo) => {
    if (!newData || newData.id !== selectedGamepadId.value) return
    
    // 浅比较检查是否真的有变化
    const current = currentGamepad.value
    if (current && 
        current.name === newData.name &&
        current.power_info === newData.power_info) {
      
      // 检查buttons和axes是否有实质变化
      let hasButtonChange = false
      let hasAxisChange = false
      
      for (const key in newData.buttons) {
        if (current.buttons[key]?.value !== newData.buttons[key]?.value) {
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
  
  // 轮询率数据更新
  const updatePollingRateData = (data: PollingRateResult) => {
    const userId = selectedGamepadId.value.toString()
    pollingRateData[userId] = data
  }
  
  // 手柄ID更新
  const updateGamepadIds = async (): Promise<number[]> => {
    try {
      const ids = await invoke<number[]>("get_gamepad_ids")
      gamepadIds.value = ids.length > 0 ? ids : [0]
      
      // 如果当前选中的ID不在列表中，选择第一个
      if (!gamepadIds.value.includes(selectedGamepadId.value)) {
        selectedGamepadId.value = gamepadIds.value[0]
      }
      
      return gamepadIds.value
    } catch (error) {
      console.error("Error updating gamepad IDs:", error)
      gamepadIds.value = [0]
      return [0]
    }
  }
  
  // 选择手柄
  const selectGamepad = (id: number) => {
    if (selectedGamepadId.value !== id && isGamepadAvailable(id)) {
      selectedGamepadId.value = id
      // 清空历史数据
      leftJoystickHistory.value = []
      rightJoystickHistory.value = []
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
