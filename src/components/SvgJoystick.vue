<template>
  <div class="svg-joystick-container">
    <svg 
      width="120" 
      height="120" 
      viewBox="0 0 120 120" 
      class="joystick-svg"
    >
      <!-- 背景网格 -->
      <defs>
        <pattern id="grid" width="24" height="24" patternUnits="userSpaceOnUse">
          <path d="M 24 0 L 0 0 0 24" fill="none" stroke="#f0f0f0" stroke-width="1" opacity="0.5"/>
        </pattern>
      </defs>
      
      <!-- 背景 -->
      <rect width="120" height="120" fill="white" stroke="#ddd" stroke-width="2" rx="4"/>
      <rect x="5" y="5" width="110" height="110" fill="url(#grid)"/>
      
      <!-- 中心线 -->
      <line x1="60" y1="8" x2="60" y2="112" stroke="#ddd" stroke-width="1"/>
      <line x1="8" y1="60" x2="112" y2="60" stroke="#ddd" stroke-width="1"/>
      
      <!-- 外圆边界 -->
      <circle cx="60" cy="60" r="52" fill="none" stroke="#ddd" stroke-width="2"/>
      
      <!-- 死区圆环（如果有死区设置） -->
      <circle 
        v-if="innerDeadzone > 0"
        cx="60" 
        cy="60" 
        :r="innerDeadzone * 52" 
        fill="none" 
        stroke="#ff6b6b" 
        stroke-width="1"
        stroke-dasharray="3,3"
        opacity="0.6"
      />
      
      <circle 
        v-if="outerDeadzone < 1"
        cx="60" 
        cy="60" 
        :r="outerDeadzone * 52" 
        fill="none" 
        stroke="#ff6b6b" 
        stroke-width="1"
        stroke-dasharray="3,3"
        opacity="0.6"
      />
      
      <!-- 历史轨迹 -->
      <g v-if="showHistory && historyPoints && historyPoints.length > 1">
        <!-- 轨迹线 -->
        <polyline
          :points="historyPointsString"
          fill="none"
          stroke="#ff6b6b"
          stroke-width="2"
          opacity="0.4"
          stroke-linejoin="round"
          stroke-linecap="round"
        />
        
        <!-- 历史点（只显示最近的几个以优化性能） -->
        <circle
          v-for="(point, index) in recentHistoryPoints"
          :key="`history-${index}`"
          :cx="point.x"
          :cy="point.y"
          :r="Math.max(1, 3 * point.opacity)"
          :fill="`rgba(255, 107, 107, ${point.opacity})`"
          class="history-point"
        />
      </g>
      
      <!-- 当前位置连线 -->
      <line
        x1="60"
        y1="60"
        :x2="currentX"
        :y2="currentY"
        stroke="#666"
        stroke-width="2"
        class="current-line"
      />
      
      <!-- 当前位置点 -->
      <circle
        :cx="currentX"
        :cy="currentY"
        r="4"
        :fill="currentPointColor"
        stroke="white"
        stroke-width="2"
        class="current-point"
      />
    </svg>
    
    <!-- 数值显示条 -->
    <div class="axis-bars">
      <div class="axis-bar x-axis">
        <label>X: {{ formatAxisValue(axisX) }}</label>
        <div class="bar-track">
          <div 
            class="bar-fill"
            :style="xAxisStyle"
          ></div>
        </div>
      </div>
      
      <div class="axis-bar y-axis">
        <label>Y: {{ formatAxisValue(axisY) }}</label>
        <div class="bar-track">
          <div 
            class="bar-fill"
            :style="yAxisStyle"
          ></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface HistoryPoint {
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

const props = withDefaults(defineProps<Props>(), {
  axisX: 0,
  axisY: 0,
  historyPoints: () => [],
  showHistory: false,
  innerDeadzone: 0,
  outerDeadzone: 1
})

// 统一的坐标转换函数，确保所有元素使用相同的计算
const transformToSvgCoords = (axisX: number, axisY: number) => ({
  x: 60 + axisX * 52,
  y: 60 - axisY * 52 // Y轴翻转
})

// 将轴值转换为SVG坐标 - 使用统一函数
const currentCoords = computed(() => transformToSvgCoords(props.axisX, props.axisY))
const currentX = computed(() => currentCoords.value.x)
const currentY = computed(() => currentCoords.value.y)

// 计算当前点颜色（基于死区）
const currentPointColor = computed(() => {
  const magnitude = Math.sqrt(props.axisX ** 2 + props.axisY ** 2)
  
  if (magnitude < props.innerDeadzone || magnitude > props.outerDeadzone) {
    return '#ff6b6b' // 红色：在死区内或超出外死区
  }
  return '#42b983' // 绿色：在有效范围内
})

// 生成历史轨迹的点字符串（用于polyline）- 使用统一的坐标转换
const historyPointsString = computed(() => {
  if (!props.historyPoints || props.historyPoints.length === 0) return ''
  
  return props.historyPoints
    .map(point => {
      const coords = transformToSvgCoords(point.x, point.y)
      return `${coords.x},${coords.y}`
    })
    .join(' ')
})

// 只显示最近的历史点以优化性能 - 使用统一的坐标转换
const recentHistoryPoints = computed(() => {
  if (!props.historyPoints || props.historyPoints.length === 0) return []
  
  const maxPoints = 15 // 减少到15个点以提高性能
  const points = props.historyPoints.slice(-maxPoints)
  
  return points.map((point, index) => {
    const coords = transformToSvgCoords(point.x, point.y)
    return {
      x: coords.x,
      y: coords.y,
      opacity: (index + 1) / points.length // 渐变透明度
    }
  })
})

// X轴样式 - 减少transition时间以提高响应性
const xAxisStyle = computed(() => ({
  width: `${Math.abs(props.axisX) * 50}%`,
  left: props.axisX > 0 ? '50%' : `${50 - Math.abs(props.axisX) * 50}%`,
  backgroundColor: currentPointColor.value,
  transition: 'all 0.02s ease' // 更快的响应
}))

// Y轴样式 - 减少transition时间以提高响应性
const yAxisStyle = computed(() => ({
  width: `${Math.abs(props.axisY) * 50}%`,
  left: props.axisY > 0 ? '50%' : `${50 - Math.abs(props.axisY) * 50}%`,
  backgroundColor: currentPointColor.value,
  transition: 'all 0.02s ease' // 更快的响应
}))

// 格式化轴值显示
const formatAxisValue = (value: number): string => {
  return value.toFixed(3)
}
</script>

<style scoped>
.svg-joystick-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px; /* 减少间距 */
  padding: 4px; /* 减少内边距 */
}

