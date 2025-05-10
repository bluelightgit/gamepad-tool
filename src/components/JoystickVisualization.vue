<template>
  <div class="joystick-container">
    <canvas ref="canvas" width="180" height="180" class="joystick-canvas"></canvas>
     
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
import { computed, ref, watch, onMounted } from "vue";

interface HistoryPoint {
  x: number;
  y: number;
}

const props = defineProps<{
  axisX: number;
  axisY: number;
  historyPoints?: HistoryPoint[];
  showHistory?: boolean;
}>();

// canvas reference and 2D context
const canvas = ref<HTMLCanvasElement | null>(null);
let ctx: CanvasRenderingContext2D | null = null;

// Draw function: throttle via requestAnimationFrame
let drawPending = false;
const draw = () => {
  if (!ctx) return;
  const c = ctx;
  const w = canvas.value!.width;
  const h = canvas.value!.height;
  c.clearRect(0, 0, w, h);
  // draw border
  c.strokeStyle = '#ddd'; c.lineWidth = 2;
  c.strokeRect(5, 5, w-10, h-10);
  // middle lines
  c.beginPath(); c.moveTo(w/2,5); c.lineTo(w/2,h-5);
  c.moveTo(5,h/2); c.lineTo(w-5,h/2); c.stroke();
  // circle
  c.beginPath(); c.arc(w/2,h/2,(w-10)/2,0,2*Math.PI); c.stroke();
  // history
  if (props.showHistory && props.historyPoints && props.historyPoints.length) {
    const pts = props.historyPoints;
    const len = pts.length;
    c.lineWidth = 1.5;
    // path
    c.beginPath();
    pts.forEach((p,i)=>{
      const x = w/2 + p.x*((w-10)/2);
      const y = h/2 - p.y*((h-10)/2);
      if (i===0) c.moveTo(x,y); else c.lineTo(x,y);
    });
    c.strokeStyle = '#ff6b6b'; c.globalAlpha = 0.3;
    c.stroke(); c.globalAlpha = 1;
    // points with fading
    pts.forEach((p,i)=>{
      const x = w/2 + p.x*((w-10)/2);
      const y = h/2 - p.y*((h-10)/2);
      const alpha = (i+1)/len;
      c.fillStyle = `rgba(255,71,87,${alpha})`;
      c.beginPath(); c.arc(x,y,2,0,2*Math.PI); c.fill();
    });
  }
  // current axis line
  const cx = w/2 + props.axisX*((w-10)/2);
  const cy = h/2 - props.axisY*((h-10)/2);
  c.beginPath(); c.moveTo(w/2,h/2); c.lineTo(cx,cy);
  c.strokeStyle = 'gray'; c.lineWidth = 1; c.stroke();
  // current point
  c.beginPath(); c.arc(cx,cy,5,0,2*Math.PI);
  c.fillStyle = 'gray'; c.fill();
};

// Watch props for redraw
watch([() => props.axisX, () => props.axisY, () => props.historyPoints],()=>{
  if (!drawPending) {
    drawPending = true;
    requestAnimationFrame(()=>{ draw(); drawPending=false; });
  }
}, { deep:true });

onMounted(()=>{
  if (canvas.value) ctx = canvas.value.getContext('2d');
  draw();
});

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
  width: 100%;
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
  transition: all 0.01s ease;
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

.joystick-canvas {
  border-radius: 4px;
  background-color: #fff;
  box-shadow: 0 0 5px rgba(0,0,0,0.1);
}

/* Adjust layout */
.joystick-container {
  flex-direction: column; /* Change to column to stack elements vertically */
  align-items: center;
}

.y-axis-bar {
  order: 1;
}

.x-axis-bar {
  order: 3;
  margin-left: 0; /* Remove left margin */
}
</style>