<template>
  <div class="vu-container relative w-6 h-96">
    <div
      class="track w-full h-full rounded-full bg-gradient-to-b from-slate-800 flex flex-col justify-end"
      ref="trackElement"
    >
      <div
        class="volume-indicator w-full rounded-xl bg-blue-500 opacity-50"
        :style="{ height: `${audioVolume}%` }"
      ></div>
    </div>

    <div
      class="thumb absolute w-6 h-6 rounded-full bg-white border border-gray-500 cursor-pointer z-10"
      :style="{ bottom: `${modelValue}%`, transform: 'translateY(50%)' }"
      @mousedown="startDrag"
      @touchstart.prevent="startDrag"
    ></div>
  </div>
</template>

<script setup>
import { ref } from 'vue';

const props = defineProps({
  audioVolume: { type: Number, required: true },
  modelValue: { type: Number, required: true },
});

const emit = defineEmits(['update:modelValue']);

const dragging = ref(false);
const trackHeight = ref(0);
const trackTop = ref(0);
const trackElement = ref(null);

function startDrag(event) {
  dragging.value = true;
  updateTrackInfo();
  handleDrag(event);

  const moveEvent = event.touches ? 'touchmove' : 'mousemove';
  const endEvent = event.touches ? 'touchend' : 'mouseup';

  document.addEventListener(moveEvent, handleDrag);
  document.addEventListener(endEvent, stopDrag);
}

function stopDrag(event) {
  dragging.value = false;

  const moveEvent = event.touches ? 'touchmove' : 'mousemove';
  const endEvent = event.touches ? 'touchend' : 'mouseup';

  document.removeEventListener(moveEvent, handleDrag);
  document.removeEventListener(endEvent, stopDrag);
}

function handleDrag(event) {
  event.preventDefault();

  if (!dragging.value) return;

  const clientY = event.touches ? event.touches[0].clientY : event.clientY;
  const position = Math.min(Math.max(clientY - trackTop.value, 0), trackHeight.value);
  const percentage = 100 - (position / trackHeight.value) * 100;

  emit('update:modelValue', Math.round(percentage));
}

function updateTrackInfo() {
  const rect = trackElement.value.getBoundingClientRect();
  trackHeight.value = rect.height;
  trackTop.value = rect.top;
}
</script>

<style scoped>
.vu-container {
  position: relative;
}

.track {
  position: relative;
}

.volume-indicator {
  transition: height 0.1s;
}

.thumb {
  position: absolute;
  transition: bottom 0.1s;
  background-color: white;
}

.thumb:active {
  background-color: #ccc;
}
</style>
