<template>
  <div class="gamepad-tabs">
    <div class="tabs-container">
      <!-- 显示无手柄状态 -->
      <div v-if="availableIds.length === 0" class="no-gamepad-message">
        <span class="no-gamepad-icon">🎮</span>
        <span class="no-gamepad-text">No Gamepad Connected</span>
      </div>
      
      <!-- 显示可用的手柄标签 -->
      <button
        v-for="id in availableIds"
        :key="id"
        class="gamepad-tab"
        :class="{ 
          active: selectedId === id,
          disabled: !isGamepadAvailable(id)
        }"
        @click="handleTabClick(id)"
        :disabled="!isGamepadAvailable(id)"
      >
        <span class="tab-icon">🎮</span>
        <span class="tab-text">Gamepad {{ id }}</span>
        <span v-if="selectedId === id" class="active-indicator"></span>
      </button>
    </div>
    
    <!-- Settings Panel integrated into tabs -->
    <div class="settings-wrapper">
      <SettingsPanel
        :current-frame-rate="currentFrameRate"
        :current-log-size="currentLogSize"
        :inner-deadzone="innerDeadzone"
        :outer-deadzone="outerDeadzone"
        :frame-rate-options="frameRateOptions"
        :log-size-options="logSizeOptions"
        @update:frame-rate="$emit('update:frameRate', $event)"
        @update:log-size="$emit('update:logSize', $event)"
        @update:inner-deadzone="$emit('update:innerDeadzone', $event)"
        @update:outer-deadzone="$emit('update:outerDeadzone', $event)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { watch, onMounted } from 'vue'
import SettingsPanel from './SettingsPanel.vue'

interface Props {
  selectedId: number
  availableIds: number[]
  isGamepadAvailable: (id: number) => boolean
  currentFrameRate: number
  currentLogSize: number
  innerDeadzone: number
  outerDeadzone: number
  frameRateOptions: number[]
  logSizeOptions: number[]
}

