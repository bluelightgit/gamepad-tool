<template>
  <main class="container">
    <div class="gamepad-test">
      <div class="gamepad-tabs">
        <div 
          v-for="id in [0, 1, 2, 3]" 
          :key="id"
          class="gamepad-tab"
          :class="{ 
            'active': selectedGamepadId === id,
            'disabled': !isGamepadAvailable(id)
          }"
          @click="isGamepadAvailable(id) && selectGamepad(id)"
        >
          <span>Gamepad {{ id }}</span>
        </div>
      </div>
      <div class="header">
        <h1></h1>
        <div class="settings">
          <button class="settings-button" @click="toggleSettingsMenu">
            ⚙️ Settings
          </button>
          <div v-if="isSettingsMenuOpen" class="settings-menu">
            <!-- 一级菜单选项 -->
            <div class="settings-options" v-if="!activeSubMenu">
              <div 
                class="settings-option" 
                @click="showSubMenu('logSize')"
              >
                Log Size
              </div>
              <div 
                class="settings-option" 
                @click="showSubMenu('frameRate')"
              >
                Frame Rate
              </div>
            </div>
            
            <!-- 日志大小二级菜单 -->
            <div class="settings-submenu" v-if="activeSubMenu === 'logSize'">
              <div class="submenu-header">
                <button class="back-button" @click="hideSubMenu">⬅️</button>
                <h4>Select Log Size</h4>
              </div>
              <div class="log-size-selector">
                <div
                  v-for="size in logSizeOptions"
                  :key="size"
                  class="log-size-option"
                  :class="{ selected: selectedLogSize === size }"
                  @click="setLogSize(size)"
                >
                  {{ size }}
                </div>
              </div>
            </div>
            
            <!-- 帧率二级菜单 -->
            <div class="settings-submenu" v-if="activeSubMenu === 'frameRate'">
              <div class="submenu-header">
                <button class="back-button" @click="hideSubMenu">⬅️</button>
                <h4>Select Frame Rate</h4>
              </div>
              <div class="frame-rate-selector">
                <div
                  v-for="rate in frameRateOptions"
                  :key="rate"
                  class="frame-rate-option"
                  :class="{ selected: selectedFrameRate === rate }"
                  @click="setFrameRate(rate)"
                >
                  {{ rate }} Hz
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 添加应用状态显示 -->
      <div v-if="isInitializing" class="status-message loading">
        <p>Initializing application, please wait...</p>
      </div>
      <div v-if="hasInitError" class="status-message error">
        <p>Initialization error: {{ initErrorMsg }}</p>
        <button @click="retryInitialization" class="retry-button">Retry</button>
      </div>

      <div class="selector">

        <div v-if="selectedGamepad" class="gamepad-info">
          <h2>{{ selectedGamepad.name }}</h2>
          <p v-if="selectedGamepad.name === 'Loading...'">
            Loading gamepad data, please wait...
          </p>
          <p v-else>Power Info: {{ selectedGamepad.power_info }}</p>

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
              <div class="joystick-controls">
                <button 
                  class="history-toggle-button" 
                  @click="toggleHistoryPoints"
                  :class="{ active: showHistoryPoints }"
                >
                {{ showHistoryPoints ? 'Hide Trajectory' : 'Show Trajectory' }}
              </button>
              </div>
              <div class="joystick-group">
                <JoystickVisualization
                  :axisX="getAxisValue('LeftThumbX')"
                  :axisY="getAxisValue('LeftThumbY')"
                  :historyPoints="leftJoystickHistory"
                  :showHistory="showHistoryPoints"
                />
                <JoystickVisualization
                  :axisX="getAxisValue('RightThumbX')"
                  :axisY="getAxisValue('RightThumbY')"
                  :historyPoints="rightJoystickHistory"
                  :showHistory="showHistoryPoints"
                />
              </div>
            </div>

            <!-- 轮询率数据 -->
            <div class="polling-rate-area">
              <h3></h3>
              <div v-if="selectedPollingRateData" class="polling-rate-data">
                <p>
                  <span>Average Rate:</span>
                  <span>{{ formatNumber(selectedPollingRateData.polling_rate_avg) }} Hz</span>
                </p>
                <p>
                  <span>Min Rate:</span>
                  <span>{{ formatNumber(selectedPollingRateData.polling_rate_min) }} Hz</span>
                </p>
                <p>
                  <span>Max Rate:</span>
                  <span>{{ formatNumber(selectedPollingRateData.polling_rate_max) }} Hz</span>
                </p>
                <p>
                  <span>Avg Interval:</span>
                  <span>{{ formatNumber(selectedPollingRateData.avg_interval) }} ms</span>
                </p>
                <p>
                  <span>Error (L):</span>
                  <span>{{ formatNumber(selectedPollingRateData.avg_error_l) }} %</span>
                </p>
                <p>
                  <span>Error (R):</span>
                  <span>{{ formatNumber(selectedPollingRateData.avg_error_r) }} %</span>
                </p>
              </div>
              <div v-else class="polling-rate-data empty">
                <p class="no-data">No polling rate data available</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </main>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, watch } from "vue";
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
  avg_error_r: number;
  avg_error_l: number;
}

