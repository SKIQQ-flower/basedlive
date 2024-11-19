<template>
    <div class="z-10">
        <div data-tauri-drag-region class="slate-900 flex items-center flex-row justify-between px-4 py-2">
            <p class="text-2xl">Basedlive</p>
        </div>
        <Menubar data-tauri-drag-region class="text-xl border-t-0 border-r-0 border-l-0">
            <MenubarMenu>
                <MenubarTrigger>
                    <Icon class="mr-2" name="mdi:file-outline" />
                    File
                </MenubarTrigger>
                <Dialog>
                    <MenubarContent>
                        <DialogTrigger asChild>
                            <MenubarItem>
                                New character <MenubarShortcut>Ctrl+N</MenubarShortcut>
                            </MenubarItem>
                        </DialogTrigger>
                        <MenubarItem @click="saveCharacterAs()">
                            Save character as<MenubarShortcut>Ctrl+Shift+N</MenubarShortcut>
                        </MenubarItem>
                        <MenubarItem @click="loadCharacterFromFile()">
                            Load character
                        </MenubarItem>
                        <!-- Adiciona submenu de personagens recentes -->
                        <MenubarSub v-if="recentCharacters.length">
                            <MenubarSubTrigger>Recent Characters</MenubarSubTrigger>
                            <MenubarSubContent>
                                <MenubarItem 
                                    v-for="name in recentCharacters" 
                                    :key="name"
                                    @click="loadRecentCharacter(name)"
                                >
                                    {{ name }}
                                </MenubarItem>
                            </MenubarSubContent>
                        </MenubarSub>
                        <MenubarSeparator />
                        <MenubarItem>New Window</MenubarItem>
                    </MenubarContent>
                    <DialogScrollContent class="sm:max-w-[750px]">
                        <DialogHeader>
                            <DialogTitle>Creating new character</DialogTitle>
                            <DialogDescription>
                                Select your character portrait for a specific audio level, or emotion. We do the rest.
                            </DialogDescription>
                        </DialogHeader>
                        <Input v-model="characterName" placeholder="Character name" />
                        <Button variant="secondary" @click="addNewEmotion">New Emotion</Button>
                        <hr class="border-slate-900">
                        <div v-for="(emotion, index) in emotions" class="flex flex-col gap-2" :key="index">
                            <div class="flex flex-row gap-2">
                                <Input v-model="emotion.name"
                                    :placeholder="index === 0 ? 'Default Emotion' : 'Emotion Name'"
                                    :disabled="index === 0" />
                                <Button class="w-10 h-10 p-2" variant="secondary" @click="removeEmotion(index)">
                                    <Icon size="4rem" name="mdi:close" />
                                </Button>
                            </div>
                            <div class="flex flex-row justify-between gap-2">
                                <div v-for="frameType in frameTypes" :key="frameType" class="w-40 h-40 relative">
                                    <label @click="selectImage(index, frameType)" class=" w-full h-full cursor-pointer
                                        flex group hover:bg-slate-800 transition-all duration-200 flex-col items-center
                                        justify-center p-5 border border-slate-900 rounded-md bg-cover bg-center
                                        relative overflow-hidden">
                                        <img v-if="emotion.frames?.[frameType]?.path"
                                            class="absolute top-0 brightness-[40%] left-0 w-full h-full object-cover"
                                            :src="convertFileSrc(emotion.frames?.[frameType]?.path)">
                                        <Icon v-if="emotion.frames?.[frameType]?.path" size="4rem"
                                            class="relative transition-opacity group-hover:opacity-100 z-10 w-full h-full opacity-0"
                                            name="material-symbols:edit-outline" />
                                        <p class="relative z-10 text-center">{{ frameType }}</p>
                                        <Icon class="relative z-10" size="4rem" name="material-symbols:upload"
                                            v-if="!emotion.frames?.[frameType]?.path" />
                                    </label>
                                </div>
                            </div>
                        </div>
                        <DialogFooter>
                            <DialogClose as-child>
                                <Button type="submit" @click="saveCharacterGlobally()" :disabled="isSaveDisabled"
                                    variant="secondary" form="dialogForm">
                                    Save changes
                                </Button>
                            </DialogClose>
                        </DialogFooter>
                    </DialogScrollContent>
                </Dialog>
            </MenubarMenu>
        </Menubar>
    </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { save, open } from '@tauri-apps/plugin-dialog'
