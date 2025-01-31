import { defineStore } from 'pinia'

export const useOllamaStore = defineStore('logs', {
  state: () => {
    return { logs: [""],init:false }
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
    inited(){
      this.init=true
    },
    clearLogs(){
      this.logs=[];
    }
  },
  getters: {
  },
})