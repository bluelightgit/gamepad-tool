// /**
//  * Type definitions for websocketOptimizer.js
//  */

// export interface WebSocketOptimizerConfig {
//   throttleInterval?: number;
//   batchSize?: number;
//   reconnectDelay?: number;
//   enableBatching?: boolean;
//   maxReconnectAttempts?: number;
// }

// export interface WebSocketOptimizerStats {
//   messagesSent: number;
//   messagesReceived: number;
//   bytesSent: number;
//   bytesReceived: number;
//   errors: number;
//   reconnects: number;
//   latency: number;
// }

// export interface ConnectionState {
//   isConnected: boolean;
//   isReconnecting: boolean;
//   reconnectCount: number;
//   lastError: any;
// }

// export interface WebSocketOptimizer {
//   throttleSend: (ws: WebSocket, data: any) => void;
//   handleMessage: (event: MessageEvent, callback: (data: any) => void) => void;
//   setupReconnection: (createConnection: () => void) => {
//     handleDisconnect: () => void;
//     resetReconnectState: () => void;
//   };
//   connectionState: any; // Vue ref
//   stats: any; // Vue ref
//   status: any; // Vue computed
// }

// export function createWebSocketOptimizer(config?: WebSocketOptimizerConfig): WebSocketOptimizer;
// export function validateWebSocketData(data: any): boolean;
// export function createBinaryEncoder(): {
//   encode: (data: any) => string;
//   decode: (data: string) => any;
// };
