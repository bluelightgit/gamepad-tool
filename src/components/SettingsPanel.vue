<template>
  <div class="settings-panel">
    <button 
      class="settings-button" 
      @click="toggleSettings"
      :class="{ active: isOpen }"
    >
      <svg class="settings-icon" width="16" height="16" viewBox="0 0 512 512" xmlns="http://www.w3.org/2000/svg">
        <path d="M262.29,192.31a64,64,0,1,0,57.4,57.4A64.13,64.13,0,0,0,262.29,192.31ZM416.39,256a154.34,154.34,0,0,1-1.53,20.79l45.21,35.46A10.81,10.81,0,0,1,462.52,326l-42.77,74a10.81,10.81,0,0,1-13.14,4.59l-44.9-18.08a16.11,16.11,0,0,0-15.17,1.75A164.48,164.48,0,0,1,325,400.8a15.94,15.94,0,0,0-8.82,12.14l-6.73,47.89A11.08,11.08,0,0,1,298.77,470H213.23a11.11,11.11,0,0,1-10.69-8.87l-6.72-47.82a16.07,16.07,0,0,0-9-12.22,155.3,155.3,0,0,1-21.46-12.57,16,16,0,0,0-15.11-1.71l-44.89,18.07a10.81,10.81,0,0,1-13.14-4.58l-42.77-74a10.8,10.8,0,0,1,2.45-13.75l38.21-30a16.05,16.05,0,0,0,6-14.08c-.36-4.17-.58-8.33-.58-12.5s.21-8.27.58-12.35a16,16,0,0,0-6.07-13.94l-38.19-30A10.81,10.81,0,0,1,49.48,186l42.77-74a10.81,10.81,0,0,1,13.14-4.59l44.9,18.08a16.11,16.11,0,0,0,15.17-1.75A164.48,164.48,0,0,1,187,111.2a15.94,15.94,0,0,0,8.82-12.14l6.73-47.89A11.08,11.08,0,0,1,213.23,42h85.54a11.11,11.11,0,0,1,10.69,8.87l6.72,47.82a16.07,16.07,0,0,0,9,12.22,155.3,155.3,0,0,1,21.46,12.57,16,16,0,0,0,15.11,1.71l44.89-18.07a10.81,10.81,0,0,1,13.14,4.58l42.77,74a10.8,10.8,0,0,1-2.45,13.75l-38.21,30a16.05,16.05,0,0,0-6.05,14.08C416.17,247.67,416.39,251.83,416.39,256Z" style="fill:none;stroke:currentColor;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/>
      </svg>
      Settings
    </button>
    
    <Transition name="settings-menu">
      <div v-if="isOpen" class="settings-menu">
        <div class="menu-header">
          <h4>Settings</h4>
          <button class="close-button" @click="closeSettings">✕</button>
        </div>
        
        <div class="settings-content">
          <!-- 帧率设置 -->
          <div class="setting-group">
            <label class="setting-label">Frame Rate</label>
            <div class="setting-options">
              <button
                v-for="rate in frameRateOptions"
                :key="rate"
                class="option-button"
                :class="{ selected: currentFrameRate === rate }"
                @click="updateFrameRate(rate)"
              >
                {{ rate }} Hz
              </button>
            </div>
          </div>
          
          <!-- 日志大小设置 -->
          <div class="setting-group">
            <label class="setting-label">Log Size</label>
            <div class="setting-options">
              <button
                v-for="size in logSizeOptions"
                :key="size"
                class="option-button"
                :class="{ selected: currentLogSize === size }"
                @click="updateLogSize(size)"
              >
                {{ size }}
              </button>
            </div>
          </div>
          
          <!-- Deadzone设置 -->
          <div class="setting-group">
            <label class="setting-label">Joystick Deadzone</label>
            <div class="deadzone-controls">
              <div class="deadzone-item">
                <label class="deadzone-label">Inner:</label>
                <input
                  type="range"
                  min="0"
                  max="0.5"
                  step="0.01"
                  :value="innerDeadzone"
                  @input="updateInnerDeadzone(($event.target as HTMLInputElement)?.value)"
                  class="deadzone-slider"
                />
                <span class="deadzone-value">{{ innerDeadzone.toFixed(2) }}</span>
              </div>
              <div class="deadzone-item">
                <label class="deadzone-label">Outer:</label>
                <input
                  type="range"
                  min="0.5"
                  max="2"
                  step="0.01"
                  :value="outerDeadzone"
                  @input="updateOuterDeadzone(($event.target as HTMLInputElement)?.value)"
                  class="deadzone-slider"
                />
                <span class="deadzone-value">{{ outerDeadzone.toFixed(2) }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Transition>
    
    <!-- 背景遮罩 -->
    <Transition name="backdrop">
      <div v-if="isOpen" class="backdrop" @click="closeSettings"></div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface Props {
  currentFrameRate: number
  currentLogSize: number
  innerDeadzone: number
  outerDeadzone: number
  frameRateOptions: number[]
  logSizeOptions: number[]
}

interface Emits {
  (e: 'update:frameRate', value: number): void
  (e: 'update:logSize', value: number): void
  (e: 'update:innerDeadzone', value: number): void
  (e: 'update:outerDeadzone', value: number): void
}

defineProps<Props>()
const emit = defineEmits<Emits>()

const isOpen = ref(false)

const toggleSettings = () => {
  isOpen.value = !isOpen.value
}

const closeSettings = () => {
  isOpen.value = false
}

const updateFrameRate = (rate: number) => {
  emit('update:frameRate', rate)
  closeSettings()
}

const updateLogSize = (size: number) => {
  emit('update:logSize', size)
  closeSettings()
}

const updateInnerDeadzone = (value: string) => {
  const numValue = parseFloat(value)
  if (!isNaN(numValue)) {
    emit('update:innerDeadzone', numValue)
  }
}

const updateOuterDeadzone = (value: string) => {
  const numValue = parseFloat(value)
  if (!isNaN(numValue)) {
    emit('update:outerDeadzone', numValue)
  }
}
</script>

<style scoped>
.settings-panel {
  position: relative;
}

.settings-button {
  padding: 6px 10px; /* 与gamepad-tab保持一致 */
  background: #e9e9e9; /* 与gamepad-tab保持一致 */
  border: 2px #4f4f4f15 solid;
  border-radius: 3px; /* 与gamepad-tab保持一致 */
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 12px; /* 与gamepad-tab保持一致 */
  font-weight: 600;
  color: #4f4f4f;
  white-space: nowrap;
  min-width: 90px; /* 与gamepad-tab保持一致 */
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
}

.settings-button:hover {
  background: #ddd; /* 与gamepad-tab保持一致 */
  color: #333;
  transform: translateY(-1px);
}

.settings-button.active {
  background: linear-gradient(135deg, #42b983 0%, #369870 100%); /* 与gamepad-tab.active保持一致 */
  color: white;
  box-shadow: 0 2px 8px rgba(66, 185, 131, 0.3);
}

/* 响应式设计 - 与GamepadTabs保持一致 */

/* 宽屏和超宽屏合并 (1000px+) */
@media (min-width: 1000px) {
  .settings-button {
    padding: 8px 14px;
    min-width: 110px;
    font-size: 13px;
    gap: 6px;
  }
  
  .settings-icon {
    width: 18px;
    height: 18px;
  }
}

/* 中屏：紧凑显示 (768-999px) */
@media (min-width: 768px) and (max-width: 999px) {
  .settings-button {
    padding: 6px 10px;
    min-width: 85px;
    font-size: 11px;
    gap: 4px;
  }
  
  .settings-icon {
    width: 16px;
    height: 16px;
  }
}

/* 小屏：更紧凑显示 (480-767px) */
@media (min-width: 480px) and (max-width: 767px) {
  .settings-button {
    padding: 5px 8px;
    min-width: 75px;
    font-size: 10px;
    gap: 3px;
  }
  
  .settings-icon {
    width: 14px;
    height: 14px;
  }
}

.settings-menu {
  position: absolute;
  top: calc(100% + 8px);
  right: 0; /* 从右边对齐，防止超出窗口 */
  min-width: 300px;
  max-width: calc(100vw - 32px); /* 确保不超出视口 */
  background: white;
  border: 1px solid #ddd;
  border-radius: 8px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
  z-index: 1002; /* 提高层级，确保在GamepadTabs之上 */
  overflow: hidden;
  /* 确保菜单不会超出视口右边界 */
  transform: translateX(0);
}

/* 当菜单可能超出右边界时的调整 */
@media (max-width: 400px) {
  .settings-menu {
    right: -50px; /* 在极小屏幕上稍微向左移动 */
  }
}

.menu-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  background: #f8f9fa;
  border-bottom: 1px solid #e9ecef;
}

.menu-header h4 {
  margin: 0;
  font-size: 1.1em;
  font-weight: 600;
  color: #333;
}

.close-button {
  background: none;
  border: none;
  font-size: 16px;
  cursor: pointer;
  color: #666;
  padding: 4px;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.close-button:hover {
  background: #e9ecef;
  color: #333;
}

.settings-content {
  padding: 20px;
}

.setting-group {
  margin-bottom: 24px;
}

.setting-group:last-child {
  margin-bottom: 0;
}

.setting-label {
  display: block;
  margin-bottom: 8px;
  font-weight: 600;
  color: #333;
  font-size: 0.9em;
}

.setting-options {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.option-button {
  padding: 6px 12px;
  background: #f8f9fa;
  border: 1px solid #e9ecef;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 0.9em;
  min-width: 60px;
}

.option-button:hover {
  background: #e9ecef;
  border-color: #dee2e6;
}

.option-button.selected {
  background: #42b983;
  color: white;
  border-color: #42b983;
}

.deadzone-controls {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.deadzone-item {
  display: flex;
  align-items: center;
  gap: 8px;
}

.deadzone-label {
  font-size: 12px;
  font-weight: 500;
  color: #666;
  min-width: 40px;
}

.deadzone-slider {
  flex: 1;
  height: 4px;
  border-radius: 2px;
  background: #e0e0e0;
  outline: none;
  cursor: pointer;
}

.deadzone-slider::-webkit-slider-thumb {
  appearance: none;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: #42b983;
  cursor: pointer;
  border: 2px solid white;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.deadzone-slider::-moz-range-thumb {
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: #42b983;
  cursor: pointer;
  border: 2px solid white;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.deadzone-value {
  font-size: 11px;
  font-weight: 600;
  color: #42b983;
  min-width: 35px;
  text-align: right;
  font-family: 'Courier New', monospace;
}

.toggle-button {
  padding: 8px 16px;
  background: #f8f9fa;
  border: 1px solid #e9ecef;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 0.9em;
  width: 100%;
}

.toggle-button:hover {
  background: #e9ecef;
}

.toggle-button.active {
  background: #42b983;
  color: white;
  border-color: #42b983;
}

.history-controls {
  display: flex;
  gap: 8px;
  flex-direction: column;
}

.clear-button {
  padding: 6px 12px;
  background: #dc3545;
  color: white;
  border: 1px solid #dc3545;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 0.85em;
}

.clear-button:hover {
  background: #c82333;
  border-color: #c82333;
}

.backdrop {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.1);
  z-index: 999;
}

/* 动画效果 */
.settings-menu-enter-active,
.settings-menu-leave-active {
  transition: all 0.3s ease;
}

.settings-menu-enter-from {
  opacity: 0;
  transform: translateY(-10px) scale(0.95);
}

.settings-menu-leave-to {
  opacity: 0;
  transform: translateY(-10px) scale(0.95);
}

.backdrop-enter-active,
.backdrop-leave-active {
  transition: opacity 0.3s ease;
}

.backdrop-enter-from,
.backdrop-leave-to {
  opacity: 0;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .settings-menu {
    min-width: 280px;
    max-width: calc(100vw - 24px);
  }
}

@media (max-width: 600px) {
  .settings-menu {
    min-width: 260px;
    max-width: calc(100vw - 16px);
  }
}

@media (max-width: 480px) {
  .settings-menu {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    right: auto;
    width: 90vw;
    max-width: 320px;
  }
  
  .settings-content {
    padding: 16px;
  }
  
  .setting-options {
    gap: 6px;
  }
  
  .option-button {
    padding: 8px 10px;
    font-size: 0.8em;
    min-width: 50px;
  }
}

.settings-icon {
  width: 16px;
  height: 16px;
  transition: transform 0.2s ease;
}

.settings-button.active .settings-icon {
  transform: rotate(30deg);
}

/* 响应式图标尺寸 */
</style>
