<template>
  <div class="settings-panel">
    <button 
      class="settings-button" 
      @click="toggleSettings"
      :class="{ active: isOpen }"
    >
      ⚙️ Settings
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
  padding: 8px 16px;
  background: #f0f0f0;
  border: 1px solid #ddd;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 14px;
  font-weight: 500;
}

.settings-button:hover {
  background: #e0e0e0;
  transform: translateY(-1px);
}

.settings-button.active {
  background: #42b983;
  color: white;
  border-color: #42b983;
}

.settings-menu {
  position: absolute;
  top: calc(100% + 8px);
  left: 0; /* 改为从左边对齐，避免右侧越界 */
  min-width: 300px;
  max-width: calc(100vw - 32px); /* 确保不超出视口 */
  background: white;
  border: 1px solid #ddd;
  border-radius: 8px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
  z-index: 1000;
  overflow: hidden;
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
</style>
