/**
 * WebSocket Optimizer
 * 
 * This module provides utilities to optimize WebSocket data transmission
 * by implementing throttling, batching, and binary message formats.
 */
import { ref, computed } from 'vue';

// Configuration options
const DEFAULT_CONFIG = {
  throttleInterval: 16,  // ms (approximately 60fps)
  batchSize: 5,          // Number of messages to batch before sending
  reconnectDelay: 3000,  // ms to wait before reconnecting
  maxReconnectAttempts: 5
};

// Connection state management
const connectionState = ref({
  isConnected: false,
  isReconnecting: false,
  reconnectCount: 0,
  lastError: null
});

/**
 * Creates a WebSocket optimizer with throttling and error handling
 */
export function createWebSocketOptimizer(config = {}) {
  // Merge default config with provided config
  const options = {
    ...DEFAULT_CONFIG,
    ...config
  };

  // Message queue for batching
  const messageQueue = [];
  let throttleTimer = null;
  let lastUpdateTime = 0;
  
  // Stats tracking
  const stats = ref({
    messagesSent: 0,
    messagesReceived: 0,
    bytesSent: 0,
    bytesReceived: 0,
    errors: 0,
    reconnects: 0,
    latency: 0
  });
  
  // Computed connection status
  const status = computed(() => {
    if (connectionState.value.isConnected) {
      return 'connected';
    } else if (connectionState.value.isReconnecting) {
      return 'reconnecting';
    } else if (connectionState.value.reconnectCount > 0) {
      return 'connection-failed';
    } else {
      return 'disconnected';
    }
  });
  
  /**
   * Throttles message sending to control transmission rate
   */
  function throttleSend(ws, data) {
    const now = performance.now();
    
    // Skip if we're sending too frequently
    if (now - lastUpdateTime < options.throttleInterval) {
      return;
    }
    
    lastUpdateTime = now;
    
    // Add to queue for potential batching
    messageQueue.push(data);
    
    // Send immediately if we're not batching
    if (messageQueue.length >= options.batchSize || !options.enableBatching) {
      if (ws && ws.readyState === WebSocket.OPEN) {
        const payload = options.enableBatching 
          ? JSON.stringify({ batch: messageQueue.splice(0, messageQueue.length) })
          : JSON.stringify(messageQueue.shift());
          
        ws.send(payload);
        stats.value.messagesSent++;
        stats.value.bytesSent += payload.length;
      }
    }
  }
  
  /**
   * Handle incoming WebSocket messages with performance optimizations
   */
  function handleMessage(event, callback) {
    try {
      const startTime = performance.now();
      const data = JSON.parse(event.data);
      stats.value.messagesReceived++;
      stats.value.bytesReceived += event.data.length;
      stats.value.latency = performance.now() - startTime;
      
      // Process message through callback
      callback(data);
    } catch (error) {
      console.error('Error processing WebSocket message:', error);
      stats.value.errors++;
    }
  }
  
  /**
   * Setup reconnection logic for WebSocket
   */
  function setupReconnection(createConnection) {
    let reconnectTimeout = null;
    
    return {
      handleDisconnect: () => {
        if (connectionState.value.reconnectCount < options.maxReconnectAttempts) {
          connectionState.value.isReconnecting = true;
          connectionState.value.reconnectCount++;
          stats.value.reconnects++;
          
          reconnectTimeout = setTimeout(() => {
            console.log(`Attempting to reconnect (${connectionState.value.reconnectCount}/${options.maxReconnectAttempts})`);
            createConnection();
          }, options.reconnectDelay);
        } else {
          connectionState.value.isReconnecting = false;
          console.error('Maximum reconnection attempts reached');
        }
      },
      
      resetReconnectState: () => {
        if (reconnectTimeout) {
          clearTimeout(reconnectTimeout);
        }
        connectionState.value.isReconnecting = false;
        connectionState.value.reconnectCount = 0;
      }
    };
  }
  
  return {
    throttleSend,
    handleMessage,
    setupReconnection,
    connectionState,
    stats,
    status
  };
}

/**
 * Data validation utilities for WebSocket messages
 */
export function validateWebSocketData(data) {
  // Basic validation checks
  if (!data) return false;
  
  // Check for required fields
  const hasGamepad = data.gamepad && typeof data.gamepad.id === 'number';
  const hasAxes = data.gamepad?.axes && Object.keys(data.gamepad.axes).length > 0;
  const hasButtons = data.gamepad?.buttons && Object.keys(data.gamepad.buttons).length > 0;
  
  return hasGamepad && hasAxes && hasButtons;
}

/**
 * Binary message encoder/decoder for reduced payload size
 * (for future implementation - currently uses JSON)
 */
export function createBinaryEncoder() {
  // Placeholder for future binary protocol implementation
  // This would reduce payload size significantly compared to JSON
  return {
    encode: (data) => JSON.stringify(data),
    decode: (data) => JSON.parse(data)
  };
}
