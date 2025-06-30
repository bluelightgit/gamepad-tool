<template>
  <div class="gamepad-test-page">
    <!-- 页面头部：包含标签页和设置面板 -->
    <header class="page-header">
      <GamepadTabs
        :selected-id="selectedGamepadId"
        :available-ids="gamepadIds"
        :is-gamepad-available="isGamepadAvailable"
        :current-frame-rate="appSettings.frameRate"
        :current-log-size="appSettings.logSize"
        :inner-deadzone="settings.innerDeadzone"
        :outer-deadzone="settings.outerDeadzone"
        :frame-rate-options="frameRateOptions"
        :log-size-options="logSizeOptions"
        @select="handleGamepadSelect"
        @update:frame-rate="handleFrameRateUpdate"
        @update:log-size="handleLogSizeUpdate"
        @update:inner-deadzone="handleInnerDeadzoneUpdate"
        @update:outer-deadzone="handleOuterDeadzoneUpdate"
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
    // Example: registerRenderCallback(() => { /* high-frequency update */ })
    
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
  gap: 8px;
  padding: 4px 8px;
  padding-top: 32px;
  max-height: 1000px;
  overflow-y: auto;
  background-color: var(--container-bg-color);
  border-radius: 6px;
  box-shadow: 0 1px 8px var(--shadow-color);
}

.page-header {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 4px;
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
  gap: 8px;
  flex: 1;
  min-height: 0;
}

.gamepad-info-card {
  padding: 6px 10px;
  background: #f8f9fa;
  border-radius: 4px;
  text-align: center;
  border: 1px solid var(--border-color);
  flex-shrink: 0;
}

.gamepad-info-card h2 {
  margin: 0 0 2px 0;
  color: var(--primary-color);
  font-size: 1.0em;
}

.gamepad-info-card p {
  margin: 0;
  color: #666;
  font-size: 0.85em;
}

.layout-grid {
  display: grid;
  gap: 12px;
  align-items: stretch;
  flex: 1;
  min-height: 0;
  /* 默认移动端布局：单列 */
  grid-template-columns: 1fr;
  grid-template-rows: auto auto auto;
}

.controls-container {
  display: flex;
  flex-direction: column;
  gap: 12px;
  height: 100%;
  min-height: 0;
}

.grid-item {
  padding: 12px;
  background: #f8f9fa;
  border-radius: 6px;
  border: 1px solid var(--border-color);
  min-width: 0;
  overflow: hidden;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

/* 基础joystick区域设置 */
.joysticks-area {
  min-height: 260px;
  max-height: 380px;
  overflow: visible;
}

.joystick-group {
  display: flex;
  justify-content: center;
  align-items: flex-start;
  flex-wrap: wrap;
  gap: 16px;
  flex: 1;
  align-content: flex-start;
  padding-top: 8px;
}

/* 基础buttons和performance区域设置 */
.buttons-area,
.performance-area {
  min-height: 200px;
  max-height: 350px;
  overflow-y: auto;
  flex: 1;
  min-width: 0; /* 允许收缩 */
  max-width: 100%; /* 限制最大宽度 */
  overflow: hidden; /* 防止内容溢出 */
}

.joysticks-area h3 {
  margin: 0 0 6px 0; /* 进一步减少间距 */
  color: #333;
  font-size: 1.0em; /* 进一步减小字体 */
  font-weight: 600;
}

.joysticks-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 6px;
  flex-wrap: nowrap; /* 防止换行 */
  gap: 8px; /* 适当增加间距 */
  flex-shrink: 0;
  min-height: 24px; /* 固定最小高度确保一致性 */
}

.joysticks-header h3 {
  margin: 0;
}

.joysticks-area {
  min-height: 280px; /* 增加基础高度 */
  max-height: 400px; /* 增加最大高度 */
  overflow: visible; /* 确保内容不被隐藏 */
}

.joystick-group {
  display: flex;
  justify-content: space-around;
  align-items: flex-start; /* 改为顶部对齐 */
  flex-wrap: wrap;
  gap: 12px;
  flex: 1;
  align-content: flex-start; /* 内容顶部对齐 */
  padding-top: 4px; /* 少量顶部内边距 */
}

.buttons-section,
.performance-section {
  flex: 1; /* 平分控制容器高度 */
  display: flex;
  flex-direction: column;
  min-height: 0; /* 允许收缩 */
}

