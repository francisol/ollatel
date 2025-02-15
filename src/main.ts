import { createApp } from "vue";
import App from "./App.vue";
import "./style.css";
import { createPinia } from "pinia";
import router from './router'
import { useOllamaStore } from "./stores/ollama";
const pinia = createPinia()
const app=createApp(App).use(pinia)
app.provide('ollamaStore',useOllamaStore())
app.use(router).mount("#app");
