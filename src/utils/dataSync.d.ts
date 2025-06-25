/**
 * Type definitions for dataSync.js
 */

export interface DataSyncOptions {
  debugMode?: boolean;
  sources?: {
    [key: string]: {
      priority: number;
      throttle: number;
    }
  };
}

export interface DataSynchronizer {
  canUpdateFrom: (source: string, additionalPriority?: number) => boolean;
  acquireLock: (source: string, timeout?: number) => boolean;
  recordUpdate: (source: string) => void;
  throttledUpdate: (callback: Function, source: string) => void;
  getStats: () => any;
  resetStats: () => void;
  priorities: {
    HIGHEST: number;
    HIGH: number;
    MEDIUM: number;
    LOW: number;
    LOWEST: number;
  };
}

export function createDataSynchronizer(options?: DataSyncOptions): DataSynchronizer;
