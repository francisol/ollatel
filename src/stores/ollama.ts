import { defineStore } from 'pinia'
import { invoke } from "@tauri-apps/api/core";

export const useOllamaStore = defineStore('logs', {
  state: () => {
    return { logs: [""],init:false,running:false }
  },
  // 也可以这样定义
  // state: () => ({ count: 0 })
  actions: {
    appendLog(log:string) {
      this.logs.push(log);
      if (this.logs.length > 100) {
        this.logs.splice(0, this.logs.length - 100);
      }    
    },
    async stopOllama(){
        await invoke('stop_ollama')
        this.running=false
    },
    async runOllama(){
        await invoke('run_ollama')
        this.running=true
    },
    async  checkStatus() {
        const status = await invoke('is_ollama_running')
        this.running=status as boolean

    },
    async  toggleOllama() {
      if (this.running) {
        await this.stopOllama()
      }else{
        await this.runOllama()
      }
    },
    inited(){
      this.init=true
    },
    setRunning(val:boolean){
      this.running=val
    },

    clearLogs(){
      this.logs=[];
    }
  },
  getters: {
  },
})