/* 确保buttons和performance区域内容能够填充空间 */
.buttons-section > *,
.performance-section > * {
  flex: 1;
}

.joystick-controls {
  display: flex;
  gap: 4px; /* 减少间距以适应紧凑布局 */
  align-items: center;
  flex-shrink: 0; /* 防止按钮被压缩 */
}

.toggle-button {
  padding: 3px 6px; /* 更紧凑的按钮 */
  background: #42b983;
  color: white;
  border: none;
  border-radius: 3px;
  cursor: pointer;
  font-size: 0.75em; /* 减小字体以适应紧凑布局 */
  transition: all 0.2s ease;
  white-space: nowrap; /* 防止文字换行 */
}

.toggle-button:hover {
  background: #369870;
}

.toggle-button.active {
  background: #369870;
  box-shadow: 0 1px 2px rgba(66, 185, 131, 0.3);
}

.clear-button {
  padding: 3px 5px; /* 更紧凑的按钮 */
  background: #ff4757;
  color: white;
  border: none;
  border-radius: 3px;
  cursor: pointer;
  font-size: 0.7em; /* 减小字体 */
  transition: all 0.2s ease;
  white-space: nowrap; /* 防止文字换行 */
}

.clear-button:hover {
  background: #ff3742;
}

/* 宽屏和超宽屏：三栏布局，Performance Stats较窄 (1000px+) */
@media (min-width: 1000px) {
  .layout-grid {
    grid-template-columns: 1.4fr 1fr 0.6fr; /* performance更窄，joystick更宽 */
    gap: 20px;
  }
  
  .controls-container {
    display: contents;
  }
  
  .joysticks-area {
    min-height: 320px;
    max-height: 400px;
  }
  
  .joystick-group {
    flex-direction: row; /* 横向排列joystick */
    justify-content: space-evenly;
    gap: 24px;
    flex-wrap: nowrap;
  }
  
  .buttons-area {
    min-height: 320px;
    max-height: 400px;
  }
  
  .performance-area {
    min-height: 320px;
    max-height: 400px;
    /* Performance区域较窄，内容会自动调整 */
  }
}

/* 中屏：joystick占一行，buttons+performance并排，紧凑显示 (768-999px) */
@media (min-width: 768px) and (max-width: 999px) {
  .layout-grid {
    grid-template-columns: 1fr;
    grid-template-rows: auto auto;
    gap: 14px;
  }
  
  .controls-container {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 14px;
  }
  
  .joysticks-area {
    min-height: 240px;
    max-height: 300px;
  }
  
  .joysticks-area h3 {
    font-size: 0.9em; /* 缩小标题字体 */
  }
  
  .joystick-group {
    flex-direction: row;
    justify-content: space-around;
    gap: 18px;
  }
  
  .buttons-area,
  .performance-area {
    min-height: 220px;
    max-height: 280px;
    overflow: visible; /* 防止内容被裁剪 */
  }
  
  /* 缩小区域内的标题字体 */
  .buttons-area h3,
  .performance-area h3 {
    font-size: 0.9em;
    margin: 0 0 8px 0;
  }
}

/* 小屏：全部纵向堆叠，紧凑按钮显示 (480-767px) */
@media (min-width: 480px) and (max-width: 767px) {
  .layout-grid {
    grid-template-columns: 1fr;
    gap: 10px;
  }
  
  .controls-container {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  
  .joysticks-area {
    min-height: 220px;
    max-height: 260px;
  }
  
  .joysticks-area h3 {
    font-size: 0.85em; /* 进一步缩小标题 */
  }
  
  .joystick-group {
    flex-direction: row;
    justify-content: space-around;
    gap: 14px;
  }
  
  .buttons-area,
  .performance-area {
    min-height: 140px; /* 减小高度避免溢出 */
    max-height: 170px; /* 减小最大高度 */
    padding: 6px; /* 减少内边距 */
    overflow: visible; /* 确保内容不被裁剪 */
  }
  
  /* 缩小所有标题和按钮 */
  .buttons-area h3,
  .performance-area h3 {
    font-size: 0.85em;
    margin: 0 0 6px 0;
  }
  
  /* 针对按钮区域的特殊优化 */
  .buttons-area {
    overflow: visible; /* 确保按钮不被裁剪 */
  }
}

</style>
