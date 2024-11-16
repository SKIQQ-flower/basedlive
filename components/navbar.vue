<template>
    <Menubar class="text-xl">
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
                    <MenubarItem>
                        Save character <MenubarShortcut>Ctrl+Shift+N</MenubarShortcut>
                    </MenubarItem>
                    <MenubarItem>New Window</MenubarItem>
                </MenubarContent>
                <DialogScrollContent class="sm:max-w-[750px]">
                    <DialogHeader>
                        <DialogTitle>Creating new character</DialogTitle>
                        <DialogDescription>
                            Select your character portrait for a specific audio level, or emotion. We do the rest.
                        </DialogDescription>
                    </DialogHeader>
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
                            <div class="w-40 h-40">
                                <label
                                    class="w-full cursor-pointer h-full flex flex-col items-center justify-center p-5 border border-slate-900 rounded-md"
                                    :for="'emotion' + index + 'fu'">
                                    <Icon size="4rem" name="material-symbols:upload" />
                                    <p>Idle</p>
                                </label>
                                <input class="hidden" :id="'emotion' + index + 'fu'" type="file" />
                            </div>
                            <div class="w-40 h-40">
                                <label
                                    class="w-full cursor-pointer h-full flex flex-col items-center justify-center p-5 border border-slate-900 rounded-md"
                                    :for="'emotion' + index + 'fu_blink'">
                                    <Icon size="4rem" name="material-symbols:upload" />
                                    <p class="text-center">Idle blinking</p>
                                </label>
                                <input class="hidden" :id="'emotion' + index + 'fu_blink'" type="file" />
                            </div>
                            <div class="w-40 h-40">
                                <label
                                    class="w-full cursor-pointer h-full flex flex-col items-center justify-center p-5 border border-slate-900 rounded-md"
                                    :for="'emotion' + index + 'fu_blink'">
                                    <Icon size="4rem" name="material-symbols:upload" />
                                    <p class="text-center">Speaking</p>
                                </label>
                                <input class="hidden" :id="'emotion' + index + 'fu_blink'" type="file" />
                            </div>
                            <div class="w-40 h-40">
                                <label
                                    class="w-full cursor-pointer h-full flex flex-col items-center justify-center p-5 border border-slate-900 rounded-md"
                                    :for="'emotion' + index + 'fu_blink'">
                                    <Icon size="4rem" name="material-symbols:upload" />
                                    <p class="text-center">Speaking Blinking</p>
                                </label>
                                <input class="hidden" :id="'emotion' + index + 'fu_blink'" type="file" />
                            </div>
                        </div>
                    </div>
                    <DialogFooter>
                        <Button type="submit" variant="secondary" form="dialogForm">
                            Save changes
                        </Button>
                    </DialogFooter>
                </DialogScrollContent>
            </Dialog>
        </MenubarMenu>
    </Menubar>
</template>

<script setup lang="ts">
import { Input } from '@/components/ui/input'
import { ref } from "vue";

enum frameType {
    BLINKING_IDLE,
    IDLE,
    SPEAKING,
    BLEAKING_SPEAKING,
}

interface emotionFrame {
    type: frameType;
    path: string;
}

interface emotion {
    name: string;
    frames: emotionFrame[] | null;
}

const emotions = ref<emotion[]>([]);

function addNewEmotion() {
    const newEmotion: emotion = {
        name: emotions.value.length === 0 ? "default" : "",
        frames: null,
    };
    emotions.value.push(newEmotion);
}

function removeEmotion(index: number) {
    emotions.value.splice(index, 1);
}

</script>
