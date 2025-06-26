<template>
  <div class="gamepad-test-page">
    <!-- 设置面板 - 固定在左上角 -->
    <div class="settings-container">
      <SettingsPanel
        :current-frame-rate="appSettings.frameRate"
        :current-log-size="appSettings.logSize"
        :inner-deadzone="settings.innerDeadzone"
        :outer-deadzone="settings.outerDeadzone"
        :frame-rate-options="frameRateOptions"
        :log-size-options="logSizeOptions"
        @update:frame-rate="handleFrameRateUpdate"
        @update:log-size="handleLogSizeUpdate"
        @update:inner-deadzone="handleInnerDeadzoneUpdate"
        @update:outer-deadzone="handleOuterDeadzoneUpdate"
      />
    </div>

    <!-- 页面头部：包含标签页 -->
    <header class="page-header">
      <GamepadTabs
        :selected-id="selectedGamepadId"
        :available-ids="gamepadIds"
        :is-gamepad-available="isGamepadAvailable"
        @select="handleGamepadSelect"
      />
    </header>

    <!-- 状态信息显示 -->
    <div v-if="appState.isInitializing" class="status-message loading">
      <p>🚀 Initializing application, please wait...</p>
    </div>
    <div v-if="appState.hasError" class="status-message error">
      <p>🔥 Initialization Error: {{ appState.errorMessage }}</p>
      <button @click="retryInitialization" class="retry-button">Retry</button>
    </div>

    <!-- 主内容区域 -->
    <div v-if="!appState.isInitializing && !appState.hasError" class="main-layout">
      <!-- 手柄信息 -->
      <div class="gamepad-info-card">
        <h2>{{ currentGamepad.name }}</h2>
        <p>Power: {{ currentGamepad.power_info }}</p>
      </div>

      <!-- 布局容器 -->
      <div class="layout-grid">
        <!-- 摇杆区域 -->
        <div class="grid-item joysticks-area">
          <div class="joysticks-header">
            <h3>Joysticks</h3>
            <div class="joystick-controls">
              <button
                class="toggle-button"
                :class="{ active: settings.showHistory }"
                @click="toggleHistoryDisplay"
              >
                {{ settings.showHistory ? 'Hide' : 'Show' }} Trail
              </button>
              <button
                v-if="settings.showHistory"
                class="clear-button"
                @click="handleClearHistory"
              >
                Clear
              </button>
            </div>
          </div>
          <div class="joystick-group">
            <OptimizedJoystick
              :axis-x="getAxisValue('LeftThumbX')"
              :axis-y="getAxisValue('LeftThumbY')"
              :history-points="leftJoystickHistory"
              :show-history="settings.showHistory"
              :inner-deadzone="settings.innerDeadzone"
              :outer-deadzone="settings.outerDeadzone"
            />
            <OptimizedJoystick
              :axis-x="getAxisValue('RightThumbX')"
              :axis-y="getAxisValue('RightThumbY')"
              :history-points="rightJoystickHistory"
              :show-history="settings.showHistory"
              :inner-deadzone="settings.innerDeadzone"
              :outer-deadzone="settings.outerDeadzone"
            />
          </div>
        </div>

        <!-- 控制区域容器 -->
        <div class="controls-container">
          <!-- 按键区域 -->
          <div class="grid-item buttons-area">
            <GamepadButtons :gamepad="currentGamepad" />
          </div>

          <!-- 性能数据显示区域 -->
          <div class="grid-item performance-area">
            <PollingRateDisplay :polling-rate-data="selectedPollingRateData" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onBeforeUnmount } from 'vue'

// 引入重构后的模块
import { useAppManager } from '../composables/useAppManager'
import { useGamepadState } from '../composables/useGamepadState'
import { useEventListeners } from '../composables/useEventListeners'
import { useRenderManager } from '../composables/useRenderManager'

// 引入重构后的组件
import GamepadTabs from '../components/GamepadTabs.vue'
import SettingsPanel from '../components/SettingsPanel.vue'
import GamepadButtons from '../components/GamepadButtons.vue'
import OptimizedJoystick from '../components/OptimizedJoystick.vue'
import PollingRateDisplay from '../components/PollingRateDisplay.vue'

// 1. 初始化应用管理器
const {
  appState,
  appSettings,
  frameRateOptions,
  logSizeOptions,
  initializeApp,
  retryInitialization,
  updateFrameRate,
  updateLogSize,
  updateSelectedGamepadId,
  cleanup: cleanupAppManager
} = useAppManager()

