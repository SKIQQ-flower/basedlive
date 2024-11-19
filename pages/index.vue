<template>
    <div class="flex flex-row gap-5 w-full absolute h-full top-0 left-0 justify-start items-start p-5">
        <Vu class="absolute top-1/3 z-10" :audioVolume="audioVolume" v-model="sensitivity" />
        <div class="draggable-container absolute w-full h-full left-0 top-0" @mousedown="onMouseDown" @wheel="onZoom"
            @mousemove="onMouseMove" @mouseup="onMouseUp" @mouseleave="onMouseUp" @touchstart="onTouchStart"
            @touchmove="onTouchMove" @touchend="onTouchEnd" ref="imageContainer">
            <template v-if="characterImage">
                <img :src="characterImage" alt="Character" :style="imageStyle" class="draggable-image max-w-full" />
            </template>
            <template v-else>
                <div class="flex items-center justify-center w-full h-full text-gray-500">
                    Nenhum personagem carregado
                </div>
            </template>
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, watch, nextTick } from 'vue';
import { invoke } from '@tauri-apps/api/core';
const { currentCharacter } = useCharacter();

const audioVolume = ref(50);
const sensitivity = ref(50);
const characterImage = ref(null);
const idleImage = ref();
const blinkingIdleImage = ref();
const speakingImage = ref();
const speakingBlinkingImage = ref();

let monitoringInterval = null;
let blinkInterval = null;
let isBlinking = ref(false);
let isSpeaking = ref(false);

const imageStyle = ref({
    transform: "translate(0px, 0px) scale(1)",
});

const dragState = ref({
    isDragging: false,
    startX: 0,
    startY: 0,
    translateX: 0,
    translateY: 0,
});

const scale = ref(1);

function onMouseDown(event) {
    dragState.value.isDragging = true;
    dragState.value.startX = event.clientX;
    dragState.value.startY = event.clientY;
}

function onMouseMove(event) {
    if (!dragState.value.isDragging) return;

    const deltaX = event.clientX - dragState.value.startX;
    const deltaY = event.clientY - dragState.value.startY;

    dragState.value.translateX += deltaX;
    dragState.value.translateY += deltaY;

    updateImageTransform();

    dragState.value.startX = event.clientX;
    dragState.value.startY = event.clientY;
}

function onMouseUp() {
    dragState.value.isDragging = false;
}

function onZoom(event) {
    event.preventDefault();
    const zoomSpeed = 0.1;
    const newScale = scale.value + (event.deltaY < 0 ? zoomSpeed : -zoomSpeed);
    scale.value = Math.max(0.5, Math.min(newScale, 3));
    updateImageTransform();
}

function onTouchStart(event) {
    if (event.touches.length === 1) {
        dragState.value.isDragging = true;
        dragState.value.startX = event.touches[0].clientX;
        dragState.value.startY = event.touches[0].clientY;
    }
}

function onTouchMove(event) {
    if (!dragState.value.isDragging || event.touches.length !== 1) return;

    const deltaX = event.touches[0].clientX - dragState.value.startX;
    const deltaY = event.touches[0].clientY - dragState.value.startY;

    dragState.value.translateX += deltaX;
    dragState.value.translateY += deltaY;

    updateImageTransform();

    dragState.value.startX = event.touches[0].clientX;
    dragState.value.startY = event.touches[0].clientY;
}

function onTouchEnd() {
    dragState.value.isDragging = false;
}

function updateImageTransform() {
    imageStyle.value.transform = `translate(${dragState.value.translateX}px, ${dragState.value.translateY}px) scale(${scale.value})`;
}

