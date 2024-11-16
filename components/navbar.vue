<template>
    <div class="flex flex-row bg-slate-950">
        <UPopover>
            <UButton class="rounded-none rounded-bl-xl text-md" color="white" label="Open">
                <Icon name="mdi:file-outline" /> File
            </UButton>

            <template #panel>
                <div class="p-4">
                    <Placeholder class="h-20 w-48" />
                </div>
            </template>
        </UPopover>
        <UPopover>
            <UButton class="rounded-none text-md" color="white" label="Open">
                <Icon name="mdi:file-outline" /> File
            </UButton>

            <template #panel>
                <div class="p-4">
                    <Placeholder class="h-20 w-48" />
                </div>
            </template>
        </UPopover>
    </div>
</template>