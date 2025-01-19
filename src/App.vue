<script setup lang="ts">

import GamepadTestPage from "./views/GamepadTestPage.vue";

</script>

<template>
<!--  <div class="container">-->
    <GamepadTestPage/>
<!--  </div>-->
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

</style>