// WebSocket客户端代码 - 优化版本
import { ref, onMounted, onBeforeUnmount, shallowRef } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { createWebSocketOptimizer, validateWebSocketData } from './websocketOptimizer';

// WebSocket状态
const wsStatus = ref('disconnected');
const wsConnection = ref(null);

// 使用shallowRef优化大数据对象的性能
const gamepadData = shallowRef(null);
const pollingRateLogData = shallowRef([]);
const pollingRateResultData = shallowRef(null);

// 性能监控数据
const performanceStats = ref({
  avgProcessingTime: 0,
  messagesPerSecond: 0,
  lastMessageTime: 0,
  messageCount: 0,
  startTime: 0
});

// WebSocket优化器
const wsOptimizer = createWebSocketOptimizer({
  throttleInterval: 8,  // 约120fps
  enableBatching: false // 目前不启用批处理
});

// 初始化WebSocket连接
export async function initWebSocket() {
  try {
    // 记录开始时间用于性能监控
    performanceStats.value.startTime = performance.now();
    performanceStats.value.messageCount = 0;
    
    // 先获取WebSocket服务器端口
    await listen('websocket_port', async (event) => {
      const port = event.payload;
      console.log(`WebSocket server port received: ${port}`);
      connectWebSocket(port);
    });
    
    // 启动WebSocket服务器
    await invoke('start_websocket_server_command');
  } catch (error) {
    console.error('Failed to initialize WebSocket:', error);
    wsStatus.value = 'error';
  }
}

// 连接到WebSocket服务器
function connectWebSocket(port) {
  // 重置性能监控
  performanceStats.value.messageCount = 0;
  performanceStats.value.startTime = performance.now();
  
  const reconnector = wsOptimizer.setupReconnection(() => connectWebSocket(port));
  
  try {
    const ws = new WebSocket(`ws://localhost:${port}`);
    wsConnection.value = ws;
    
    ws.onopen = () => {
      console.log('WebSocket connected');
      wsStatus.value = 'connected';
      reconnector.resetReconnectState();
      wsOptimizer.connectionState.value.isConnected = true;
    };
    
    ws.onmessage = (event) => {
      const startTime = performance.now();
      
      wsOptimizer.handleMessage(event, (data) => {
        // 验证数据完整性
        if (!validateWebSocketData(data)) {
          console.warn('Invalid WebSocket data received');
          return;
        }
        
        // 使用不可变更新模式更新数据
        gamepadData.value = Object.freeze({ ...data.gamepad });
        pollingRateLogData.value = Object.freeze([...data.polling_rate_log]);
        pollingRateResultData.value = Object.freeze({ ...data.polling_rate_result });
        
        // 更新性能统计
        const processingTime = performance.now() - startTime;
        updatePerformanceStats(processingTime);
      });
    };
    
    ws.onerror = (error) => {
      console.error('WebSocket error:', error);
      wsStatus.value = 'error';
      wsOptimizer.connectionState.value.lastError = error;
      wsOptimizer.stats.value.errors++;
    };
    
    ws.onclose = () => {
      console.log('WebSocket connection closed');
      wsStatus.value = 'disconnected';
      wsOptimizer.connectionState.value.isConnected = false;
      
      // 使用优化的重连逻辑
      reconnector.handleDisconnect();
    };
  } catch (error) {
    console.error('Error creating WebSocket connection:', error);
    wsStatus.value = 'error';
    reconnector.handleDisconnect();
  }
}

// 关闭WebSocket连接
export function closeWebSocket() {
  if (wsConnection.value && wsConnection.value.readyState === WebSocket.OPEN) {
    wsConnection.value.close();
  }
}

// 更新性能统计
function updatePerformanceStats(processingTime) {
  const stats = performanceStats.value;
  stats.messageCount++;
  
  // 更新平均处理时间 (使用指数移动平均)
  if (stats.avgProcessingTime === 0) {
    stats.avgProcessingTime = processingTime;
  } else {
    stats.avgProcessingTime = stats.avgProcessingTime * 0.95 + processingTime * 0.05;
  }
  
  // 更新每秒消息数
  const now = performance.now();
  const elapsed = now - stats.startTime;
  
  if (elapsed >= 1000) {
    stats.messagesPerSecond = (stats.messageCount / elapsed) * 1000;
    stats.startTime = now;
    stats.messageCount = 0;
  }
  
  stats.lastMessageTime = now;
}

// 获取WebSocket优化器统计信息
export function getWebSocketStats() {
  return {
    connection: wsOptimizer.connectionState,
    performance: performanceStats.value,
    messages: wsOptimizer.stats
  };
}

// 导出WebSocket数据和状态
export function useWebSocketData() {
  onMounted(() => {
    initWebSocket();
  });
  
  onBeforeUnmount(() => {
    closeWebSocket();
  });
  
  return {
    wsStatus,
    gamepadData,
    pollingRateLogData,
    pollingRateResultData,
    stats: wsOptimizer.stats,
    performanceStats
  };
}
