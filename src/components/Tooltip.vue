<template>
  <div 
    ref="wrapperRef"
    class="tooltip-wrapper" 
    @mouseenter="showTooltip" 
    @mouseleave="hideTooltip"
  >
    <slot></slot>
    <Teleport to="body">
      <Transition name="tooltip">
        <div 
          v-if="visible" 
          ref="tooltipRef"
          class="tooltip" 
          :class="actualPosition"
          :style="tooltipStyle"
        >
          {{ text }}
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, nextTick } from 'vue'

interface Props {
  text: string
  position?: 'top' | 'bottom' | 'left' | 'right'
  delay?: number
}

const props = withDefaults(defineProps<Props>(), {
  position: 'top',
  delay: 300
})

const visible = ref(false)
const wrapperRef = ref<HTMLElement>()
const tooltipRef = ref<HTMLElement>()
const actualPosition = ref(props.position)
let timeoutId: number | null = null

// 计算最佳位置
const calculateBestPosition = (): 'top' | 'bottom' | 'left' | 'right' => {
  if (!wrapperRef.value || !tooltipRef.value) {
    return props.position
  }

  const wrapper = wrapperRef.value.getBoundingClientRect()
  const tooltip = tooltipRef.value.getBoundingClientRect()
  const viewport = {
    width: window.innerWidth,
    height: window.innerHeight
  }

  const positions: Array<'top' | 'bottom' | 'left' | 'right'> = ['top', 'bottom', 'left', 'right']
  const spaceNeeded = {
    top: tooltip.height + 16,
    bottom: tooltip.height + 16,
    left: tooltip.width + 16,
    right: tooltip.width + 16
  }

  const availableSpace = {
    top: wrapper.top,
    bottom: viewport.height - wrapper.bottom,
    left: wrapper.left,
    right: viewport.width - wrapper.right
  }

  // 首选用户指定的位置，如果空间足够
  if (availableSpace[props.position] >= spaceNeeded[props.position]) {
    return props.position
  }

  // 否则找到空间最大的位置
  let bestPosition: 'top' | 'bottom' | 'left' | 'right' = props.position
  let maxSpace = availableSpace[props.position]

  for (const pos of positions) {
    if (availableSpace[pos] >= spaceNeeded[pos] && availableSpace[pos] > maxSpace) {
      bestPosition = pos
      maxSpace = availableSpace[pos]
    }
  }

  return bestPosition
}

// 动态样式
const tooltipStyle = computed(() => {
  if (!wrapperRef.value || !tooltipRef.value) {
    return {}
  }

  const wrapper = wrapperRef.value.getBoundingClientRect()
  const tooltip = tooltipRef.value.getBoundingClientRect()
  const viewport = {
    width: window.innerWidth,
    height: window.innerHeight
  }

  let style: Record<string, string> = {}

  // 因为使用了Teleport到body，需要使用fixed定位
  style.position = 'fixed'

  // 根据位置调整样式
  if (actualPosition.value === 'top') {
    style.top = `${wrapper.top - tooltip.height - 8}px`
    // 水平居中，但限制偏移以保持箭头指向
    const centerLeft = wrapper.left + wrapper.width / 2 - tooltip.width / 2
    const maxOffset = Math.min(tooltip.width * 0.3, 40)
    const leftBound = Math.max(8, centerLeft - maxOffset)
    const rightBound = Math.min(viewport.width - tooltip.width - 8, centerLeft + maxOffset)
    
    const finalLeft = Math.max(leftBound, Math.min(rightBound, centerLeft))
    style.left = `${finalLeft}px`
    
    // 箭头位置
    const arrowLeft = wrapper.left + wrapper.width / 2 - finalLeft
    style['--arrow-left'] = `${Math.max(10, Math.min(tooltip.width - 10, arrowLeft))}px`
  }
  
  else if (actualPosition.value === 'bottom') {
    style.top = `${wrapper.bottom + 8}px`
    // 水平居中，但限制偏移以保持箭头指向
    const centerLeft = wrapper.left + wrapper.width / 2 - tooltip.width / 2
    const maxOffset = Math.min(tooltip.width * 0.3, 40)
    const leftBound = Math.max(8, centerLeft - maxOffset)
    const rightBound = Math.min(viewport.width - tooltip.width - 8, centerLeft + maxOffset)
    
    const finalLeft = Math.max(leftBound, Math.min(rightBound, centerLeft))
    style.left = `${finalLeft}px`
    
    // 箭头位置
    const arrowLeft = wrapper.left + wrapper.width / 2 - finalLeft
    style['--arrow-left'] = `${Math.max(10, Math.min(tooltip.width - 10, arrowLeft))}px`
  }
  
  else if (actualPosition.value === 'left') {
    style.left = `${wrapper.left - tooltip.width - 8}px`
    // 垂直居中，但限制偏移以保持箭头指向
    const centerTop = wrapper.top + wrapper.height / 2 - tooltip.height / 2
    const maxOffset = Math.min(tooltip.height * 0.3, 30)
    const topBound = Math.max(8, centerTop - maxOffset)
    const bottomBound = Math.min(viewport.height - tooltip.height - 8, centerTop + maxOffset)
    
    const finalTop = Math.max(topBound, Math.min(bottomBound, centerTop))
    style.top = `${finalTop}px`
    
    // 箭头位置
    const arrowTop = wrapper.top + wrapper.height / 2 - finalTop
    style['--arrow-top'] = `${Math.max(10, Math.min(tooltip.height - 10, arrowTop))}px`
  }
  
  else if (actualPosition.value === 'right') {
    style.left = `${wrapper.right + 8}px`
    // 垂直居中，但限制偏移以保持箭头指向
    const centerTop = wrapper.top + wrapper.height / 2 - tooltip.height / 2
    const maxOffset = Math.min(tooltip.height * 0.3, 30)
    const topBound = Math.max(8, centerTop - maxOffset)
    const bottomBound = Math.min(viewport.height - tooltip.height - 8, centerTop + maxOffset)
    
    const finalTop = Math.max(topBound, Math.min(bottomBound, centerTop))
    style.top = `${finalTop}px`
    
    // 箭头位置
    const arrowTop = wrapper.top + wrapper.height / 2 - finalTop
    style['--arrow-top'] = `${Math.max(10, Math.min(tooltip.height - 10, arrowTop))}px`
  }

  return style
})