// 2. 初始化手柄状态管理器
const {
  selectedGamepadId,
  gamepadIds,
  currentGamepad,
  settings,
  leftJoystickHistory,
  rightJoystickHistory,
  selectedPollingRateData,
  isGamepadAvailable,
  getAxisValue,
  updateGamepadData,
  updatePollingRateData,
  updateGamepadIds,
  selectGamepad,
  toggleHistoryDisplay,
  clearAllHistory,
  updateInnerDeadzone,
  updateOuterDeadzone
} = useGamepadState()

// 3. 初始化事件监听器
const { registerCallback, cleanup: cleanupEventListeners } = useEventListeners()

// 4. 初始化渲染管理器
const { cleanup: cleanupRenderManager } = useRenderManager()

// --- 事件回调注册 ---
registerCallback('gamepads_info', updateGamepadData)
registerCallback('polling_rate_result', updatePollingRateData)

// --- 业务逻辑和处理函数 ---
const handleGamepadSelect = (id: number) => {
  selectGamepad(id)
  updateSelectedGamepadId(id)
}

const handleFrameRateUpdate = (rate: number) => {
  updateFrameRate(rate)
}

const handleLogSizeUpdate = (size: number) => {
  updateLogSize(size)
}

const handleClearHistory = () => {
  clearAllHistory()
}

const handleInnerDeadzoneUpdate = (value: number) => {
  updateInnerDeadzone(value)
}

const handleOuterDeadzoneUpdate = (value: number) => {
  updateOuterDeadzone(value)
}

// --- 生命周期钩子 ---
onMounted(async () => {
  try {
    // 启动应用
    await initializeApp()
    
    // 获取初始手柄列表
    await updateGamepadIds()
    
    // 设置定时器定期刷新手柄列表
    const gamepadUpdateInterval = setInterval(updateGamepadIds, 2000)
    onBeforeUnmount(() => clearInterval(gamepadUpdateInterval))
    
    // 注册一个渲染回调来更新UI（如果需要的话）
    // 示例：registerRenderCallback(() => { /* a high-frequency update */ })
    
  } catch (error) {
    console.error("Failed to mount GamepadTestPage:", error)
  }
})

onBeforeUnmount(() => {
  // 清理资源
  cleanupAppManager()
  cleanupEventListeners()
  cleanupRenderManager()
})
</script>

<style scoped>
.gamepad-test-page {
  position: relative;
  display: flex;
  flex-direction: column;
  gap: 24px;
  padding: 16px;
  padding-top: 60px; /* 为固定设置按钮留出空间 */
  background-color: var(--container-bg-color);
  border-radius: 12px;
  box-shadow: 0 4px 20px var(--shadow-color);
}

.settings-container {
  position: fixed;
  top: 16px;
  left: 16px;
  z-index: 1000;
}

.page-header {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-wrap: wrap;
  gap: 16px;
}

.status-message {
  padding: 16px;
  border-radius: 8px;
  text-align: center;
  font-weight: 500;
}

.status-message.loading {
  background-color: #e3f2fd;
  color: #0d47a1;
  border: 1px solid #90caf9;
}

.status-message.error {
  background-color: #ffebee;
  color: #b71c1c;
  border: 1px solid #ef9a9a;
}

.retry-button {
  margin-top: 12px;
  padding: 8px 24px;
  background-color: #f44336;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: background-color 0.2s ease;
}

.retry-button:hover {
  background-color: #d32f2f;
}

.main-layout {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.gamepad-info-card {
  padding: 16px;
  background: #f8f9fa;
  border-radius: 8px;
  text-align: center;
  border: 1px solid var(--border-color);
}

.gamepad-info-card h2 {
  margin: 0 0 8px 0;
  color: var(--primary-color);
}

.gamepad-info-card p {
  margin: 0;
  color: #666;
}

.layout-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 20px;
  align-items: start;
}

.controls-container {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.grid-item {
  padding: 16px;
  background: #f8f9fa;
  border-radius: 8px;
  border: 1px solid var(--border-color);
  min-width: 0;
  overflow: hidden;
  box-sizing: border-box;
}

.joysticks-area h3 {
  margin: 0 0 16px 0;
  color: #333;
  font-size: 1.2em;
  font-weight: 600;
}

.joysticks-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  flex-wrap: wrap;
  gap: 8px;
}

.joysticks-header h3 {
  margin: 0;
}

.joystick-controls {
  display: flex;
  gap: 8px;
  align-items: center;
}

.toggle-button {
  padding: 6px 12px;
  background: #42b983;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.85em;
  transition: all 0.2s ease;
}

