<!-- components/OllamaControl.vue -->
<template>
  <div class="control-container">
    <n-card title="Ollama 控制面板">
      <n-space vertical>
        <n-alert :type="ollamaRunning ? 'success' : 'warning'">
          Ollama 当前状态: {{ ollamaRunning ? '运行中' : '未运行' }}
        </n-alert>
        <n-space>

          <n-button :type="ollamaRunning ? 'error' : 'primary'" :loading="isLoading" @click="toggleOllama">
            {{ ollamaRunning ? '停止 Ollama' : '启动 Ollama' }}
          </n-button>

          <n-button type="info" @click="tryUpdate">
            更新 Ollama
          </n-button>

          <n-button type="info" @click="openCmd">
            打开 CMD
          </n-button>

          <n-button type="info" @click="openPowerShell">
            打开 PowerShell
          </n-button>
        </n-space>
        <n-divider />

        <n-collapse>
          <n-collapse-item title="运行日志" name="logs">
            <n-scrollbar style="height: 100%">
              <n-log trim :lines="ollamaStore.logs" :rows="13" />
            </n-scrollbar>
          </n-collapse-item>
        </n-collapse>
      </n-space>
    </n-card>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event'
import { loadSettings } from '../store';
import { useOllamaStore } from "../stores/ollama";
import { useNotification, useDialog,useMessage  } from 'naive-ui'
const dialog = useDialog()
const notification = useNotification();
const ollamaStore = useOllamaStore();
const ollamaRunning = ref(false)
const isLoading = ref(false)
const out = listen("run-ollama-output", (ev) => { ollamaStore.appendLog(ev.payload) });
const err = listen("run-ollama-error", (ev) => ollamaStore.appendLog(ev.payload));
const message = useMessage()

async function stopOllama() {

}

async function tryUpdate() {
  if (ollamaRunning.value) {
    dialog.warning({
      title: '警告',
      content: '服务正在运行，升级时请关闭服务',
      positiveText: '升级并关闭服务',
      negativeText: '稍后升级',
      draggable: true,
      onPositiveClick: async () => {
        await toggleOllama()
        await upgradeEnv()
        await toggleOllama()
      },
      onNegativeClick: () => {
        message.error('取消更新')
      }
    })
  }else{
    await upgradeEnv()
  }
}
async function upgradeEnv() {

  ollamaStore.appendLog('更新 ipex-llm...')
  let out = null
  let err = null
  out = listen("run-pip-output", (ev) => ollamaStore.appendLog(ev.payload));
  err = listen("run-pip-error", (ev) => ollamaStore.appendLog(ev.payload));
  await invoke('install_deps')

  out = listen("install-ollama-output", (ev) => ollamaStore.appendLog(ev.payload));
  err = listen("install-ollama-error", (ev) => ollamaStore.appendLog(ev.payload));
  await invoke('install_ollama')
  ollamaStore.appendLog('更新 ipex-llm 完成')
}
async function toggleOllama() {
  isLoading.value = true
  ollamaStore.clearLogs();
  try {
    if (ollamaRunning.value) {
      await invoke('stop_ollama')
    } else {
      await invoke('run_ollama')
    }
    await checkStatus()
  } catch (error) {
    notification.warning({
      content: `Ollama操作失败: ${error}`,
      duration: 3000
    })
    console.error('Ollama操作失败:', error)
  } finally {
    isLoading.value = false
  }
}

async function checkStatus() {
  try {
    const status = await invoke('is_ollama_running')
    ollamaRunning.value = status
  } catch (error) {
    console.error('状态检查失败:', error)
    notification.warning({
      content: `状态检查失败: ${error}`,
      duration: 3000
    })
  }
}

async function openCmd() {
  try {
    await invoke('open_terminal', { terminalType: 'cmd' })
  } catch (error) {
    notification.warning({
      content: `打开CMD失败: ${error}`,
      duration: 3000
    })
    console.error('打开CMD失败:', error)
  }
}

async function openPowerShell() {
  try {
    await invoke('open_terminal', { terminalType: 'powershell' })
  } catch (error) {
    notification.warning({
      content: `打开PowerShell失败: ${error}`,
      duration: 3000
    })
    console.error('打开PowerShell失败:', error)
  }
}

onMounted(async () => {
  await checkStatus()
  statusCheckInterval = setInterval(checkStatus, 5000)
  const settings = await loadSettings()
  console.log(ollamaStore.init);

  if (ollamaStore.init) {
    return;
  }
  if (settings.autoUpdateOllama) {
    await upgradeEnv()
  }
  if ((!settings) || settings.autoRunOllama) {
    if (!ollamaRunning.value) {
      await toggleOllama()
    }
  }
  ollamaStore.inited();
})

onUnmounted(() => {
  if (statusCheckInterval) {
    clearInterval(statusCheckInterval)
  }
})
</script>

<style scoped>
.control-container {
  padding: 24px;
  max-width: 800px;
  margin: 0 auto;
}
</style>