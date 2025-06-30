<template>
  <div class="joystick-visualization">
    <SvgJoystick
      :axis-x="axisX"
      :axis-y="axisY"
      :history-points="historyPoints"
      :show-history="showHistory"
      :inner-deadzone="innerDeadzone"
      :outer-deadzone="outerDeadzone"
    />
  </div>
</template>

<script setup lang="ts">
import SvgJoystick from './SvgJoystick.vue'

export interface HistoryPoint {
  x: number
  y: number
}

interface Props {
  axisX: number
  axisY: number
  historyPoints?: HistoryPoint[]
  showHistory?: boolean
  innerDeadzone?: number
  outerDeadzone?: number
}

withDefaults(defineProps<Props>(), {
  axisX: 0,
  axisY: 0,
  historyPoints: () => [],
  showHistory: false,
  innerDeadzone: 0.05,
  outerDeadzone: 1.0
})
</script>

<style scoped>
.joystick-visualization {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 100%;
  min-width: 120px; /* 进一步减少最小宽度 */
  max-width: 160px; /* 减少最大宽度以适应三栏布局 */
  margin: 0 auto; /* 居中显示 */
}

/* 响应式优化 - 基于新的布局断点 */

/* 宽屏和超宽屏合并：标准尺寸 (1000px+) */
@media (min-width: 1000px) {
  .joystick-visualization {
    min-width: 140px;
    max-width: 180px;
  }
}

/* 中屏：紧凑尺寸 (768-999px) */
@media (min-width: 768px) and (max-width: 999px) {
  .joystick-visualization {
    min-width: 115px;
    max-width: 150px;
  }
}

/* 小屏：更紧凑尺寸 (480-767px) */
@media (min-width: 480px) and (max-width: 767px) {
  .joystick-visualization {
    min-width: 105px;
    max-width: 140px;
  }
}
</style>