interface HistoryPoint {
  x: number;
  y: number;
}

interface JoystickLevelsData {
  leftLevelCount: number;   // 左摇杆X轴分级数量
  rightLevelCount: number;  // 右摇杆X轴分级数量
  leftValues: number[];     // 左摇杆X轴所有不同值
  rightValues: number[];    // 右摇杆X轴所有不同值
}

interface OutputLog {
  timestamp: number;
  xyxy: [number, number, number, number]; // 已经缩放到-1至1范围的(x1, y1, x2, y2)
}

const selectedGamepad = ref<GamepadInfo>(createDefaultGamepad());
const selectedGamepadId = ref(0);
const pollingRateData = ref<Record<string, PollingRateResult | null>>({
  '0': createDefaultPollingRateResult()
});
const joystickLevelsData = ref<Record<string, JoystickLevelsData | null>>({});
const isSettingsMenuOpen = ref(false);
const activeSubMenu = ref<'logSize' | 'frameRate' | null>(null);
const selectedFrameRate = ref(60);
const selectedLogSize = ref(2000);
const showHistoryPoints = ref(false);
const leftJoystickHistory = ref<HistoryPoint[]>([]);
const rightJoystickHistory = ref<HistoryPoint[]>([]);
const maxHistoryPoints = selectedLogSize.value / 5;
const frameRateOptions = [30, 60, 120, 180];
const logSizeOptions = [1000, 2000, 4000, 8000];
const fixedButtonOrder = [
  "A",
  "B",
  "X",
  "Y", // 主按钮
  "LeftShoulder",
  "RightShoulder", // 肩键
  "LeftTrigger",
  "RightTrigger", // 扳机键
  "Back",
  "Start", // 功能键
  "LeftThumb",
  "RightThumb", // 摇杆按键
  "DPadUp",
  "DPadDown",
  "DPadLeft",
  "DPadRight", // 方向键
];

const selectedPollingRateData = computed(() =>
  selectedGamepad.value ? pollingRateData.value[selectedGamepadId.value] : null
);
const gamepadIds = ref<number[]>([]);

const getButtonValue = (buttonKey: string) =>
  selectedGamepad.value?.buttons[buttonKey]?.value || 0;
const getButtonName = (buttonKey: string) =>
  selectedGamepad.value?.buttons[buttonKey]?.button || buttonKey;
const formatButtonValue = (buttonKey: string) =>
  getButtonValue(buttonKey).toFixed(2);
const getAxisValue = (axisKey: string) =>
  selectedGamepad.value?.axes[axisKey]?.value || 0;
