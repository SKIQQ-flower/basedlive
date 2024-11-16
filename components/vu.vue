<template>
    <div class="relative w-5 h-96"> 
      <div class="track w-full h-full rounded-full bg-gradient-to-b from-slate-800 flex flex-col justify-end">
        <div class="thumb w-full rounded-xl bg-blue-500" :style="{ height: `${audioVolume}%` }"></div>
      </div>
      <input 
        type="range"
        class="absolute bottom-1/2 left-1/2 w-96 h-full -rotate-90 range origin-left"
        :value="sensitivity"
        @input="updateSensitivity($event)"
      >
    </div>
  </template>
  
  <script setup>
  import { defineProps, defineEmits } from 'vue';
  defineProps({
    audioVolume: {
      type: Number,
      required: true,
    },
    sensitivity: {
      type: Number,
      required: true,
    },
  });
  const emit = defineEmits(['update:sensitivity']);
  function updateSensitivity(event) {
    emit('update:sensitivity', Number(event.target.value));
  }
  </script>
  
  <style>
  .range {
    appearance: none;
    opacity: 1;
    background-color: transparent;
    cursor: pointer;
    transform-origin: left;
    transform: rotate(-90deg) translateX(-100%);
  }
  </style>