.joystick-svg {
  border-radius: 6px; /* 减小圆角 */
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.1); /* 减小阴影 */
  background: white;
}

/* SVG元素的硬件加速动画 - 移除位置transition以消除迟滞 */
.current-point {
  filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.2));
  will-change: cx, cy, fill; /* 启用GPU加速 */
  /* 只对颜色变化添加transition，位置变化即时响应 */
  transition: fill 0.1s ease;
}

.current-line {
  will-change: x2, y2; /* 启用GPU加速 */
  /* 移除transition，确保与current-point同步 */
}

.history-point {
  transition: opacity 0.2s ease;
}

/* 启用硬件加速 */
.joystick-svg {
  transform: translateZ(0); /* 强制GPU层 */
  backface-visibility: hidden; /* 避免闪烁 */
}

/* 轴值显示条 */
.axis-bars {
  display: flex;
  flex-direction: column;
  gap: 4px; /* 进一步减少间距 */
  width: 120px; /* 匹配SVG宽度 */
}

.axis-bar {
  display: flex;
  align-items: center;
  gap: 4px; /* 进一步减少间距 */
}

.axis-bar label {
  font-size: 10px; /* 进一步减小字体 */
  font-weight: 600;
  color: #333;
  min-width: 55px; /* 进一步减少宽度 */
  font-family: 'Courier New', monospace;
}

.bar-track {
  flex: 1;
  height: 10px; /* 进一步减小高度 */
  background: #f0f0f0;
  border-radius: 5px; /* 减小圆角 */
  position: relative;
  border: 1px solid #ddd;
  overflow: hidden;
}

.bar-fill {
  position: absolute;
  height: 100%;
  border-radius: 3px; /* 进一步减小圆角 */
  top: 0;
  transition: all 0.02s ease;
  will-change: width, left, background-color;
}

/* 响应式设计 - 基于新的布局断点 */

/* 宽屏和超宽屏合并：标准尺寸 (1000px+) */
@media (min-width: 1000px) {
  .svg-joystick-container {
    gap: 10px;
  }
  
  .joystick-svg {
    width: 180px;
    height: 180px;
  }
  
  .axis-bars {
    width: 180px;
  }
  
  .axis-bar label {
    min-width: 65px;
    font-size: 11px;
  }
  
  .bar-track {
    height: 12px;
  }
}

/* 中屏：紧凑尺寸 (768-999px) */
@media (min-width: 768px) and (max-width: 999px) {
  .svg-joystick-container {
    gap: 6px;
  }
  
  .joystick-svg {
    width: 160px; /* 缩小以适应紧凑布局 */
    height: 160px;
  }
  
  .axis-bars {
    width: 160px;
  }
  
  .axis-bar label {
    min-width: 50px; /* 缩小标签宽度 */
    font-size: 9px; /* 缩小字体 */
  }
  
  .bar-track {
    height: 10px;
  }
}

/* 小屏：更紧凑尺寸 (480-767px) */
@media (min-width: 480px) and (max-width: 767px) {
  .svg-joystick-container {
    gap: 5px;
  }
  
  .joystick-svg {
    width: 140px; /* 进一步缩小 */
    height: 140px;
  }
  
  .axis-bars {
    width: 140px;
  }
  
  .axis-bar label {
    min-width: 45px;
    font-size: 8px; /* 更小字体 */
  }
  
  .bar-track {
    height: 9px;
  }
}
</style>