const formatNumber = (value: number) => value.toFixed(2);
const isGamepadAvailable = (id: number): boolean => {
  return gamepadIds.value.includes(id);
};

// 添加选择手柄的函数
const selectGamepad = (id: number) => {
  if (selectedGamepadId.value !== id) {
    selectedGamepadId.value = id;
  }
};
// 添加初始化状态变量
const isInitializing = ref(true);
const hasInitError = ref(false);
const initErrorMsg = ref("");

onMounted(async () => {
  try {
    isInitializing.value = true;
    hasInitError.value = false;
    initErrorMsg.value = "";
    
    console.log("Component mounted, initializing...");
    
    // 首先设置事件监听器
    console.log("Setting up event listeners...");
    await Promise.all([
      fetchGamepads(), 
      fetchPollingRate(),
      fetchPollingRateLog(),
      fetchJoystickLevels()
    ]);
    
    // 然后更新手柄ID
    console.log("Updating gamepad IDs...");
    await updateGamepadIds();
    
    // 设置日志大小
    console.log(`Setting log size to ${selectedLogSize.value}...`);
    await set_log_size(selectedLogSize.value);
    
    // 最后启动主线程
    console.log("Starting main thread...");
    const userIdToUse = selectedGamepadId.value ? selectedGamepadId.value : 0;
    await start_main_thread(userIdToUse, selectedFrameRate.value);
    
    console.log("Initialization complete");
    isInitializing.value = false;

    // 添加定时更新手柄状态
    setInterval(async () => {
      await updateGamepadIds();
    }, 1000);
  } catch (error) {
    console.error("Error during initialization:", error);
    hasInitError.value = true;
    initErrorMsg.value = error instanceof Error ? error.message : "未知错误";
    isInitializing.value = false;
  }
});

const toggleSettingsMenu = () => {
  isSettingsMenuOpen.value = !isSettingsMenuOpen.value;
  // 关闭菜单时，也关闭子菜单
  if (!isSettingsMenuOpen.value) {
    activeSubMenu.value = null;
  }
};

const showSubMenu = (menuName: 'logSize' | 'frameRate') => {
  activeSubMenu.value = menuName;
};

const hideSubMenu = () => {
  activeSubMenu.value = null;
};

const setFrameRate = async (rate: number) => {
  selectedFrameRate.value = rate;
  await stop_main_thread();
  // 确保在重新启动时应用当前日志大小设置
  await set_log_size(selectedLogSize.value);
  await start_main_thread(selectedGamepadId.value, rate);
  // 关闭菜单
  hideSubMenu();
  isSettingsMenuOpen.value = false;
};

// 监听 selectedGamepadId 的变化，每次变化时重新调用 start_main_thread
watch(selectedGamepadId, async (newVal, oldVal) => {
  if (newVal !== oldVal) {
    await stop_main_thread();
    // 确保在重新启动时应用当前日志大小设置
    await set_log_size(selectedLogSize.value);
    await start_main_thread(newVal ? newVal : 0, selectedFrameRate.value);
    updateGamepadIds();
  }
});

async function start_main_thread(
  userId?: number | 0,
  frameRate: number = selectedFrameRate.value
) {
  try {
    console.log(`Starting main thread with userId: ${userId}, frameRate: ${frameRate}`);
    return await invoke<void>("start_update", { userId, frameRate });
  } catch (error) {
    console.error("Error in start_main_thread:", error);
    throw error;
  }
}

async function stop_main_thread() {
  return invoke<void>("stop_update");
}

async function get_gamepad_ids() {
  return invoke<number[]>("get_gamepad_ids");
}

