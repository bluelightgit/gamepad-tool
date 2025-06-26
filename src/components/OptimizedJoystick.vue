<template>
  <div class="joystick-visualization">
    <canvas 
      ref="canvasRef" 
      :width="canvasSize" 
      :height="canvasSize" 
      class="joystick-canvas"
    ></canvas>
    
    <div class="axis-bars">
      <div class="axis-bar x-axis">
        <div class="bar-background"></div>
        <div class="bar-fill" :style="xAxisStyle"></div>
        <span class="bar-label">X: {{ formatValue(axisX) }}</span>
      </div>
      
      <div class="axis-bar y-axis">
        <div class="bar-background"></div>
        <div class="bar-fill" :style="yAxisStyle"></div>
        <span class="bar-label">Y: {{ formatValue(axisY) }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onBeforeUnmount } from 'vue'

export interface HistoryPoint {
  x: number
  y: number
}

interface Props {
  axisX: number
  axisY: number
  historyPoints?: HistoryPoint[]
  showHistory?: boolean
  size?: number
  innerDeadzone?: number
  outerDeadzone?: number
}

const props = withDefaults(defineProps<Props>(), {
  axisX: 0,
  axisY: 0,
  historyPoints: () => [],
  showHistory: false,
  size: 180,
  innerDeadzone: 0.05,
  outerDeadzone: 1.0
})

const canvasRef = ref<HTMLCanvasElement | null>(null)
const windowWidth = ref(typeof window !== 'undefined' ? window.innerWidth : 1024)

// 响应式 canvas 大小
const canvasSize = computed(() => {
  const width = windowWidth.value
  if (width <= 360) return Math.min(props.size, 100)
  if (width <= 480) return Math.min(props.size, 120)
  if (width <= 600) return Math.min(props.size, 140)
  if (width <= 768) return Math.min(props.size, 160)
  return props.size
})

// 格式化数值显示
const formatValue = (value: number): string => {
  return value.toFixed(3)
}

// 计算轴条样式
const xAxisStyle = computed(() => {
  const absValue = Math.abs(props.axisX)
  const percentage = Math.min(50, absValue * 50)
  const distance = Math.sqrt(props.axisX ** 2 + props.axisY ** 2)
  const isInDeadzone = distance < props.innerDeadzone || distance > props.outerDeadzone
  
  return {
    width: `${percentage}%`,
    left: props.axisX >= 0 ? '50%' : `${50 - percentage}%`,
    backgroundColor: isInDeadzone ? '#ff4757' : '#42b983'
  }
})

const yAxisStyle = computed(() => {
  const absValue = Math.abs(props.axisY)
  const percentage = Math.min(50, absValue * 50)
  const distance = Math.sqrt(props.axisX ** 2 + props.axisY ** 2)
  const isInDeadzone = distance < props.innerDeadzone || distance > props.outerDeadzone
  
  return {
    width: `${percentage}%`,
    left: props.axisY >= 0 ? '50%' : `${50 - percentage}%`,
    backgroundColor: isInDeadzone ? '#ff4757' : '#42b983'
  }
})

// Canvas 绘制函数
let animationFrameId: number | null = null
let lastDrawTime = 0
const DRAW_THROTTLE = 16 // ~60fps for drawing

