<template>
  <div class="gamepad-tabs">
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
</template>

<script setup lang="ts">
interface Props {
  selectedId: number
  availableIds: number[]
  isGamepadAvailable: (id: number) => boolean
}

interface Emits {
  (e: 'select', id: number): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const handleTabClick = (id: number) => {
  if (props.isGamepadAvailable(id) && id !== props.selectedId) {
    emit('select', id)
  }
}
</script>

<style scoped>
.gamepad-tabs {
  display: flex;
  gap: 2px;
  background: #f5f5f5;
  padding: 4px;
  border-radius: 8px;
  margin-bottom: 20px;
  width: 100%;
  overflow-x: auto;
}

.gamepad-tab {
  position: relative;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  background: #e9e9e9;
  border: none;
  border-radius: 6px;
  font-weight: 600;
  color: #666;
  cursor: pointer;
  transition: all 0.2s ease;
  white-space: nowrap;
  min-width: 120px;
  justify-content: center;
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
  font-size: 16px;
  opacity: 0.8;
}

.gamepad-tab.active .tab-icon {
  opacity: 1;
}

.tab-text {
  font-size: 14px;
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

/* 响应式设计 */
@media (max-width: 768px) {
  .gamepad-tabs {
    gap: 1px;
    padding: 2px;
  }
  
  .gamepad-tab {
    padding: 10px 12px;
    min-width: 100px;
    font-size: 13px;
  }
  
  .tab-icon {
    font-size: 14px;
  }
  
  .tab-text {
    font-size: 13px;
  }
}

@media (max-width: 480px) {
  .gamepad-tab {
    padding: 8px 10px;
    min-width: 80px;
    gap: 4px;
  }
  
  .tab-icon {
    font-size: 12px;
  }
  
  .tab-text {
    font-size: 12px;
  }
}
</style>
