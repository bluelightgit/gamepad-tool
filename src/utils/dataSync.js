/**
 * 数据同步工具 - 处理WebSocket和Events数据同步问题
 * 增强版 - 支持多数据源冲突管理、优先级排序和性能优化
 */

import { ref, reactive } from 'vue';

// 优先级常量
const PRIORITY = {
  HIGHEST: 100,
  HIGH: 75,
  MEDIUM: 50,
  LOW: 25,
  LOWEST: 0
};

// 数据源配置
const DEFAULT_SOURCE_CONFIG = {
  'websocket': { priority: PRIORITY.HIGH, throttle: 8 },    // WebSocket更新 (~120fps)
  'events': { priority: PRIORITY.MEDIUM, throttle: 16 },    // 事件更新 (~60fps)
  'animation': { priority: PRIORITY.HIGHEST, throttle: 0 }, // 动画渲染 (无节流)
  'user': { priority: PRIORITY.HIGHEST, throttle: 0 },      // 用户操作 (无节流)
  'other': { priority: PRIORITY.LOW, throttle: 33 }         // 其他来源 (~30fps)
};

/**
 * 创建一个增强型数据同步器，协调多个数据源更新同一状态
 * @param {Object} options - 同步器配置选项
 * @returns {Object} - 同步器实例
 */
export function createDataSynchronizer(options = {}) {
  // 合并默认配置和自定义配置
  const config = {
    defaultThrottle: 16,
    lockTimeout: 100,
    sources: { ...DEFAULT_SOURCE_CONFIG, ...options.sources },
    debugMode: options.debugMode || false
  };
  
  // 状态管理
  const state = reactive({
    lastSource: null,
    lastTimestamp: 0,
    syncLock: false,
    updateCount: {}, // 记录各数据源更新次数
    conflicts: 0,    // 记录冲突次数
    skipped: 0       // 跳过的更新次数
  });
  
  // 节流定时器
  const throttleTimers = new Map();
  
  // 日志函数
  const log = (message, level = 'debug') => {
    if (!config.debugMode) return;
    
    switch (level) {
      case 'error': console.error(`[DataSync] ${message}`); break;
      case 'warn': console.warn(`[DataSync] ${message}`); break;
      case 'info': console.info(`[DataSync] ${message}`); break;
      default: console.debug(`[DataSync] ${message}`);
    }
  };
  
  // 获取当前的时间戳
  const now = () => performance.now();
  
  /**
   * 尝试获取更新锁
   * @param {string} source - 数据源名称
   * @param {number} timeout - 锁定时间
   * @returns {boolean} - 是否成功获取锁
   */
  const acquireLock = (source, timeout = config.lockTimeout) => {
    if (state.syncLock) {
      log(`Failed to acquire lock for ${source}, already locked by ${state.lastSource}`);
      return false;
    }
    
    state.syncLock = true;
    setTimeout(() => {
      state.syncLock = false;
    }, timeout);
    
    log(`Lock acquired for ${source} for ${timeout}ms`);
    return true;
  };
  
  /**
   * 检查是否可以从特定数据源更新
   * @param {string} source - 数据源名称 
   * @param {number} additionalPriority - 附加优先级
   * @returns {boolean} - 是否允许更新
   */
  const canUpdateFrom = (source, additionalPriority = 0) => {
    // 验证数据源
    const sourceConfig = config.sources[source] || config.sources.other;
    const throttleDelay = sourceConfig.throttle || config.defaultThrottle;
    const currentTime = now();
    
    // 更新统计数据
    if (!state.updateCount[source]) {
      state.updateCount[source] = 0;
    }
    
    // 如果当前有锁，不允许更新
    if (state.syncLock) {
      state.skipped++;
      log(`Update from ${source} skipped (locked)`);
      return false;
    }
    
    // 如果是第一次更新，允许
    if (!state.lastSource) {
      log(`First update from ${source} allowed`);
      return true;
    }
    
    // 如果是相同的数据源，检查节流
    if (source === state.lastSource) {
      const shouldThrottle = currentTime - state.lastTimestamp < throttleDelay;
      
      if (shouldThrottle) {
        state.skipped++;
        log(`Update from ${source} throttled`);
      }
      
      return !shouldThrottle;
    }
    
    // 计算当前数据源的实际优先级
    const sourcePriority = sourceConfig.priority + additionalPriority;
    const lastSourceConfig = config.sources[state.lastSource] || config.sources.other;
    const lastSourcePriority = lastSourceConfig.priority;
    
    // 优先级规则: 高优先级可以中断低优先级的数据源
    if (sourcePriority > lastSourcePriority) {
      log(`${source} (${sourcePriority}) interrupts ${state.lastSource} (${lastSourcePriority})`);
      return true;
    }
    
    // 如果优先级相同，检查是否过了节流时间
    if (sourcePriority === lastSourcePriority) {
      return currentTime - state.lastTimestamp > throttleDelay;
    }
    
    // 低优先级数据源不能中断高优先级数据源
    state.conflicts++;
    log(`${source} update rejected (lower priority than ${state.lastSource})`, 'warn');
    return false;
  };
  
  /**
   * 记录更新
   * @param {string} source - 数据源名称
   */
  const recordUpdate = (source) => {
    state.lastSource = source;
    state.lastTimestamp = now();
    
    // 更新统计数据
    if (!state.updateCount[source]) {
      state.updateCount[source] = 0;
    }
    state.updateCount[source]++;
    
    log(`Update recorded from ${source}`);
  };
  
  /**
   * 执行节流更新
   * @param {Function} callback - 更新回调函数
   * @param {string} source - 数据源名称
   */
  const throttledUpdate = (callback, source) => {
    const sourceConfig = config.sources[source] || config.sources.other;
    const delay = sourceConfig.throttle || config.defaultThrottle;
    
    // 每个数据源有自己的节流计时器
    if (throttleTimers.has(source)) {
      return;
    }
    
    const timerId = setTimeout(() => {
      callback();
      throttleTimers.delete(source);
      recordUpdate(source);
    }, delay);
    
    throttleTimers.set(source, timerId);
  };
  
  /**
   * 获取统计信息
   * @returns {Object} - 统计数据
   */
  const getStats = () => ({
    updateCounts: { ...state.updateCount },
    conflicts: state.conflicts,
    skipped: state.skipped,
    lastSource: state.lastSource,
    timeSinceLastUpdate: now() - state.lastTimestamp
  });
  
  /**
   * 重置统计信息
   */
  const resetStats = () => {
    state.updateCount = {};
    state.conflicts = 0;
    state.skipped = 0;
  };
  
  // 返回数据同步器接口
  return {
    canUpdateFrom,
    acquireLock,
    recordUpdate,
    throttledUpdate,
    getStats,
    resetStats,
    priorities: PRIORITY
  };
}