.toggle-button:hover {
  background: #369870;
}

.toggle-button.active {
  background: #369870;
  box-shadow: 0 2px 4px rgba(66, 185, 131, 0.3);
}

.clear-button {
  padding: 6px 10px;
  background: #ff4757;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.8em;
  transition: all 0.2s ease;
}

.clear-button:hover {
  background: #ff3742;
}

/* 大屏幕：三列显示 */
@media (min-width: 1200px) {
  .layout-grid {
    grid-template-columns: 1fr 1fr 1fr;
    gap: 24px;
  }
  
  .controls-container {
    display: contents; /* 让子元素直接参与网格布局 */
  }
}

/* 中等大屏幕：两列，但性能数据仍在右侧 */
@media (min-width: 900px) and (max-width: 1199px) {
  .layout-grid {
    grid-template-columns: 1fr 1fr;
    gap: 20px;
  }
  
  .controls-container {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }
}

.joystick-group {
  display: flex;
  justify-content: space-around;
  align-items: center;
  flex-wrap: wrap;
  gap: 20px;
}

/* 中等屏幕：摇杆在上方，按键和性能并排在下方 */
@media (min-width: 769px) and (max-width: 899px) {
  .layout-grid {
    grid-template-columns: 1fr;
    gap: 20px;
  }
  
  .controls-container {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
  }
}

/* 响应式设计 */
@media (max-width: 768px) {
  .gamepad-test-page {
    padding: 12px;
    padding-top: 60px;
    gap: 16px;
  }
  
  .settings-container {
    top: 12px;
    left: 12px;
  }
  
  .page-header {
    flex-direction: column;
    align-items: stretch;
  }
  
  .layout-grid {
    grid-template-columns: 1fr;
    gap: 16px;
  }
  
  .controls-container {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
  }
  
  .grid-item {
    padding: 12px;
  }
  
  .joysticks-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 10px;
  }
  
  .joystick-controls {
    gap: 6px;
  }
  
  .toggle-button,
  .clear-button {
    font-size: 0.8em;
    padding: 5px 10px;
  }
  
  .joystick-group {
    gap: 16px;
    flex-direction: row;
    justify-content: space-evenly;
  }
}

@media (max-width: 600px) {
  .gamepad-test-page {
    padding: 10px;
    padding-top: 60px;
    gap: 14px;
  }
  
  .layout-grid {
    gap: 14px;
  }
  
  .controls-container {
    grid-template-columns: 1fr;
    gap: 10px;
  }
  
  .grid-item {
    padding: 10px;
  }
  
  .joysticks-header {
    gap: 8px;
  }
  
  .toggle-button,
  .clear-button {
    font-size: 0.75em;
    padding: 4px 8px;
  }
  
  .joystick-group {
    flex-direction: column;
    gap: 12px;
    align-items: center;
  }
}

@media (max-width: 480px) {
  .gamepad-test-page {
    padding: 8px;
    padding-top: 60px;
    gap: 12px;
  }
  
  .settings-container {
    top: 8px;
    left: 8px;
  }
  
  .layout-grid {
    gap: 12px;
  }
  
  .controls-container {
    gap: 8px;
  }
  
  .grid-item {
    padding: 8px;
  }
  
  .toggle-button,
  .clear-button {
    font-size: 0.7em;
    padding: 3px 6px;
  }
  
  .gamepad-info-card {
    padding: 12px;
  }
  
  .gamepad-info-card h2 {
    font-size: 1.3em;
  }
  
  .joystick-group {
    flex-direction: column;
    gap: 10px;
  }
}

@media (max-width: 360px) {
  .gamepad-test-page {
    padding: 6px;
    padding-top: 60px;
  }
  
  .settings-container {
    top: 6px;
    left: 6px;
  }
  
  .controls-container {
    gap: 6px;
  }
  
  .grid-item {
    padding: 6px;
  }
  
  .toggle-button,
  .clear-button {
    font-size: 0.65em;
    padding: 2px 4px;
  }
  
  .gamepad-info-card {
    padding: 8px;
  }
  
  .gamepad-info-card h2 {
    font-size: 1.2em;
  }
  
  .joystick-group {
    gap: 8px;
  }
}

@media (max-width: 320px) {
  .gamepad-test-page {
    padding: 4px;
    padding-top: 60px;
  }
  
  .controls-container {
    gap: 4px;
  }
  
  .grid-item {
    padding: 4px;
  }
  
  .gamepad-info-card {
    padding: 6px;
  }
  
  .gamepad-info-card h2 {
    font-size: 1.1em;
  }
}
</style>
