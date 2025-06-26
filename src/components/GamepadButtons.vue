<template>
  <div class="gamepad-buttons">
    <h3>Buttons</h3>
    <div class="buttons-grid">
      <div
        v-for="buttonKey in BUTTON_ORDER"
        :key="buttonKey"
        class="button-item"
        :class="{ pressed: isButtonPressed(buttonKey) }"
      >
        <div class="progress-container">
          <div 
            class="progress-bar" 
            :style="getProgressStyle(buttonKey)"
          ></div>
        </div>
        <div class="button-info">
          <div class="button-name">{{ getButtonDisplayName(buttonKey) }}</div>
          <div class="button-value">{{ getButtonValueText(buttonKey) }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'
import type { GamepadInfo } from '../composables/useGamepadState'

interface Props {
  gamepad: GamepadInfo
}

const props = defineProps<Props>()

// 响应式窗口宽度
const windowWidth = ref(typeof window !== 'undefined' ? window.innerWidth : 1024)

// 固定的按钮顺序
const BUTTON_ORDER = [
  "A", "B", "X", "Y",
  "LeftShoulder", "RightShoulder",
  "LeftTrigger", "RightTrigger",
  "Back", "Start",
  "LeftThumb", "RightThumb",
  "DPadUp", "DPadDown", "DPadLeft", "DPadRight"
] as const

// 优化的计算函数
const getButtonValue = (buttonKey: string): number => {
  return props.gamepad?.buttons[buttonKey]?.value || 0
}

const isButtonPressed = (buttonKey: string): boolean => {
  return props.gamepad?.buttons[buttonKey]?.is_pressed || false
}

const getButtonValueText = (buttonKey: string): string => {
  return getButtonValue(buttonKey).toFixed(2)
}

// 按钮名称缩写映射（用于小屏幕）
const getButtonDisplayName = (buttonKey: string): string => {
  // 使用响应式的窗口宽度
  const width = windowWidth.value
  
  // 超小屏幕使用极简缩写
  if (width <= 360) {
    const miniNames: Record<string, string> = {
      "LeftShoulder": "LB",
      "RightShoulder": "RB", 
      "LeftTrigger": "LT",
      "RightTrigger": "RT",
      "LeftThumb": "L3",
      "RightThumb": "R3",
      "DPadUp": "U",
      "DPadDown": "D", 
      "DPadLeft": "L",
      "DPadRight": "R",
      "Back": "Bk",
      "Start": "St"
    }
    return miniNames[buttonKey] || buttonKey.substring(0, 2)
  }
  
  // 小屏幕使用标准缩写
  if (width <= 600) {
    const shortNames: Record<string, string> = {
      "LeftShoulder": "LB",
      "RightShoulder": "RB", 
      "LeftTrigger": "LT",
      "RightTrigger": "RT",
      "LeftThumb": "L3",
      "RightThumb": "R3",
      "DPadUp": "D↑",
      "DPadDown": "D↓", 
      "DPadLeft": "D←",
      "DPadRight": "D→",
      "Back": "Back",
      "Start": "Start"
    }
    return shortNames[buttonKey] || buttonKey
  }
  
  // 大屏幕使用完整名称
  return buttonKey
}

// 使用 CSS 自定义属性来避免频繁的样式计算
const getProgressStyle = (buttonKey: string) => {
  const value = getButtonValue(buttonKey)
  const percentage = Math.max(0, Math.min(100, value * 100))
  
  return {
    '--progress-height': `${percentage}%`,
    '--progress-color': isButtonPressed(buttonKey) ? '#ff4757' : '#42b983'
  }
}

// 窗口大小变化处理
const handleResize = () => {
  windowWidth.value = window.innerWidth
}

// 组件挂载时添加监听器
onMounted(() => {
  if (typeof window !== 'undefined') {
    window.addEventListener('resize', handleResize)
  }
})

// 组件卸载时移除监听器
onBeforeUnmount(() => {
  if (typeof window !== 'undefined') {
    window.removeEventListener('resize', handleResize)
  }
})
</script>

<style scoped>
.gamepad-buttons {
  width: 100%;
  min-width: 280px;
}

.gamepad-buttons h3 {
  margin: 0 0 16px 0;
  color: #333;
  font-size: 1.2em;
  font-weight: 600;
}

.buttons-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 8px;
  width: 100%;
  min-width: 0; /* 防止网格溢出 */
}