const showTooltip = async () => {
  if (timeoutId) {
    clearTimeout(timeoutId)
  }
  timeoutId = window.setTimeout(async () => {
    visible.value = true
    await nextTick()
    // 计算最佳位置
    actualPosition.value = calculateBestPosition()
  }, props.delay)
}

const hideTooltip = () => {
  if (timeoutId) {
    clearTimeout(timeoutId)
    timeoutId = null
  }
  visible.value = false
  actualPosition.value = props.position // 重置为默认位置
}
</script>

<style scoped>
.tooltip-wrapper {
  position: relative;
  display: inline-block;
}

.tooltip {
  position: fixed; /* 改为fixed定位 */
  background: rgba(0, 0, 0, 0.9);
  color: white;
  padding: 8px 12px;
  border-radius: 6px;
  font-size: 12px;
  z-index: 9999; /* 提高层级以避免被遮挡 */
  pointer-events: none;
  max-width: 220px;
  word-wrap: break-word;
  white-space: normal;
  line-height: 1.4;
  box-sizing: border-box;
  hyphens: auto;
  overflow-wrap: break-word;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3); /* 添加阴影提升视觉层次 */
}

/* 移除位置相关的样式，因为现在由JavaScript动态计算 */
.tooltip.top::after {
  content: '';
  position: absolute;
  top: 100%;
  left: var(--arrow-left, 50%);
  transform: translateX(-50%);
  border: 5px solid transparent;
  border-top-color: rgba(0, 0, 0, 0.9);
}

.tooltip.bottom::after {
  content: '';
  position: absolute;
  bottom: 100%;
  left: var(--arrow-left, 50%);
  transform: translateX(-50%);
  border: 5px solid transparent;
  border-bottom-color: rgba(0, 0, 0, 0.9);
}

.tooltip.left::after {
  content: '';
  position: absolute;
  left: 100%;
  top: var(--arrow-top, 50%);
  transform: translateY(-50%);
  border: 5px solid transparent;
  border-left-color: rgba(0, 0, 0, 0.9);
}

.tooltip.right::after {
  content: '';
  position: absolute;
  right: 100%;
  top: var(--arrow-top, 50%);
  transform: translateY(-50%);
  border: 5px solid transparent;
  border-right-color: rgba(0, 0, 0, 0.9);
}

.tooltip-enter-active,
.tooltip-leave-active {
  transition: opacity 0.2s ease;
}

.tooltip-enter-from,
.tooltip-leave-to {
  opacity: 0;
}
</style>
