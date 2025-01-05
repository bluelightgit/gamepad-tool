<template>
  <div class="joystick-container">
    <svg width="120" height="120">
      <rect x="5" y="5" width="110" height="110" stroke="#ddd" stroke-width="2" fill="none" />
      <line x1="60" y1="5" x2="60" y2="115" stroke="#ddd" stroke-width="2" />
      <line x1="5" y1="60" x2="115" y2="60" stroke="#ddd" stroke-width="2" />
      <circle cx="60" cy="60" r="55" stroke="#ddd" stroke-width="2" fill="none" />
      <line
        :x1="60"
        :y1="60"
        :x2="60 + axisX * 52.5"
        :y2="60 - axisY * 52.5"
        stroke="gray"
        stroke-width="1"
      />
      <circle
        :cx="60 + axisX * 52.5"
        :cy="60 - axisY * 52.5"
        r="5"
        fill="gray"
      />
    </svg>
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
  display: inline-block;
  margin: 10px;
}
</style>
