<template>
  <Navbar/>
  <div class="w-full h-full flex flex-col items-center justify-center overflow-auto">
    <NuxtPage />
  </div>
  <Toaster />
</template>

<script lang="ts" setup>
/*
if (localStorage.getItem("welcomed") == null) {
  navigateTo("/welcome")
}
*/
</script>

<style>
body,
#__nuxt {
  @apply bg-slate-950 text-white flex flex-col w-full h-dvh rounded-2xl border-slate-800 border overflow-hidden
}


body {
  @apply bg-transparent;
}

.page-enter-active,
.page-leave-active {
  transition: all 0.2s;
}

.page-enter-from,
.page-leave-to {
  opacity: 0;
  transform: translateX(5);
}
</style>
