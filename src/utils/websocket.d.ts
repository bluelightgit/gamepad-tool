// // WebSocket client type definitions
// import { Ref } from 'vue';

// export interface GamepadInfo {
//   id: number;
//   name: string;
//   vendor_id?: number;
//   product_id?: number;
//   guid: string;
//   power_info: string;
//   axes: Record<string, AxisData>;
//   buttons: Record<string, ButtonData>;
// }

// export interface AxisData {
//   axis: string;
//   value: number;
// }

// export interface ButtonData {
//   button: string;
//   is_pressed: boolean;
//   value: number;
// }

// export interface PollingRateResult {
//   polling_rate_avg: number;
//   polling_rate_min: number;
//   polling_rate_max: number;
//   avg_interval: number;
//   drop_rate: number;
//   avg_error_l: number;
//   avg_error_r: number;
// }

// export interface OutputLog {
//   timestamp: number;
//   xyxy: [number, number, number, number];
// }

// export function initWebSocket(): Promise<void>;
// export function closeWebSocket(): void;
// export interface PerformanceStats {
//   avgProcessingTime: number;
//   messagesPerSecond: number;
//   lastMessageTime: number;
//   messageCount: number;
//   startTime: number;
// }

// export function getWebSocketStats(): any;

// export function useWebSocketData(): {
//   wsStatus: Ref<string>;
//   gamepadData: Ref<GamepadInfo | null>;
//   pollingRateLogData: Ref<OutputLog[]>;
//   pollingRateResultData: Ref<PollingRateResult | null>;
//   stats: Ref<any>;
//   performanceStats: Ref<PerformanceStats>;
// };
