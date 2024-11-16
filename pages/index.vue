<template>
    <div class="flex flex-row gap-5">
        <Vu :audioVolume="audioVolume" v-model:sensitivity="sensitivity" />
        <img :src="image"/>

    </div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core';
const audioVolume = ref(50);
const sensitivity = ref(50);
const image = ref("")
async function getAudioVolume() {
    try {
        return await invoke('get_audio_volume');
    } catch (error) {
        console.error('Error fetching audio volume:', error);
        return 0;
    }
}
let monitoringInterval = null;

async function updateAudioVolume() {
    const volume = await getAudioVolume();
    console.log("Volume:", volume, "Sensitivity:", sensitivity.value);
    image.value = volume > sensitivity.value ? "/sinas1.png" : "/sinas2.png";
    audioVolume.value = volume;
}

onMounted(() => {
    monitoringInterval = setInterval(updateAudioVolume, 100);
});

onUnmounted(() => {
    clearInterval(monitoringInterval);
});
</script>