interface Emits {
  (e: 'select', id: number): void
  (e: 'update:frameRate', value: number): void
  (e: 'update:logSize', value: number): void
  (e: 'update:innerDeadzone', value: number): void
  (e: 'update:outerDeadzone', value: number): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const handleTabClick = (id: number) => {
  if (props.isGamepadAvailable(id) && id !== props.selectedId) {
    emit('select', id)
  }
}

// 检查当前选中的手柄是否可用，如果不可用则自动切换
const checkAndSwitchGamepad = () => {
  // 如果没有任何可用的手柄，不需要切换
  if (props.availableIds.length === 0) {
    return
  }
  
  // 如果当前选中的手柄不在可用列表中，或者不可用
  if (props.selectedId === -1 || 
      !props.availableIds.includes(props.selectedId) || 
      !props.isGamepadAvailable(props.selectedId)) {
    
    // 找到第一个可用的手柄
    const firstAvailableId = props.availableIds.find(id => props.isGamepadAvailable(id))
    
    if (firstAvailableId !== undefined && firstAvailableId !== props.selectedId) {
      console.log(`GamepadTabs: Current gamepad ${props.selectedId} not available, switching to ${firstAvailableId}`)
      emit('select', firstAvailableId)
    }
  }
}

// 监听可用手柄列表的变化
watch(() => props.availableIds, () => {
  checkAndSwitchGamepad()
}, { immediate: true })

// 监听当前选中的手柄变化
watch(() => props.selectedId, () => {
  checkAndSwitchGamepad()
})

// 组件挂载时也检查一次
onMounted(() => {
  checkAndSwitchGamepad()
})
</script>

<style scoped>
.gamepad-tabs {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  background: #f5f5f5;
  padding: 2px;
  border-radius: 4px;
  margin-bottom: 6px;
  width: 100%;
  overflow: visible; /* 允许settings菜单溢出 */
  gap: 8px;
}

.tabs-container {
  display: flex;
  gap: 2px;
  flex: 1;
  min-width: 0; /* 允许收缩 */
  overflow: hidden; /* 隐藏溢出内容，不显示滚动条 */
}

.settings-wrapper {
  flex-shrink: 0; /* 防止设置按钮被压缩 */
  position: relative; /* 为settings菜单提供定位上下文 */
  z-index: 1; /* 确保在合适的层级 */
}

.gamepad-tab {
  position: relative;
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 6px 10px;
  background: #e9e9e9;
  border: none;
  border-radius: 3px;
  font-weight: 600;
  color: #666;
  cursor: pointer;
  transition: all 0.2s ease;
  white-space: nowrap;
  min-width: 90px;
  justify-content: center;
  font-size: 12px;
}

.gamepad-tab:hover:not(.disabled) {
  background: #ddd;
  color: #333;
  transform: translateY(-1px);
}

.gamepad-tab.active {
  background: linear-gradient(135deg, #42b983 0%, #369870 100%);
  color: white;
  box-shadow: 0 2px 8px rgba(66, 185, 131, 0.3);
}

.gamepad-tab.active:hover {
  background: linear-gradient(135deg, #369870 0%, #2d7a5f 100%);
}

.gamepad-tab.disabled {
  opacity: 0.4;
  cursor: not-allowed;
  background: #f5f5f5;
  color: #aaa;
}

.gamepad-tab.disabled:hover {
  transform: none;
  background: #f5f5f5;
}

.tab-icon {
  font-size: 12px;
  opacity: 0.8;
}

.gamepad-tab.active .tab-icon {
  opacity: 1;
}

.tab-text {
  font-size: 11px;
}

.active-indicator {
  position: absolute;
  bottom: -2px;
  left: 50%;
  transform: translateX(-50%);
  width: 20px;
  height: 3px;
  background: white;
  border-radius: 2px;
  opacity: 0.9;
}

/* 响应式设计 - 基于新的布局断点 */

/* 宽屏和超宽屏合并 (1000px+) */
@media (min-width: 1000px) {
  .gamepad-tabs {
    padding: 3px;
    margin-bottom: 8px;
    gap: 12px;
  }
  
  .tabs-container {
    gap: 3px;
  }
  
  .gamepad-tab {
    padding: 8px 14px;
    min-width: 110px;
    font-size: 13px;
    gap: 6px;
  }
  
  .tab-icon {
    font-size: 14px;
  }
  
  .tab-text {
    font-size: 12px;
  }
}

/* 中屏：紧凑显示 (768-999px) */
@media (min-width: 768px) and (max-width: 999px) {
  .gamepad-tabs {
    padding: 2px;
    margin-bottom: 6px;
    gap: 8px;
  }
  
  .tabs-container {
    gap: 2px;
  }
  
  .gamepad-tab {
    padding: 6px 10px;
    min-width: 85px;
    font-size: 11px;
    gap: 4px;
  }
  
  .tab-icon {
    font-size: 11px;
  }
  
  .tab-text {
    font-size: 10px;
  }
}

/* 小屏：更紧凑显示 (480-767px) */
@media (min-width: 480px) and (max-width: 767px) {
  .gamepad-tabs {
    padding: 2px;
    margin-bottom: 5px;
    gap: 6px;
  }
  
  .tabs-container {
    gap: 1px;
  }
  
  .gamepad-tab {
    padding: 6px 8px;
    min-width: 75px;
    font-size: 10px;
    gap: 3px;
  }
  
  .tab-icon {
    font-size: 10px;
  }
  
  .tab-text {
    font-size: 9px;
  }
}

/* 极小屏：垂直布局 */
@media (max-width: 479px) {
  .gamepad-tabs {
    flex-direction: column;
    gap: 4px;
    padding: 4px;
  }
  
  .tabs-container {
    width: 100%;
    justify-content: center;
  }
  
  .settings-wrapper {
    align-self: center;
  }
}

.no-gamepad-message {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background: #f5f5f5;
  border: 1px solid #ddd;
  border-radius: 3px;
  color: #999;
  font-weight: 500;
  font-size: 0.9em;
}

.no-gamepad-icon {
  opacity: 0.6;
}

.no-gamepad-text {
  white-space: nowrap;
}
</style>
