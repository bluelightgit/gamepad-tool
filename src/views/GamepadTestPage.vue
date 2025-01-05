<template>
  <main class="container">
    <div class="gamepad-test">
      <h1>Gamepad Test</h1>

      <div v-if="Object.keys(gamepads).length > 0">
        <div class="selector">
          <label for="gamepad-select">Select Gamepad:</label>
          <select id="gamepad-select" v-model="selectedGamepadId">
            <option v-for="(gamepad, id) in gamepads" :key="id" :value="id">
              {{ gamepad.name }}
            </option>
          </select>
        </div>

        <div v-if="selectedGamepad" class="gamepad-info">
          <h2>{{ selectedGamepad.name }}</h2>
          <p>Power Info: {{ selectedGamepad.power_info }}</p>
          <p>Vendor ID: {{ selectedGamepad.vendor_id }}</p>
          <p>Product ID: {{ selectedGamepad.product_id }}</p>

          <div class="layout-container">
            <!-- 按键 -->
            <div class="buttons-area">
              <h3>Buttons</h3>
              <div class="buttons-grid">
                <div
                  v-for="buttonKey in fixedButtonOrder"
                  :key="buttonKey"
                  class="button-item"
                >
                  <div class="progress-container">
                    <progress
                      :value="selectedGamepad.buttons[buttonKey]?.value || 0"
                      max="1"
                      class="vertical-progress"
                    ></progress>
                  </div>
                  <div class="button-info">
                    <div class="button-name">
                      {{ selectedGamepad.buttons[buttonKey]?.button || buttonKey }}
                    </div>
                    <div class="button-value">
                      {{ (selectedGamepad.buttons[buttonKey]?.value || 0).toFixed(2) }}
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- 摇杆 -->
            <div class="axes-area">
              <h3>Axes & Joysticks</h3>
              <!-- <div class="axes-grid">
                <div v-for="(axis, axisKey) in selectedGamepad.axes" :key="axisKey" class="axis-item">
                  <strong>{{ axisKey }}:</strong> {{ axis.value.toFixed(2) }}
                </div>
              </div> -->
              <div class="joystick-group">
                <JoystickVisualization
                  :axisX="selectedGamepad.axes['LeftThumbX']?.value || 0"
                  :axisY="selectedGamepad.axes['LeftThumbY']?.value || 0"
                />
                <JoystickVisualization
                  :axisX="selectedGamepad.axes['RightThumbX']?.value || 0"
                  :axisY="selectedGamepad.axes['RightThumbY']?.value || 0"
                />
              </div>
            </div>
          </div>
        </div>
      </div>

      <div v-else>
        <p>{{ orignalData }}</p>
      </div>
    </div>
  </main>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { listen } from "@tauri-apps/api/event";
import JoystickVisualization from "../components/JoystickVisualization.vue";
import { invoke } from "@tauri-apps/api/core";

interface AxisData {
  axis: string;
  value: number;
}

interface ButtonData {
  button: string;
  is_pressed: boolean;
  value: number;
}

interface GamepadInfo {
  id: number;
  name: string;
  vendor_id?: number;
  product_id?: number;
  power_info: string;
  axes: Record<string, AxisData>;
  buttons: Record<string, ButtonData>;
}

const orignalData = ref<string>("Loading...");
const gamepads = ref<Record<string, GamepadInfo>>({});
const selectedGamepadId = ref<string | null>(null);

// 定义固定的按键顺序
const fixedButtonOrder = [
  "A", "B", "X", "Y", // 主按钮
  "LeftShoulder", "RightShoulder", // 肩键
  "LeftTrigger", "RightTrigger", // 扳机键
  "Back", "Start", // 功能键
  "LeftThumb", "RightThumb", // 摇杆按键
  "DPadUp", "DPadDown", "DPadLeft", "DPadRight" // 方向键
];

const selectedGamepad = computed(() => {
  return selectedGamepadId.value ? gamepads.value[selectedGamepadId.value] : null;
});

function startUpdate() {
  invoke("start_update_thread").catch((error) => {
    console.error("Failed to start update:", error);
  });
}

async function fetchGamepads() {
  try {
    listen("gamepads_info", (event) => {
      console.log(event.payload);
      const result = event.payload;
      gamepads.value = result as Record<string, GamepadInfo>;
    });

    if (!selectedGamepadId.value && Object.keys(gamepads.value).length > 0) {
      selectedGamepadId.value = Object.keys(gamepads.value)[0];
    }
  } catch (error) {
    console.error("Failed to fetch gamepads:", error);
  }
}

onMounted(() => {
  startUpdate();
  fetchGamepads();
});
</script>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 20px;
  background-color: #f0f0f0;
}

.gamepad-test {
  max-width: 700px;
  margin: 20px auto;
  text-align: center;
  border: 1px solid #ddd;
  padding: 15px;
  border-radius: 6px;
  background-color: #fff;
}

.selector {
  margin-bottom: 20px;
}

.layout-container {
  display: flex;
  gap: 20px;
}

.buttons-area,
.axes-area {
  flex: 1;
}

.buttons-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr); /* 每行显示4个按键 */
  gap: 8px;
  margin-bottom: 20px;
}

.button-item {
  display: flex;
  align-items: center;
  background: #f9f9f9;
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 0.9em;
}

.progress-container {
  width: 8px; /* 固定宽度 */
  height: 40px;
  margin-right: 8px;
  position: relative;
  display: flex;
  align-items: flex-end;
  flex-shrink: 0; /* 禁止缩小 */
}

.vertical-progress {
  width: 100%;
  height: 100%;
  appearance: none;
  writing-mode: vertical-lr;
  transform: rotate(180deg);
}

.vertical-progress::-webkit-progress-bar {
  background-color: #e0e0e0;
  border-radius: 5px;
}

.vertical-progress::-webkit-progress-value {
  background-color: #42b983;
  border-radius: 5px;
}

.button-info {
  display: flex;
  flex-direction: column;
  justify-content: left;
  min-width: 0;
}

.button-name {
  font-weight: bold;
  margin-bottom: 2px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 60px; /* 限制显示长度 */
}

.button-value {
  font-size: 0.8em;
  color: #666;
}

.axes-area {
  flex: 1;
}

.axes-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 10px;
  margin-bottom: 20px;
}

.axis-item {
  background: #f9f9f9;
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 0.9em;
}

.joystick-group {
  display: flex;
  justify-content: center;
  gap: 20px;
  margin-top: 20px;
}
</style>