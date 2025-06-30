<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'
import GamepadTestPage from './views/RefactoredGamepadTestPage.vue'

// 响应式状态
const showHeader = ref(false)
const showFooter = ref(false)
const lastScrollY = ref(0)
const isAtTop = ref(true)
const isAtBottom = ref(false)
const canShowHeader = ref(false) // 标记是否可以显示header
const canShowFooter = ref(false) // 标记是否可以显示footer

// 滚动处理函数
const handleScroll = () => {
  const currentScrollY = window.scrollY
  const windowHeight = window.innerHeight
  const documentHeight = document.documentElement.scrollHeight
  const scrollDirection = currentScrollY > lastScrollY.value ? 'down' : 'up'
  
  // 检查是否有滚动条
  const hasScrollbar = documentHeight > windowHeight
  
  // 更新位置状态
  const newIsAtTop = currentScrollY <= 5
  const newIsAtBottom = hasScrollbar && (currentScrollY + windowHeight >= documentHeight - 5)
  
  // Header逻辑：只有在到达顶部后再次向上滚动才显示
  if (newIsAtTop) {
    if (!isAtTop.value) {
      // 刚刚到达顶部，设置可以显示header的标记
      canShowHeader.value = true
    }
  } else {
    // 离开顶部时，如果正在显示header且向下滚动，则隐藏
    if (showHeader.value && scrollDirection === 'down') {
      showHeader.value = false
    }
    canShowHeader.value = false
  }
  
  // Footer逻辑：只有在到达底部后再次向下滚动才显示
  if (newIsAtBottom) {
    if (!isAtBottom.value) {
      // 刚刚到达底部，设置可以显示footer的标记
      canShowFooter.value = true
    }
  } else {
    // 离开底部时，如果正在显示footer且向上滚动，则隐藏
    if (showFooter.value && scrollDirection === 'up') {
      showFooter.value = false
    }
    canShowFooter.value = false
  }
  
  // 更新状态
  isAtTop.value = newIsAtTop
  isAtBottom.value = newIsAtBottom
  lastScrollY.value = currentScrollY
}

// 节流函数
let ticking = false
const throttledHandleScroll = () => {
  if (!ticking) {
    requestAnimationFrame(() => {
      handleScroll()
      ticking = false
    })
    ticking = true
  }
}

// 处理滚轮事件（用于检测滚动尝试）
const handleWheel = (event: WheelEvent) => {
  const currentScrollY = window.scrollY
  const windowHeight = window.innerHeight
  const documentHeight = document.documentElement.scrollHeight
  const hasScrollbar = documentHeight > windowHeight
  
  // 如果没有滚动条，直接根据滚轮方向判断
  if (!hasScrollbar) {
    if (event.deltaY < 0) {
      // 向上滚动
      if (showFooter.value) {
        // 优先隐藏footer，不触发header显示
        showFooter.value = false
        canShowFooter.value = true
      } else if (!showHeader.value) {
        // 只有在footer没有显示时才显示header
        showHeader.value = true
        canShowHeader.value = false
        // 重置footer状态，允许下次显示
        canShowFooter.value = true
      }
    } else if (event.deltaY > 0) {
      // 向下滚动
      if (showHeader.value) {
        // 优先隐藏header，不触发footer显示
        showHeader.value = false
        canShowHeader.value = true
      } else if (!showFooter.value) {
        // 只有在header没有显示时才显示footer
        showFooter.value = true
        canShowFooter.value = false
        // 重置header状态，允许下次显示
        canShowHeader.value = true
      }
    }
    return
  }
  
  // 有滚动条的情况下，检查边界尝试
  const isAtTop = currentScrollY <= 5
  const isAtBottom = currentScrollY + windowHeight >= documentHeight - 5
  
  if (isAtTop && event.deltaY < 0 && canShowHeader.value && !showHeader.value) {
    // 在顶部尝试向上滚动
    showHeader.value = true
    canShowHeader.value = false
  } else if (isAtBottom && event.deltaY > 0 && canShowFooter.value && !showFooter.value) {
    // 在底部尝试向下滚动
    showFooter.value = true
    canShowFooter.value = false
  }
}

// 生命周期钩子
onMounted(() => {
  window.addEventListener('scroll', throttledHandleScroll)
  window.addEventListener('wheel', handleWheel, { passive: false })
  
  // 初始化状态
  setTimeout(() => {
    const windowHeight = window.innerHeight
    const documentHeight = document.documentElement.scrollHeight
    const hasScrollbar = documentHeight > windowHeight
    
    if (!hasScrollbar) {
      // 没有滚动条时，设置可以显示header和footer的标记
      canShowHeader.value = true
      canShowFooter.value = true
    } else {
      // 有滚动条时，正常初始化
      handleScroll()
    }
  }, 100) // 延迟一点确保DOM完全渲染
})

onBeforeUnmount(() => {
  window.removeEventListener('scroll', throttledHandleScroll)
  window.removeEventListener('wheel', handleWheel)
})
</script>

<template>
  <div id="app-container">
    <!-- 动态显示的Header -->
    <Transition name="header-slide">
      <header v-show="showHeader" class="app-header">
        <h1 class="app-title">Gamepad Tool</h1>
        <p class="app-subtitle">Real-time Gamepad Monitoring & Testing</p>
      </header>
    </Transition>
    
    <main class="main-content">
      <GamepadTestPage />
    </main>
    
    <!-- 动态显示的Footer -->
    <Transition name="footer-slide">
      <footer v-show="showFooter" class="app-footer">
        <p>Built with Tauri 2.0 & Vue 3</p>
      </footer>
    </Transition>
  </div>
</template>

<style>
/* Global Styles */
:root {
  --primary-color: #42b983;
  --background-color: #f0f2f5;
  --container-bg-color: #ffffff;
  --text-color: #333;
  --border-color: #e0e0e0;
  --shadow-color: rgba(0, 0, 0, 0.1);
}

body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  background-color: var(--background-color);
  color: var(--text-color);
}

#app-container {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  position: relative;
}

.app-header {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1000;
  padding: 24px;
  background: var(--primary-color);
  color: white;
  text-align: center;
  box-shadow: 0 2px 8px var(--shadow-color);
}

.app-title {
  margin: 0;
  font-size: 2em;
  font-weight: 700;
}

.app-subtitle {
  margin: 4px 0 0;
  font-size: 1em;
  opacity: 0.9;
}

.main-content {
  flex: 1;
  padding: 24px;
  min-width: 0; /* 防止内容溢出 */
  box-sizing: border-box;
}

.app-footer {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  z-index: 1000;
  padding: 16px;
  text-align: center;
  font-size: 0.9em;
  color: #666;
  background: #f8f9fa;
  border-top: 1px solid var(--border-color);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .app-header {
    padding: 16px 12px;
  }
  
  .app-title {
    font-size: 1.6em;
  }
  
  .app-subtitle {
    font-size: 0.9em;
  }
  
  .main-content {
    padding: 16px 12px;
  }
  
  .app-footer {
    padding: 12px;
    font-size: 0.8em;
  }
}

/* Header和Footer的过渡动画 */
.header-slide-enter-active,
.header-slide-leave-active {
  transition: transform 0.3s ease-out;
}

.header-slide-enter-from {
  transform: translateY(-100%);
}

.header-slide-leave-to {
  transform: translateY(-100%);
}

.footer-slide-enter-active,
.footer-slide-leave-active {
  transition: transform 0.3s ease-out;
}

.footer-slide-enter-from {
  transform: translateY(100%);
}

.footer-slide-leave-to {
  transform: translateY(100%);
}
</style>