async function updateGamepadIds() {
  try {
    console.log("Fetching gamepad IDs...");
    const ids = await get_gamepad_ids();
    console.log("Received gamepad IDs:", ids);
    gamepadIds.value = ids;
    
    // 如果没有找到任何手柄ID，添加默认ID 0
    if (!ids || ids.length === 0) {
      console.log("No gamepad IDs found, adding default ID 0");
      gamepadIds.value = [0];
    }
    
    // 如果当前选定的ID不在列表中，选择第一个可用的ID
    if (!gamepadIds.value.includes(selectedGamepadId.value)) {
      console.log(`Selected ID ${selectedGamepadId.value} not in list, selecting first ID`);
      selectedGamepadId.value = gamepadIds.value[0];
    }
    
    return ids;
  } catch (error) {
    console.error("Error in updateGamepadIds:", error);
    // 如果出错，添加默认ID
    gamepadIds.value = [0];
    return [0];
  }
}

async function fetchGamepads() {
  try {
    console.log("Setting up gamepads_info event listener");
    listen("gamepads_info", (event) => {
      console.log("Received gamepads_info event:", event.payload);
      if (event.payload) {
        selectedGamepad.value = event.payload as GamepadInfo;
      } else {
        console.warn("Received empty gamepads_info event payload");
        // 如果接收到空数据，使用默认手柄数据
        if (!selectedGamepad.value || selectedGamepad.value.name === "加载中...") {
          selectedGamepad.value = createDefaultGamepad(selectedGamepadId.value);
        }
      }
    });
  } catch (error) {
    console.error("Error in fetchGamepads:", error);
  }
}

async function fetchPollingRate() {
  try {
    console.log("Setting up polling_rate_result event listener");
    listen("polling_rate_result", (event) => {
      console.log("Received polling_rate_result:", event.payload);
      const result = event.payload as PollingRateResult;
      const userId = selectedGamepadId.value.toString();
      pollingRateData.value = { 
        [userId]: result 
      };
    });
  } catch (error) {
    console.error("Error in fetchPollingRate:", error);
  }
}

async function set_log_size(log_size: number) {
  try {
    console.log(`Setting log size to ${log_size}...`);
    await invoke<void>("set_log_size", { log_size });
    console.log(`Log size set to ${log_size}`);
  } catch (error) {
    console.error("Error in set_log_size:", error);
  }
}

let lastHistoryUpdateTime = 0;
const THROTTLE_INTERVAL = 16;

async function fetchPollingRateLog() {
  try {
    console.log("Setting up polling_rate_log event listener");
    listen("polling_rate_log", (event) => {
      const now = performance.now();
      const logs = event.payload as OutputLog[];
      
      // 如果开启了历史轨迹显示，则存储坐标点
      if (showHistoryPoints.value && logs && logs.length > 0
          && now - lastHistoryUpdateTime > THROTTLE_INTERVAL
      ) {
        lastHistoryUpdateTime = now;
        const maxProcessLogs = 10;
        const recentLogs = logs.slice(-maxProcessLogs);
        
        for (const log of recentLogs) {
          try {
            // 确保 xyxy 是有效的数组
            if (log.xyxy && typeof log.xyxy === 'object') {
              // 添加新的坐标点 - 直接使用后端已经缩放的值
              const newLeftPoint: HistoryPoint = {
                x: log.xyxy[0],
                y: log.xyxy[1]
              };
              
              const newRightPoint: HistoryPoint = {
                x: log.xyxy[2],
                y: log.xyxy[3]
              };
              
              // 检查点是否有效（避免添加接近原点的点）
              const isValidPoint = (point: HistoryPoint) => 
                Math.abs(point.x) > 0.01 || Math.abs(point.y) > 0.01;
              
              // 只添加有效点
              if (isValidPoint(newLeftPoint)) {
                leftJoystickHistory.value.push(newLeftPoint);
                if (leftJoystickHistory.value.length > maxHistoryPoints) {
                  leftJoystickHistory.value.shift();
                }
              }
              
              if (isValidPoint(newRightPoint)) {
                rightJoystickHistory.value.push(newRightPoint);
                if (rightJoystickHistory.value.length > maxHistoryPoints) {
                  rightJoystickHistory.value.shift();
                }
              }
            }
          } catch (error) {
            console.error('Error processing joystick log data:', error);
          }
        }
      }
    });
  } catch (error) {
    console.error("Error in fetchPollingRateLog:", error);
  }
}

