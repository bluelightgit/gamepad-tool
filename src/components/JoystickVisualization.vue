<template>
  <div class="joystick-container">
    <div class="joystick-area">
      <svg width="180" height="180">
        <rect x="5" y="5" width="170" height="170" stroke="#ddd" stroke-width="2" fill="none" />
        <line x1="90" y1="5" x2="90" y2="175" stroke="#ddd" stroke-width="2" />
        <line x1="5" y1="90" x2="175" y2="90" stroke="#ddd" stroke-width="2" />
        <circle cx="90" cy="90" r="85" stroke="#ddd" stroke-width="2" fill="none" />
        <line
            :x1="90"
            :y1="90"
            :x2="90 + axisX * 85"
            :y2="90 - axisY * 85"
            stroke="gray"
            stroke-width="1"
        />
        <circle
            :cx="90 + axisX * 85"
            :cy="90 - axisY * 85"
            r="5"
            fill="gray"
        />
      </svg>
    </div>

    <div class="x-axis-bar">
      <div class="bar-background"></div>
      <div
          class="bar-fill"
          :style="xAxisStyle"
      ></div>
      <span class="bar-value">{{ formatAxisValue(axisX) }}</span>
    </div>
    <div class="x-axis-bar">
      <div class="bar-background"></div>
      <div
          class="bar-fill"
          :style="yAxisStyle"
      ></div>
      <span class="bar-value">{{ formatAxisValue(axisY) }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  axisX: number;
  axisY: number;
}>();

const yAxisStyle = computed(() => ({
  width: `${Math.abs(props.axisY) * 50}%`,
  left: props.axisY > 0 ? '50%' : `${50 - Math.abs(props.axisY) * 50}%`,
  backgroundColor: '#42b983',
}));

const xAxisStyle = computed(() => ({
  width: `${Math.abs(props.axisX) * 50}%`,
  left: props.axisX > 0 ? '50%' : `${50 - Math.abs(props.axisX) * 50}%`,
  backgroundColor: '#42b983',
}));

const formatAxisValue = (value: number) => value.toFixed(6);
</script>

<style scoped>
.joystick-container {
  display: flex;
  flex-direction: column; /* Change to column to stack elements vertically */
  align-items: center;
  gap: 20px;
  margin: 1px;
}

.x-axis-bar {
  width: 170px;
  height: 20px;
  position: relative;
  background-color: #f0f0f0;
  border: 1px solid #ddd;
  border-radius: 10px;
  overflow: hidden;
  margin-top: 2px;
}

.bar-background {
  position: absolute;
  width: 50%;
  height: 90%;
  background-color: #f0f0f0;
}

.bar-fill {
  position: absolute;
  //transition: all 0.01s ease;
}

.y-axis-bar .bar-fill {
  width: 100%;
  left: 0;
}

.x-axis-bar .bar-fill {
  height: 100%;
  top: 0;
}

.bar-value {
  position: absolute;
  font-size: 14px;
  color: white;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.908);
  z-index: 1;
}

.y-axis-bar .bar-value {
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%) rotate(-90deg);
  white-space: nowrap;
}

.x-axis-bar .bar-value {
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  white-space: nowrap;
}

.joystick-area {
  display: flex;
  justify-content: center;
  align-items: center;
}

/* Adjust layout */
.joystick-container {
  flex-direction: column; /* Change to column to stack elements vertically */
  align-items: center;
}

.joystick-area {
  order: 2;
}

.y-axis-bar {
  order: 1;
}

.x-axis-bar {
  order: 3;
  margin-left: 0; /* Remove left margin */
}
</style>