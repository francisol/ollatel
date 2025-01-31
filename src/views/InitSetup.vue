<!-- components/InitSetup.vue -->
<template>
  <div class="container">

  </div>
  <div class="init-container">
    <n-card title="环境初始化" class="init-card">
      <n-steps :current="currentStep" :status="currentStatus">
        <n-step title="检查Python" description="检查Python环境" />
        <n-step title="安装pip" description="安装pip包" />
        <n-step title="安装依赖" description="安装必要的pip包" />
        <n-step title="安装Ollama" description="下载并安装Ollama" />
      </n-steps>

      <div class="status-display">
        <n-alert :type="statusType" v-if="statusMessage">
          {{ statusMessage }}
        </n-alert>
      </div>

      <div class="action-area">
        <n-button type="primary" :loading="isInitializing" :disabled="isInitializing" @click="startInitialization">
          开始初始化
        </n-button>
      </div>
    </n-card>
    <n-divider />
    <n-card>

    <n-collapse>
      <n-collapse-item title="安装日志" name="1">
        <div> <n-log trim :log="logs " :rows="10"></n-log>
        </div>
      </n-collapse-item>
    </n-collapse>
  </n-card>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import { initDone } from '../store';
import { useRouter } from 'vue-router';
import { useNotification } from 'naive-ui'
const notification = useNotification();

// import { listen } from '@tauri-apps/api/event'
const currentStep = ref(0)
const currentStatus = ref('process')
const statusMessage = ref('')
const statusType = ref('info')
const logs = ref('')
const isInitializing = ref(false)
const router = useRouter()
function appendLog(line) {
  logs.value+="\n"
  logs.value+=line
}

async function startInitialization() {
  isInitializing.value = true
  let out=null
  let err=null
  try {
    await invoke('init_env')
    // 检查Python
    currentStep.value = 1
    statusMessage.value = '正在检查Python环境...'
    await invoke('install_python')

    // 安装依赖
    currentStep.value = 2
    statusMessage.value = '正在安装pip...'
    out=listen("install-pip-output",(ev)=>{appendLog(ev.payload)    });
    err=listen("install-pip-error",(ev)=>appendLog(ev.payload) );
    await invoke('install_pip')
    
    // 安装依赖
    out=listen("run-pip-output",(ev)=>appendLog(ev.payload) );
    err=listen("run-pip-error",(ev)=>appendLog(ev.payload) );
    currentStep.value = 3
    statusMessage.value = '正在安装依赖...'
    await invoke('install_deps')

    // 安装Ollama
    out=listen("install-ollama-output",(ev)=>appendLog(ev.payload) );
      err=listen("install-ollama-error",(ev)=>appendLog(ev.payload) );
    currentStep.value = 4
    statusMessage.value = '正在安装Ollama...'
    await invoke('install_ollama')

    statusType.value = 'success'
    statusMessage.value = '初始化完成'
    currentStatus.value = 'finish'
    await initDone();
    // 刷新页面或通知父组件重新检查环境
    // window.location.reload()
    router.push("/")
  } catch (error) {
    notification.warning({
          content: error
        })
    
    statusType.value = 'error'
    statusMessage.value = `初始化失败: ${error}`
    currentStatus.value = 'error'
  } finally {
    isInitializing.value = false
    // if (out) {
    //   (await out)();
    // }
    // if (err) {
    //   (await err)();
    // }
  }
}
</script>

<style scoped>
.init-container {
  /* padding: 24px; */
}

.init-card {
  /* max-width: 600px; */
  margin: 0 auto;
}

.status-display {
  margin: 20px 0;
}

.action-area {
  margin-top: 20px;
  text-align: center;
}
</style>