.button-item {
  display: flex;
  align-items: center;
  background: #f9f9f9;
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 6px;
  transition: all 0.1s ease;
  position: relative;
  min-width: 0; /* 防止内容溢出 */
  box-sizing: border-box;
}

.button-item.pressed {
  background: #ffebee;
  border-color: #ff4757;
  transform: scale(0.98);
}

.progress-container {
  width: 8px;
  height: 40px;
  margin-right: 8px;
  position: relative;
  background: #e0e0e0;
  border-radius: 4px;
  overflow: hidden;
  flex-shrink: 0;
}

.progress-bar {
  position: absolute;
  bottom: 0;
  left: 0;
  width: 100%;
  height: var(--progress-height, 0%);
  background: var(--progress-color, #42b983);
  border-radius: 4px;
  transition: height 0.05s ease-out, background-color 0.1s ease;
}

.button-info {
  display: flex;
  flex-direction: column;
  justify-content: center;
  min-width: 0;
  flex: 1;
}

.button-name {
  font-weight: 600;
  font-size: 0.9em;
  margin-bottom: 2px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  color: #333;
}

.button-value {
  font-size: 0.75em;
  color: #666;
  font-family: 'Courier New', monospace;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .buttons-grid {
    grid-template-columns: repeat(4, 1fr);
    gap: 6px;
  }
  
  .button-item {
    padding: 6px 4px;
    font-size: 0.9em;
  }
  
  .progress-container {
    height: 35px;
    width: 6px;
    margin-right: 4px;
  }
  
  .button-name {
    font-size: 0.75em;
  }
  
  .button-value {
    font-size: 0.65em;
  }
}

@media (max-width: 600px) {
  .buttons-grid {
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
  }
  
  .button-item {
    padding: 8px 6px;
    flex-direction: column;
    align-items: center;
    min-height: 65px;
    text-align: center;
  }
  
  .progress-container {
    width: 100%;
    height: 6px;
    margin-right: 0;
    margin-bottom: 4px;
  }
  
  .progress-bar {
    width: var(--progress-height, 0%);
    height: 100%;
    left: 0;
    bottom: 0;
  }
  
  .button-info {
    width: 100%;
    align-items: center;
  }
  
  .button-name {
    font-size: 0.75em;
    margin-bottom: 2px;
    text-align: center;
  }
  
  .button-value {
    font-size: 0.7em;
  }
}

@media (max-width: 480px) {
  .gamepad-buttons {
    min-width: 240px;
  }
  
  .buttons-grid {
    grid-template-columns: repeat(2, 1fr);
    gap: 6px;
  }
  
  .button-item {
    padding: 8px 4px;
    min-height: 60px;
  }
  
  .button-name {
    font-size: 0.7em;
    line-height: 1.2;
  }
  
  .button-value {
    font-size: 0.65em;
  }
}

@media (max-width: 360px) {
  .gamepad-buttons {
    min-width: 200px;
  }
  
  .buttons-grid {
    grid-template-columns: repeat(2, 1fr);
    gap: 4px;
  }
  
  .button-item {
    padding: 6px 3px;
    min-height: 55px;
  }
  
  .button-name {
    font-size: 0.65em;
    line-height: 1.1;
    font-weight: 700; /* 增强可读性 */
  }
  
  .button-value {
    font-size: 0.6em;
  }
  
  .progress-container {
    height: 5px;
    margin-bottom: 3px;
  }
}

@media (max-width: 320px) {
  .gamepad-buttons {
    min-width: 180px;
  }
  
  .buttons-grid {
    grid-template-columns: 1fr 1fr;
    gap: 3px;
  }
  
  .button-item {
    padding: 4px 2px;
    min-height: 50px;
  }
  
  .button-name {
    font-size: 0.6em;
    line-height: 1.0;
    font-weight: 700;
  }
  
  .button-value {
    font-size: 0.55em;
  }
  
  .progress-container {
    height: 4px;
    margin-bottom: 2px;
  }
}
</style>
