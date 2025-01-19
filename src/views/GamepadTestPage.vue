<template>
  <main class="container">
    <div class="gamepad-test">
      <h1>Gamepad Test</h1>

      <div v-if="hasGamepads">
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
          <p>GUID: {{ selectedGamepad.guid }}</p>

          <div class="layout-container">
            <!-- 按键区域 -->
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
                        :value="getButtonValue(buttonKey)"
                        max="1"
                        class="vertical-progress"
                    ></progress>
                  </div>
                  <div class="button-info">
                    <div class="button-name">
                      {{ getButtonName(buttonKey) }}
                    </div>
                    <div class="button-value">
                      {{ formatButtonValue(buttonKey) }}
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- 摇杆区域 -->
            <div class="axes-area">
              <h3>Joysticks</h3>
              <div class="joystick-group">
                <JoystickVisualization
                    :axisX="getAxisValue('LeftThumbX')"
                    :axisY="getAxisValue('LeftThumbY')"
                />
                <JoystickVisualization
                    :axisX="getAxisValue('RightThumbX')"
                    :axisY="getAxisValue('RightThumbY')"
                />
              </div>
            </div>

            <!-- 轮询率数据 -->
            <div class="polling-rate-area">
              <div v-if="selectedPollingRateData" class="polling-rate-data">
                <p>Avg: {{ formatNumber(selectedPollingRateData.polling_rate_avg) }} Hz</p>
                <p>Min: {{ formatNumber(selectedPollingRateData.polling_rate_min) }} Hz</p>
                <p>Max: {{ formatNumber(selectedPollingRateData.polling_rate_max) }} Hz</p>
                <p>Avg Interval: {{ formatNumber(selectedPollingRateData.avg_interval) }} ms</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </main>
</template>

<script setup lang="ts">
import {ref, onMounted, computed} from "vue";
import { listen } from "@tauri-apps/api/event";
import JoystickVisualization from "../components/JoystickVisualization.vue";

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
  guid: string;
  power_info: string;
  axes: Record<string, AxisData>;
  buttons: Record<string, ButtonData>;
}

interface PollingRateResult {
  polling_rate_avg: number;
  polling_rate_min: number;
  polling_rate_max: number;
  avg_interval: number;
  drop_rate: number;
}

interface PollingRateLog {
  timestamp: number;
  xxyy: [number, number, number, number];
}

// const originalData = ref<string>("Loading...");
const gamepads = ref<Record<string, GamepadInfo>>({});
const selectedGamepadId = ref<string | null>(null);
const pollingRateData = ref<Record<string, PollingRateResult | null>>({});

const fixedButtonOrder = [
  "A", "B", "X", "Y", // 主按钮
  "LeftShoulder", "RightShoulder", // 肩键
  "LeftTrigger", "RightTrigger", // 扳机键
  "Back", "Start", // 功能键
  "LeftThumb", "RightThumb", // 摇杆按键
  "DPadUp", "DPadDown", "DPadLeft", "DPadRight" // 方向键
];

const hasGamepads = computed(() => Object.keys(gamepads.value).length > 0);
const selectedGamepad = computed(() => selectedGamepadId.value ? gamepads.value[selectedGamepadId.value] : null);
const selectedPollingRateData = computed(() => selectedGamepadId.value ? pollingRateData.value[selectedGamepadId.value] : null);

const getButtonValue = (buttonKey: string) => selectedGamepad.value?.buttons[buttonKey]?.value || 0;
const getButtonName = (buttonKey: string) => selectedGamepad.value?.buttons[buttonKey]?.button || buttonKey;
const formatButtonValue = (buttonKey: string) => getButtonValue(buttonKey).toFixed(2);
const getAxisValue = (axisKey: string) => selectedGamepad.value?.axes[axisKey]?.value || 0;
const formatNumber = (value: number) => value.toFixed(2);

onMounted(async () => {
  await Promise.all([fetchGamepads(),
    fetchPollingRate()]);
});

async function fetchGamepads() {
  listen("gamepads_info", (event) => {
    gamepads.value = event.payload as Record<string, GamepadInfo>;
    if (!selectedGamepadId.value && hasGamepads.value) {
      selectedGamepadId.value = Object.keys(gamepads.value)[0];
    }
  });
}

async function fetchPollingRate() {
  listen("polling_rate_result", (event) => {
    pollingRateData.value = event.payload as Record<string, PollingRateResult>;
  });
}
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
  max-width: 1200px;
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
  grid-template-columns: repeat(4, 1fr);
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
  width: 8px;
  height: 40px;
  margin-right: 8px;
  position: relative;
  display: flex;
  align-items: flex-end;
  flex-shrink: 0;
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
  max-width: 60px;
}

.button-value {
  font-size: 0.8em;
  color: #666;
}

.axes-area {
  flex: 1;
}

.joystick-group {
  display: flex;
  justify-content: center;
  gap: 20px;
  margin-top: 20px;
}

.polling-rate-area {
  margin-top: 20px;
  text-align: center;
}

.polling-rate-area button {
  padding: 10px 20px;
  font-size: 1em;
  cursor: pointer;
  background-color: #42b983;
  color: white;
  border: none;
  border-radius: 5px;
}

.polling-rate-data {
  margin-top: 10px;
  background: #f9f9f9;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 6px;
}

.log-area {
  margin-top: 20px;
  background-color: #2c3e50;
  color: #ecf0f1;
  padding: 10px;
  border-radius: 6px;
  max-height: 200px;
  overflow-y: auto;
}

.log-area h3 {
  margin-bottom: 10px;
  font-size: 1em;
}

.log-list {
  font-size: 0.8em;
}

.log-item {
  display: flex;
  justify-content: space-between;
  padding: 4px 0;
  border-bottom: 1px solid #34495e;
}

.log-item:last-child {
  border-bottom: none;
}

.log-timestamp {
  color: #bdc3c7;
}

.log-data {
  color: #ecf0f1;
}
</style>