<template>
  <div class="joystick-container">
    <div class="y-axis-bar">
      <div class="bar-background"></div>
      <div
        class="bar-fill"
        :style="{
          height: `${Math.abs(axisY) * 50}%`,
          top: axisY > 0 ? `${50 - Math.abs(axisY) * 50}%` : '50%',
          backgroundColor: '#42b983',
        }"
      ></div>
      <span class="bar-value">{{ axisY.toFixed(4) }}</span>
    </div>

    <div class="joystick-area">
      <svg width="180" height="180">
        <rect x="5" y="5" width="170" height="170" stroke="#ddd" stroke-width="2" fill="none" />
        <line x1="90" y1="5" x2="90" y2="175" stroke="#ddd" stroke-width="2" />
        <line x1="5" y1="90" x2="175" y2="90" stroke="#ddd" stroke-width="2" />
        <circle cx="90" cy="90" r="85" stroke="#ddd" stroke-width="2" fill="none" />
        <line
          :x1="90"
          :y1="90"
          :x2="90 + axisX * 80"
          :y2="90 - axisY * 80"
          stroke="gray"
          stroke-width="1"
        />
        <circle
          :cx="90 + axisX * 80"
          :cy="90 - axisY * 80"
          r="5"
          fill="gray"
        />
      </svg>
    </div>

    <!-- X轴数值条 -->
    <div class="x-axis-bar">
      <div class="bar-background"></div>
      <div
        class="bar-fill"
        :style="{
          width: `${Math.abs(axisX) * 50}%`,
          left: axisX > 0 ? '50%' : `${50 - Math.abs(axisX) * 50}%`,
          backgroundColor: '#42b983',
        }"
      ></div>
      <span class="bar-value">{{ axisX.toFixed(2) }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  axisX: number;
  axisY: number;
}>();
</script>

<style scoped>
.joystick-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
  margin: 10px;
}

.y-axis-bar {
  width: 20px;
  height: 170px;
  position: relative;
  background-color: #f0f0f0;
  border: 1px solid #ddd;
  border-radius: 10px;
  overflow: hidden;
}

.x-axis-bar {
  width: 170px;
  height: 20px;
  position: relative;
  background-color: #f0f0f0;
  border: 1px solid #ddd;
  border-radius: 10px;
  overflow: hidden;
}

.bar-background {
  position: absolute;
  width: 50%;
  height: 90%;
  background-color: #f0f0f0;
}

.bar-fill {
  position: absolute;
  transition: all 0.1s ease;
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
</style>