async function updateEmotionImages() {
    try {
        console.log('Current character:', currentCharacter.value);

        if (!currentCharacter.value?.emotions?.length) {
            console.warn("No character loaded or no emotions found");
            resetImages();
            return;
        }

        const defaultEmotion = currentCharacter.value.emotions[0];
        console.log('Default emotion:', defaultEmotion);

        if (!defaultEmotion?.frames) {
            console.warn("No frames found in default emotion");
            resetImages();
            return;
        }

        console.log('Frame paths:', {
            idle: defaultEmotion.frames["Idle"]?.path,
            blinkingIdle: defaultEmotion.frames["Blinking idle"]?.path,
            speaking: defaultEmotion.frames["Speaking"]?.path,
            speakingBlinking: defaultEmotion.frames["Speaking Blinking"]?.path
        });

        idleImage.value = defaultEmotion.frames["Idle"]?.path || null;
        blinkingIdleImage.value = defaultEmotion.frames["Blinking idle"]?.path || null;
        speakingImage.value = defaultEmotion.frames["Speaking"]?.path || null;
        speakingBlinkingImage.value = defaultEmotion.frames["Speaking Blinking"]?.path || null;

        characterImage.value = idleImage.value;

        console.log('Updated image values:', {
            character: characterImage.value,
            idle: idleImage.value,
            blinkingIdle: blinkingIdleImage.value,
            speaking: speakingImage.value,
            speakingBlinking: speakingBlinkingImage.value
        });

        await nextTick();
        startBlinking();

    } catch (error) {
        console.error('Error updating emotion images:', error);
        resetImages();
    }
}

function resetImages() {
    characterImage.value = null;
    idleImage.value = null;
    blinkingIdleImage.value = null;
    speakingImage.value = null;
    speakingBlinkingImage.value = null;
}


async function getAudioVolume() {
    try {
        return await invoke('get_audio_volume');
    } catch (error) {
        console.error('Error fetching audio volume:', error);
        return 0;
    }
}

function startBlinking() {
    const blinkDuration = 200;

    if (blinkInterval) {
        clearInterval(blinkInterval);
    }

    blinkInterval = setInterval(() => {
        isBlinking.value = !isBlinking.value;
        updateCharacterImage();

        if (isBlinking.value) {
            setTimeout(() => {
                isBlinking.value = false;
                updateCharacterImage();
            }, blinkDuration);
        }
    }, 3000);
}

function updateCharacterImage() {
    if (isSpeaking.value) {
        characterImage.value = isBlinking.value ? speakingBlinkingImage.value : speakingImage.value;
    } else {
        characterImage.value = isBlinking.value ? blinkingIdleImage.value : idleImage.value;
    }
}

async function updateAudioVolume() {
    const volume = await getAudioVolume();
    audioVolume.value = volume;

    const wasSpeaking = isSpeaking.value;
    isSpeaking.value = volume > sensitivity.value;

    if (wasSpeaking !== isSpeaking.value) {
        updateCharacterImage();
    }
}

watch(
    currentCharacter,
    async (newCharacter) => {
        console.log('Character changed in viewer:', newCharacter);
        await updateEmotionImages();
    },
    { immediate: true, deep: true }
);

onMounted(async () => {
    console.log('Component mounted');
    await updateEmotionImages();
    monitoringInterval = setInterval(updateAudioVolume, 100);
});

onUnmounted(() => {
    if (monitoringInterval) clearInterval(monitoringInterval);
    if (blinkInterval) clearInterval(blinkInterval);
});
</script>

<style scoped>
.draggable-container {
    overflow: hidden;
    cursor: grab;
    background-color: rgb(15 23 42);
    border-radius: 0.375rem;
    display: flex;
    align-items: center;
    justify-content: center;
}

.draggable-container:active {
    cursor: grabbing;
}

.draggable-image {
    display: block;
    max-width: 100%;
    max-height: 100%;
    width: auto;
    height: auto;
    object-fit: contain;
    user-select: none;
    touch-action: none;
    -webkit-user-drag: none;
    -moz-user-select: none;
    -webkit-user-select: none;
    -ms-user-select: none;
}


.draggable-image {
    display: block;
    height: auto;
    user-select: none;
    touch-action: none;
    -webkit-user-drag: none;
    -moz-user-select: none;
    -webkit-user-select: none;
    -ms-user-select: none;
}
</style>