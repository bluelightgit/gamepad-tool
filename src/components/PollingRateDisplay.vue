<template>
  <div class="polling-rate-display">
    <div v-if="pollingRateData" class="stats-grid">
      <Tooltip :text="t('tooltips.avgRate')" position="top">
        <div class="stat-item primary">
          <span class="stat-label">Avg Rate</span>
          <span class="stat-value">{{ formatNumber(pollingRateData.polling_rate_avg) }} Hz</span>
        </div>
      </Tooltip>
      
      <Tooltip :text="t('tooltips.minRate')" position="top">
        <div class="stat-item">
          <span class="stat-label">Min Rate</span>
          <span class="stat-value">{{ formatNumber(pollingRateData.polling_rate_min) }} Hz</span>
        </div>
      </Tooltip>
      
      <Tooltip :text="t('tooltips.maxRate')" position="top">
        <div class="stat-item">
          <span class="stat-label">Max Rate</span>
          <span class="stat-value">{{ formatNumber(pollingRateData.polling_rate_max) }} Hz</span>
        </div>
      </Tooltip>
      
      <Tooltip :text="t('tooltips.avgInterval')" position="top">
        <div class="stat-item">
          <span class="stat-label">Avg Interval</span>
          <span class="stat-value">{{ formatNumber(pollingRateData.avg_interval) }} ms</span>
        </div>
      </Tooltip>
      
      <Tooltip :text="t('tooltips.errorL')" position="top">
        <div class="stat-item">
          <span class="stat-label">Error (L)</span>
          <span class="stat-value">{{ formatNumber(pollingRateData.avg_error_l * 100) }}%</span>
        </div>
      </Tooltip>
      
      <Tooltip :text="t('tooltips.errorR')" position="top">
        <div class="stat-item">
          <span class="stat-label">Error (R)</span>
          <span class="stat-value">{{ formatNumber(pollingRateData.avg_error_r * 100) }}%</span>
        </div>
      </Tooltip>
    </div>
    
    <div v-else class="no-data">
      <p>No performance data available</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { PollingRateResult } from '../composables/useGamepadState'
import Tooltip from './Tooltip.vue'
import { useI18n } from '../i18n'

interface Props {
  pollingRateData: PollingRateResult | null
}

defineProps<Props>()

const { t } = useI18n()

const formatNumber = (value: number): string => {
  return value.toFixed(2)
}
</script>

<style scoped>
.polling-rate-display {
  width: 100%;
  min-width: 200px; /* 减小基础最小宽度 */
  height: 100%;
  display: flex;
  flex-direction: column;
}

.polling-rate-display h3 {
  margin: 0 0 12px 0; /* 减少底部间距 */
  color: #333;
  font-size: 1em; /* 减小基础字体 */
  font-weight: 600;
  flex-shrink: 0;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
  width: 100%;
  flex: 1; /* 占用剩余空间 */
  align-content: start;
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

/* 响应式设计 - 与主布局断点一致 */

/* 宽屏及以上 (1000px+) */
@media (min-width: 1000px) {
  .polling-rate-display {
    min-width: 100px; /* 由于performance列更窄，减小最小宽度 */
  }
  
  .polling-rate-display h3 {
    font-size: 1.1em;
    margin: 0 0 14px 0;
  }
  
  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
    gap: 12px;
  }
  
  .stat-item {
    padding: 12px;
  }
  
  .stat-label {
    font-size: 0.85em;
  }
  
  .stat-value {
    font-size: 1.1em;
  }
  
  .stat-item.primary .stat-value {
    font-size: 1.2em;
  }
}

/* 中屏：紧凑3列布局 (768-999px) */
@media (min-width: 768px) and (max-width: 999px) {
  .polling-rate-display {
    min-width: 200px;
  }
  
  .polling-rate-display h3 {
    font-size: 0.95em;
    margin: 0 0 10px 0;
  }
  
  .stats-grid {
    grid-template-columns: repeat(3, 1fr);
    gap: 12px;
  }
  
  .stat-item {
    padding: 12px;
  }
  
  .stat-label {
    font-size: 0.85em;
  }
  
  .stat-value {
    font-size: 1.1em;
  }
  
  .stat-item.primary .stat-value {
    font-size: 1.2em;
  }
}

/* 小屏：2列布局 (480-767px) */
@media (min-width: 480px) and (max-width: 767px) {
  .polling-rate-display {
    min-width: 180px;
  }
  
  .polling-rate-display h3 {
    font-size: 0.9em;
    margin: 0 0 8px 0;
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
    font-size: 0.75em;
  }
  
  .stat-value {
    font-size: 0.85em;
  }
  
  .stat-item.primary .stat-value {
    font-size: 0.9em;
  }
}

</style>
