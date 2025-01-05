<template>
    <div class="gamepad-status">
      <h2>{{ title }}</h2>
      <div class="status-grid">
        <div v-for="(value, key) in buttonStates" :key="key">
          {{ key }}: {{ value.toFixed(2) }}
        </div>
        <div v-for="(axis, index) in axisStates" :key="index">
          AXIS {{ index }}: {{ axis.toFixed(5) }}
        </div>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  // Props 输入接口
  defineProps<{
    title: string;
    buttonStates: Record<string, number>;
    axisStates: number[];
  }>();
  </script>
  
  <style scoped>
  .gamepad-status {
    border: 1px solid #ddd;
    padding: 10px;
    border-radius: 6px;
  }
  .status-grid {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
  }
  </style>
  