async function fetchJoystickLevels() {
  try {
    console.log("Setting up joystick_levels event listener");
    listen("joystick_levels", (event) => {
      console.log("Received joystick_levels event:", event.payload);
      try {
        // 数据格式现在是直接的元组(leftCount, rightCount, leftValues[], rightValues[])
        if (event.payload) {
          const [leftCount, rightCount, leftValues, rightValues] = event.payload as [number, number, number[], number[]];
          
          // 为当前选中的用户ID创建数据条目
          const userId = selectedGamepadId.value.toString();
          joystickLevelsData.value = {
            [userId]: {
              leftLevelCount: leftCount,
              rightLevelCount: rightCount,
              leftValues: leftValues || [],
              rightValues: rightValues || []
            }
          };
        } else {
          console.warn("Received empty joystick_levels event payload");
        }
      } catch (error) {
        console.error("Error processing joystick_levels data:", error);
      }
    });
  } catch (error) {
    console.error("Error in fetchJoystickLevels:", error);
  }
}

const toggleHistoryPoints = () => {
  showHistoryPoints.value = !showHistoryPoints.value;
  
  if (!showHistoryPoints.value) {
    leftJoystickHistory.value = [];
    rightJoystickHistory.value = [];
  }
};

const setLogSize = async (size: number) => {
  selectedLogSize.value = size;
  await set_log_size(size);
  // 设置成功后关闭菜单
  hideSubMenu();
  isSettingsMenuOpen.value = false;
};

// 添加默认手柄数据函数
function createDefaultGamepad(id: number = 0): GamepadInfo {
  return {
    id,
    name: "Loading...",
    guid: "",
    power_info: "Unknown",
    axes: {
      "LeftThumbX": { axis: "LeftThumbX", value: 0 },
      "LeftThumbY": { axis: "LeftThumbY", value: 0 },
      "RightThumbX": { axis: "RightThumbX", value: 0 },
      "RightThumbY": { axis: "RightThumbY", value: 0 }
    },
    buttons: {
      "A": { button: "A", is_pressed: false, value: 0 },
      "B": { button: "B", is_pressed: false, value: 0 },
      "X": { button: "X", is_pressed: false, value: 0 },
      "Y": { button: "Y", is_pressed: false, value: 0 },
      "LeftShoulder": { button: "LeftShoulder", is_pressed: false, value: 0 },
      "RightShoulder": { button: "RightShoulder", is_pressed: false, value: 0 },
      "LeftTrigger": { button: "LeftTrigger", is_pressed: false, value: 0 },
      "RightTrigger": { button: "RightTrigger", is_pressed: false, value: 0 },
      "Back": { button: "Back", is_pressed: false, value: 0 },
      "Start": { button: "Start", is_pressed: false, value: 0 },
      "LeftThumb": { button: "LeftThumb", is_pressed: false, value: 0 },
      "RightThumb": { button: "RightThumb", is_pressed: false, value: 0 },
      "DPadUp": { button: "DPadUp", is_pressed: false, value: 0 },
      "DPadDown": { button: "DPadDown", is_pressed: false, value: 0 },
      "DPadLeft": { button: "DPadLeft", is_pressed: false, value: 0 },
      "DPadRight": { button: "DPadRight", is_pressed: false, value: 0 }
    }
  };
}

// 添加默认的轮询率数据
function createDefaultPollingRateResult(): PollingRateResult {
  return {
    polling_rate_avg: 0,
    polling_rate_min: 0,
    polling_rate_max: 0,
    avg_interval: 0,
    drop_rate: 0,
    avg_error_l: 0,
    avg_error_r: 0
  };
}

