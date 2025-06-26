/**
 * 高性能渲染管理器
 * 使用 requestAnimationFrame 优化渲染性能
 */
import { ref, onMounted, onBeforeUnmount } from 'vue'

export interface RenderStats {
  fps: number
  frameTime: number
  lastUpdateTime: number
  frameCount: number
}

/**
 * 渲染管理器 Hook
 */
export function useRenderManager() {
  const isRenderingActive = ref(false)
  const renderStats = ref<RenderStats>({
    fps: 0,
    frameTime: 0,
    lastUpdateTime: 0,
    frameCount: 0
  })
  
  // 渲染回调列表
  const renderCallbacks = new Set<() => void>()
  
  // 性能监控变量
  let animationFrameId: number | null = null
  let lastFrameTime = 0
  let frameCount = 0
  let fpsLastTime = 0
  
  // 主渲染循环
  const renderLoop = (currentTime: number) => {
    if (!isRenderingActive.value) return
    
    // 计算帧时间
    const deltaTime = currentTime - lastFrameTime
    lastFrameTime = currentTime
    
    // 更新统计信息
    frameCount++
    if (currentTime - fpsLastTime >= 1000) {
      renderStats.value = {
        fps: Math.round((frameCount * 1000) / (currentTime - fpsLastTime)),
        frameTime: deltaTime,
        lastUpdateTime: currentTime,
        frameCount
      }
      frameCount = 0
      fpsLastTime = currentTime
    }
    
    // 执行所有渲染回调
    renderCallbacks.forEach(callback => {
      try {
        callback()
      } catch (error) {
        console.error("Error in render callback:", error)
      }
    })
    
    // 继续下一帧
    animationFrameId = requestAnimationFrame(renderLoop)
  }
  
  // 开始渲染循环
  const startRendering = () => {
    if (isRenderingActive.value) return
    
    isRenderingActive.value = true
    lastFrameTime = performance.now()
    fpsLastTime = lastFrameTime
    frameCount = 0
    animationFrameId = requestAnimationFrame(renderLoop)
    
    console.log("Render manager started")
  }
  
  // 停止渲染循环
  const stopRendering = () => {
    if (!isRenderingActive.value) return
    
    isRenderingActive.value = false
    if (animationFrameId !== null) {
      cancelAnimationFrame(animationFrameId)
      animationFrameId = null
    }
    
    console.log("Render manager stopped")
  }
  
  // 注册渲染回调
  const registerRenderCallback = (callback: () => void): (() => void) => {
    renderCallbacks.add(callback)
    
    // 返回取消注册函数
    return () => {
      renderCallbacks.delete(callback)
    }
  }
  
  // 创建节流渲染回调（用于降低某些回调的执行频率）
  const createThrottledRenderCallback = (
    callback: () => void,
    interval: number
  ): (() => void) => {
    let lastCall = 0
    
    const throttledCallback = () => {
      const now = performance.now()
      if (now - lastCall >= interval) {
        lastCall = now
        callback()
      }
    }
    
    return registerRenderCallback(throttledCallback)
  }
  
  // 批量执行回调的工具函数
  const batchRenderUpdates = (updates: (() => void)[]): void => {
    const batchCallback = () => {
      updates.forEach(update => {
        try {
          update()
        } catch (error) {
          console.error("Error in batch update:", error)
        }
      })
    }
    
    // 在下一帧执行批量更新
    requestAnimationFrame(batchCallback)
  }
  
  // 清理函数
  const cleanupRenderManager = () => {
    stopRendering()
    renderCallbacks.clear()
    console.log("Render manager cleaned up")
  }

  // 自动管理生命周期
  onMounted(() => {
    startRendering()
  })
  
  onBeforeUnmount(cleanupRenderManager)
  
  return {
    // 状态
    isRenderingActive,
    renderStats,
    
    // 方法
    startRendering,
    stopRendering,
    registerRenderCallback,
    createThrottledRenderCallback,
    batchRenderUpdates,
    cleanup: cleanupRenderManager
  }
}
