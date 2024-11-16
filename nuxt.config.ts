export default defineNuxtConfig({
  // (optional) Enable the Nuxt devtools
  devtools: { enabled: true },
  app: {
    pageTransition: { name: 'page', mode: 'out-in' }
  },

  // Enable SSG
  ssr: false,

  // Enables the development server to be discoverable by other devices when running on iOS physical devices
  devServer: { host: process.env.TAURI_DEV_HOST || 'localhost' },

  vite: {
    // Better support for Tauri CLI output
    clearScreen: false,
    // Enable environment variables
    // Additional environment variables can be found at
    // https://v2.tauri.app/reference/environment-variables/
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      // Tauri requires a consistent port
      strictPort: true,
    },
  },

  modules: ['@vueuse/nuxt', '@nuxtjs/color-mode', '@nuxt/icon', '@nuxt/ui', '@nuxtjs/tailwindcss'],
  colorMode: {
    classSuffix: "",
  },
  ui: {
    primary: 'slate',
    popover: {
      ring: '',
    }
  }
});