// 添加重试初始化方法
const retryInitialization = async () => {
  try {
    isInitializing.value = true;
    hasInitError.value = false;
    initErrorMsg.value = "";
    
    // 重新运行初始化流程
    // 首先设置事件监听器
    await Promise.all([
      fetchGamepads(), 
      fetchPollingRate(),
      fetchPollingRateLog(),
      fetchJoystickLevels()
    ]);
    
    // 然后更新手柄ID
    await updateGamepadIds();
    
    // 设置日志大小
    await set_log_size(selectedLogSize.value);
    
    // 最后启动主线程
    const userIdToUse = selectedGamepadId.value ? selectedGamepadId.value : 0;
    await start_main_thread(userIdToUse, selectedFrameRate.value);
    
    isInitializing.value = false;
  } catch (error) {
    console.error("Error during retry initialization:", error);
    hasInitError.value = true;
    initErrorMsg.value = error instanceof Error ? error.message : "未知错误";
    isInitializing.value = false;
  }
};
</script>

<style scoped>
.container {  
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 0;
  background-color: #f0f0f0;
  margin: 0;
}

.gamepad-tabs {
  display: flex;
  gap: 2px;
  margin-bottom: 20px;
  background-color: #f0f0f0;
  padding: 4 4px;
  border-bottom: 1px solid #ddd;
  width: 100%;
}

.gamepad-tab {
  position: relative;
  padding: 12px 12px;
  background-color: #e9e9e9;
  border-radius: 8px 8px 0 0;
  border: 1px solid #ddd;
  border-bottom: none;
  font-weight: 1800;
  color: #555;
  cursor: pointer;
  transition: all 0.2s ease;
  margin-bottom: -1px; /* 让激活的标签下边框覆盖容器的下边框 */
  z-index: 1;
  box-shadow: 0 -2px 4px rgba(0, 0, 0, 0.05);
}

.gamepad-tab.active {
  background-color: #fff;
  color: #42b983;
  border-bottom: 1px solid #fff;
  z-index: 2;
  box-shadow: 0 -3px 6px rgba(0, 0, 0, 0.2);
}

.gamepad-tab.disabled {
  opacity: 0.5;
  cursor: not-allowed;
  background-color: #f5f5f5;
  color: #aaa;
}

.gamepad-tab:hover:not(.disabled) {
  background-color: #f4f4f4;
}

.gamepad-tab.active:hover {
  background-color: #fff;
}

.gamepad-test {
  max-width: 2400px;
  /* width: 100%; */
  margin: 0 auto;
  text-align: center;
  border: 1px solid #ddd;
  padding: 10px;
  border-radius: 6px;
  background-color: #fff;
}

.layout-container {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  grid-template-areas: 
    "buttons joysticks polling";
  gap: 20px;
  width: 100%;
  min-height: 400px;
}

.buttons-area {
  grid-area: buttons;
  min-width: 250px;
}

.axes-area {
  grid-area: joysticks;
  min-width: 250px;
}