const drawJoystick = () => {
  const canvas = canvasRef.value
  if (!canvas) return
  
  const ctx = canvas.getContext('2d')
  if (!ctx) return
  
  const size = canvasSize.value
  const center = size / 2
  const radius = (size - 20) / 2
  
  // 清空画布
  ctx.clearRect(0, 0, size, size)
  
  // 绘制背景圆圈
  ctx.strokeStyle = '#ddd'
  ctx.lineWidth = 2
  ctx.beginPath()
  ctx.arc(center, center, radius, 0, 2 * Math.PI)
  ctx.stroke()
  
  // 绘制内部deadzone圆圈
  const innerRadius = props.innerDeadzone * radius
  if (innerRadius > 0) {
    ctx.strokeStyle = '#ffcccc'
    ctx.lineWidth = 1
    ctx.setLineDash([3, 3])
    ctx.beginPath()
    ctx.arc(center, center, innerRadius, 0, 2 * Math.PI)
    ctx.stroke()
    ctx.setLineDash([])
  }
  
  // 绘制外部deadzone圆圈
  const outerRadius = props.outerDeadzone * radius
  if (outerRadius < radius) {
    ctx.strokeStyle = '#ffcccc'
    ctx.lineWidth = 1
    ctx.setLineDash([3, 3])
    ctx.beginPath()
    ctx.arc(center, center, outerRadius, 0, 2 * Math.PI)
    ctx.stroke()
    ctx.setLineDash([])
  }
  
  // 绘制十字线
  ctx.strokeStyle = '#eee'
  ctx.lineWidth = 1
  ctx.setLineDash([])
  ctx.beginPath()
  ctx.moveTo(10, center)
  ctx.lineTo(size - 10, center)
  ctx.moveTo(center, 10)
  ctx.lineTo(center, size - 10)
  ctx.stroke()
  
  // 绘制历史轨迹
  if (props.showHistory && props.historyPoints && props.historyPoints.length > 1) {
    const points = props.historyPoints
    const len = points.length
    
    // 绘制轨迹线
    ctx.strokeStyle = 'rgba(255, 107, 107, 0.4)'
    ctx.lineWidth = 2
    ctx.beginPath()
    
    points.forEach((point, index) => {
      const x = center + point.x * radius
      const y = center - point.y * radius // Y轴翻转
      
      if (index === 0) {
        ctx.moveTo(x, y)
      } else {
        ctx.lineTo(x, y)
      }
    })
    ctx.stroke()
    
    // 绘制历史点（带渐变透明度）
    points.forEach((point, index) => {
      const x = center + point.x * radius
      const y = center - point.y * radius
      const alpha = (index + 1) / len
      
      ctx.fillStyle = `rgba(255, 71, 87, ${alpha * 0.6})`
      ctx.beginPath()
      ctx.arc(x, y, 2, 0, 2 * Math.PI)
      ctx.fill()
    })
  }
  
  // 绘制当前位置连线
  const currentX = center + props.axisX * radius
  const currentY = center - props.axisY * radius
  
  ctx.strokeStyle = '#666'
  ctx.lineWidth = 1
  ctx.beginPath()
  ctx.moveTo(center, center)
  ctx.lineTo(currentX, currentY)
  ctx.stroke()
  
  // 绘制当前位置点
  const distance = Math.sqrt(props.axisX ** 2 + props.axisY ** 2)
  const isInDeadzone = distance < props.innerDeadzone || distance > props.outerDeadzone
  ctx.fillStyle = isInDeadzone ? '#ff4757' : '#42b983'
  ctx.beginPath()
  ctx.arc(currentX, currentY, 5, 0, 2 * Math.PI)
  ctx.fill()
  
  // 添加白色边框
  ctx.strokeStyle = 'white'
  ctx.lineWidth = 2
  ctx.stroke()
}

// 节流绘制函数
const throttledDraw = () => {
  const now = performance.now()
  if (now - lastDrawTime < DRAW_THROTTLE) return
  
  lastDrawTime = now
  drawJoystick()
}

// 响应式重绘
const scheduleRedraw = () => {
  if (animationFrameId) return
  
  animationFrameId = requestAnimationFrame(() => {
    throttledDraw()
    animationFrameId = null
  })
}

// 监听属性变化
watch([() => props.axisX, () => props.axisY, () => props.historyPoints, canvasSize], scheduleRedraw, {
  deep: true
})

// 窗口大小变化处理
const handleResize = () => {
  windowWidth.value = window.innerWidth
}

// 组件挂载时初始化
onMounted(() => {
  scheduleRedraw()
  
  // 添加窗口大小变化监听器
  if (typeof window !== 'undefined') {
    window.addEventListener('resize', handleResize)
  }
})

// 组件卸载时清理
onBeforeUnmount(() => {
  if (animationFrameId) {
    cancelAnimationFrame(animationFrameId)
  }
  
  // 移除窗口大小变化监听器
  if (typeof window !== 'undefined') {
    window.removeEventListener('resize', handleResize)
  }
})
</script>

<style scoped>
.joystick-visualization {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 8px;
}

.joystick-canvas {
  border-radius: 8px;
  background: #fff;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  border: 1px solid #e0e0e0;
}

.axis-bars {
  display: flex;
  flex-direction: column;
  gap: 8px;
  width: 100%;
  max-width: 180px;
}

.axis-bar {
  position: relative;
  height: 20px;
  background: #f0f0f0;
  border: 1px solid #ddd;
  border-radius: 10px;
  overflow: hidden;
}

.bar-background {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: #f0f0f0;
}

.bar-fill {
  position: absolute;
  top: 0;
  height: 100%;
  transition: all 0.1s ease;
  border-radius: 10px;
}

.bar-label {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-size: 11px;
  font-weight: 600;
  color: #333;
  text-shadow: 1px 1px 2px rgba(255, 255, 255, 0.8);
  pointer-events: none;
  z-index: 2;
  font-family: 'Courier New', monospace;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .joystick-visualization {
    gap: 10px;
    padding: 8px;
  }
  
  .axis-bar {
    height: 18px;
  }
  
  .bar-label {
    font-size: 10px;
  }
}

@media (max-width: 600px) {
  .joystick-visualization {
    gap: 8px;
    padding: 6px;
    max-width: 160px;
  }
  
  .axis-bar {
    height: 16px;
  }
  
  .bar-label {
    font-size: 9px;
  }
}

@media (max-width: 480px) {
  .joystick-visualization {
    gap: 6px;
    padding: 4px;
    max-width: 140px;
  }
  
  .axis-bar {
    height: 14px;
  }
  
  .bar-label {
    font-size: 8px;
  }
}

@media (max-width: 360px) {
  .joystick-visualization {
    gap: 4px;
    padding: 2px;
    max-width: 120px;
  }
  
  .axis-bar {
    height: 12px;
  }
  
  .bar-label {
    font-size: 7px;
  }
}
</style>
