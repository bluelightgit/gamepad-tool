<template>
  <div class="polling-rate-display">
    <h3>Performance Stats</h3>
    <div v-if="pollingRateData" class="stats-grid">
      <div class="stat-item primary">
        <span class="stat-label">Avg Rate</span>
        <span class="stat-value">{{ formatNumber(pollingRateData.polling_rate_avg) }} Hz</span>
      </div>
      
      <div class="stat-item">
        <span class="stat-label">Min Rate</span>
        <span class="stat-value">{{ formatNumber(pollingRateData.polling_rate_min) }} Hz</span>
      </div>
      
      <div class="stat-item">
        <span class="stat-label">Max Rate</span>
        <span class="stat-value">{{ formatNumber(pollingRateData.polling_rate_max) }} Hz</span>
      </div>
      
      <div class="stat-item">
        <span class="stat-label">Avg Interval</span>
        <span class="stat-value">{{ formatNumber(pollingRateData.avg_interval) }} ms</span>
      </div>
      
      <div class="stat-item">
        <span class="stat-label">Error (L)</span>
        <span class="stat-value">{{ formatNumber(pollingRateData.avg_error_l) }}%</span>
      </div>
      
      <div class="stat-item">
        <span class="stat-label">Error (R)</span>
        <span class="stat-value">{{ formatNumber(pollingRateData.avg_error_r) }}%</span>
      </div>
    </div>
    
    <div v-else class="no-data">
      <p>No performance data available</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { PollingRateResult } from '../composables/useGamepadState'

interface Props {
  pollingRateData: PollingRateResult | null
}

defineProps<Props>()

const formatNumber = (value: number): string => {
  return value.toFixed(2)
}
</script>

<style scoped>
.polling-rate-display {
  width: 100%;
  min-width: 280px;
}

.polling-rate-display h3 {
  margin: 0 0 16px 0;
  color: #333;
  font-size: 1.2em;
  font-weight: 600;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
  width: 100%;
}

.stat-item {
  display: flex;
  flex-direction: column;
  padding: 12px;
  background: #f9f9f9;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  transition: all 0.2s ease;
  position: relative;
}

.stat-item:hover {
  background: #f0f0f0;
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.stat-item.primary {
  background: linear-gradient(135deg, #42b983 0%, #369870 100%);
  color: white;
  border-color: #42b983;
}

.stat-item.primary:hover {
  background: linear-gradient(135deg, #369870 0%, #2d7a5f 100%);
}

.stat-label {
  font-size: 0.85em;
  font-weight: 500;
  margin-bottom: 4px;
  opacity: 0.8;
}

.stat-item.primary .stat-label {
  opacity: 0.9;
}

.stat-value {
  font-size: 1.1em;
  font-weight: 700;
  font-family: 'Courier New', monospace;
  color: #333;
}

.stat-item.primary .stat-value {
  color: white;
  font-size: 1.2em;
}

.no-data {
  padding: 24px;
  text-align: center;
  background: #f9f9f9;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  color: #666;
}

.no-data p {
  margin: 0;
  font-style: italic;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
    gap: 10px;
  }
  
  .stat-item {
    padding: 10px;
  }
  
  .stat-label {
    font-size: 0.8em;
  }
  
  .stat-value {
    font-size: 1em;
  }
  
  .stat-item.primary .stat-value {
    font-size: 1.1em;
  }
}

@media (max-width: 600px) {
  .stats-grid {
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
  }
  
  .stat-item {
    padding: 8px;
  }
  
  .stat-label {
    font-size: 0.75em;
  }
  
  .stat-value {
    font-size: 0.9em;
  }
  
  .stat-item.primary .stat-value {
    font-size: 1em;
  }
}

@media (max-width: 480px) {
  .polling-rate-display {
    min-width: 240px;
  }
  
  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
    gap: 6px;
  }
  
  .stat-item {
    padding: 6px;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
  }
  
  .stat-label {
    margin-bottom: 0;
    font-size: 0.7em;
  }
  
  .stat-value {
    font-size: 0.8em;
  }
  
  .stat-item.primary .stat-value {
    font-size: 0.9em;
  }
}

@media (max-width: 360px) {
  .polling-rate-display {
    min-width: 200px;
  }
  
  .stats-grid {
    grid-template-columns: 1fr;
    gap: 4px;
  }
  
  .stat-item {
    padding: 8px;
  }
  
  .stat-label {
    font-size: 0.75em;
  }
  
  .stat-value {
    font-size: 0.85em;
  }
}

@media (max-width: 320px) {
  .polling-rate-display {
    min-width: 180px;
  }
  
  .stat-item {
    padding: 6px;
  }
  
  .stat-label {
    font-size: 0.7em;
  }
  
  .stat-value {
    font-size: 0.8em;
  }
}
</style>