import { Input } from '@/components/ui/input'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from '@/components/ui/toast/use-toast'
import { convertFileSrc } from '@tauri-apps/api/core'

const { toast } = useToast()
const { saveCharacter, loadCharacter, getRecentCharacters } = useCharacter()
const currentWindow = getCurrentWindow()
const characterName = ref("")

const frameTypes = ["Idle", "Blinking idle", "Speaking", "Speaking Blinking"] as const;
type FrameType = typeof frameTypes[number];

interface EmotionFrame {
    frame_type: FrameType;
    path: string;
}

interface Emotion {
    name: string;
    frames: Record<FrameType, EmotionFrame> | null;
}

const emotions = ref<Emotion[]>([]);
const newCharacter = computed(() => ({
    name: characterName.value,
    emotions: emotions.value,
}));


function addNewEmotion() {
    const newEmotion: Emotion = {
        name: emotions.value.length === 0 ? "Default Emotion" : "",
        frames: null,
    };
    emotions.value.push(newEmotion);
}

function removeEmotion(index: number) {
    emotions.value.splice(index, 1);
}

async function loadCharacterFromFile() {
    const characterPath = await open({
        filters: [{
            name: "Basedlive character file",
            extensions: ["basedchar"]
        }]
    })

    if (characterPath) {
        loadCharacter(characterPath)
    }
}

const recentCharacters = computed(() => getRecentCharacters())

async function loadRecentCharacter(name: string) {
    try {
        toast({
            description: `Loading character: ${name}`,
        })
    } catch (error) {
        console.error("Error when loading recent character:", error)
        toast({
            description: 'Error when loading character.',
            variant: 'destructive'
        })
    }
}

function validateCharacter(character: { name: any; emotions: string | any[] }) {
    return character.name && 
           character.emotions?.length > 0 && 
           character.emotions[0]?.frames
}

async function selectImage(emotionIndex: number, frameType: FrameType) {
    try {
        const selectedFile = await open({
            filters: [
                { name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'gif'] },
            ],
            multiple: false,
        });

        if (!selectedFile) return;

        if (!emotions.value[emotionIndex].frames) {
            emotions.value[emotionIndex].frames = {} as Record<FrameType, EmotionFrame>;
        }

        emotions.value[emotionIndex].frames[frameType] = {
            frame_type: frameType,
            path: selectedFile,
        };
    } catch (error) {
        console.error("Error when sleecting image:", error);
        toast({
            description: 'Error when selecting image.',
        });
    }
}

const isSaveDisabled = computed(() => {
    return !emotions.value.some(emotion =>
        emotion.frames &&
        frameTypes.every(frameType => emotion.frames![frameType]?.path) && characterName.value
    );
});

function saveCharacterGlobally() {
    console.log("Saving character:", newCharacter.value);
    saveCharacter(newCharacter.value);
}

async function saveCharacterAs() {
    if (!characterName.value || !emotions.value.length) return;
    const outputPath = await save({
        filters: [
            {
                name: "Basedlive character file",
                extensions: ['basedchar'],
            },
        ],
    });

    if (!outputPath) {
        toast({
            description: 'Error when selecting path to salve file.',
        });
        return;
    }

    try {
        await invoke('save_character', {
            data: newCharacter.value,
            outputPath: outputPath,
        });

        toast({
            description: 'Character saved with success!',
        });
    } catch (error) {
        console.error("Error when saving character:", error);
        toast({
            description: 'Error when saving character.',
        });
    }
}
</script>
