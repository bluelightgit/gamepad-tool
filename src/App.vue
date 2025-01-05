<script setup lang="ts">
import { ref } from "vue";
import SysInfoPage from "./views/SysInfo.vue";
import GamepadTestPage from "./views/GamepadTestPage.vue";

const currentPage = ref<string | null>(null);
</script>

<template>
  <div class="container">
    <div class="tabs">
      <button class="tab-button" @click="currentPage = 'gamepadTest'">
        Gamepad Test
      </button>
    </div>
    <GamepadTestPage v-if="currentPage === 'gamepadTest'" />
  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  align-items: flex-start; /* 将元素靠左对齐 */
  justify-content: center;
  padding: 20px;
}

h1, h2, p {
  text-align: left;
}

.tabs {
  display: flex;
  margin-bottom: 20px;
}

.tab-button {
  background-color: #76c7c0;
  color: white;
  border: none;
  padding: 10px 20px;
  font-size: 16px;
  cursor: pointer;
  border-radius: 5px;
  transition: background-color 0.3s;
  margin-right: 10px; /* 添加右边距 */
}

.tab-button:hover {
  background-color: #5aa9a3;
}
</style>