.polling-rate-area {
  grid-area: polling;
  min-width: 250px;
  width: 100%;
  margin-top: 10px;
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

.joystick-group {
  display: flex;
  justify-content: center;
  gap: 20px;
  margin-top: 20px;
}

.polling-rate-data {
  width: 100%;
  margin-top: 10px;
  background: #f9f9f9;
  padding: 15px;
  border: 1px solid #ddd;
  border-radius: 6px;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.polling-rate-data p {
  margin: 0;
  padding: 4px 0;
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 1px solid #eee;
}

.polling-rate-data p:last-child {
  border-bottom: none;
}

@media (max-width: 1200px) {
  .layout-container {
    grid-template-columns: minmax(250px, 1fr) minmax(250px, 1fr);
    grid-template-areas: 
      "buttons joysticks"
      "polling polling";
  }
  
  .polling-rate-area {
    margin-top: 20px;
  }
  
  /* 中等屏幕下，polling-rate-data 区域的数据项采用水平排列，但内部数据垂直排列 */
  .polling-rate-data {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 12px;
    width: 100%;
    max-width: 800px;
    margin: 0 auto;
  }
  
  .polling-rate-data p {
    flex-direction: column;
    align-items: flex-start;
    padding: 8px;
    border: 1px solid #eee;
    border-radius: 4px;
    background-color: #fcfcfc;
    margin: 0;
  }
  
  .polling-rate-data p span:first-child {
    font-weight: bold;
    margin-bottom: 4px;
  }
}

@media (max-width: 800px) {
  .layout-container {
    grid-template-columns: 1fr;
    grid-template-areas: 
      "buttons"
      "joysticks"
      "polling";
  }
  
  .polling-rate-data {
    grid-template-columns: repeat(2, 1fr);
    max-width: 100%;
  }
}

/* 最小屏幕下的样式调整 */
@media (max-width: 500px) {
  .polling-rate-data {
    grid-template-columns: 1fr;
  }
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  position: relative;
}

.gamepad-info {
  width: 100%;
  min-width: 0;
  max-width: none;
  margin: 0 auto;
}

/* 添加平滑过渡效果 */
.gamepad-info,
.layout-container,
.buttons-area,
.axes-area,
.polling-rate-area,
.polling-rate-data {
  transition: all 0.2s ease;
  box-sizing: border-box;
}

.settings {
  position: relative;
}

.settings-button {
  padding: 8px 16px;
  background: #f0f0f0;
  border: 1px solid #ddd;
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.2s;
}

.settings-button:hover {
  background: #e0e0e0;
}

.settings-menu {
  position: absolute;
  right: 0;
  top: 100%;
  background: white;
  border: 1px solid #ddd;
  border-radius: 6px;
  padding: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  z-index: 100;
  min-width: 180px;
}

.settings-options {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.settings-option {
  padding: 10px 12px;
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.2s;
  font-weight: 500;
}

.settings-option:hover {
  background: #f0f0f0;
}

.settings-submenu {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.submenu-header {
  display: flex;
  align-items: center;
  margin-bottom: 8px;
}

.back-button {
  background: none;
  border: 1px solid #ababab;
  width: 25px;
  height: 25px;
  border-radius: 4px;
  cursor: pointer;
  font-size: 25px;
  display: flex;
  justify-content: center;
  align-items: center;
  color: #666;
}

.back-button:hover {
  color: #333;
  border: 7px solid #ffffff;
}

.frame-rate-selector,
.log-size-selector {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.frame-rate-option,
.log-size-option {
  padding: 8px 12px;
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.2s;
}

.frame-rate-option:hover,
.log-size-option:hover {
  background: #f0f0f0;
}

.frame-rate-option.selected,
.log-size-option.selected {
  background: #42b983;
  color: white;
}

.joystick-controls {
  display: flex;
  justify-content: center;
  margin-bottom: 10px;
}

.history-toggle-button {
  padding: 8px 16px;
  background: #f0f0f0;
  border: 1px solid #ddd;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
}

.history-toggle-button:hover {
  background: #e0e0e0;
}

.history-toggle-button.active {
  background: #42b983;
  color: white;
}

/* 添加状态消息样式 */
.status-message {
  margin: 20px 0;
  padding: 12px 16px;
  border-radius: 6px;
  text-align: center;
}

.status-message.loading {
  background-color: #e3f2fd;
  color: #0d47a1;
  border: 1px solid #90caf9;
}

.status-message.error {
  background-color: #ffebee;
  color: #b71c1c;
  border: 1px solid #ef9a9a;
}

.retry-button {
  margin-top: 8px;
  padding: 6px 16px;
  background-color: #f44336;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.retry-button:hover {
  background-color: #d32f2f;
}
</style>
