<!-- components/Settings.vue -->
<template>
  <div class="settings-container">
    <n-card title="Ollama 配置">
      <n-form ref="formRef" :model="settings" label-placement="left" label-width="220"
        require-mark-placement="right-hanging">
        <n-space vertical>

          <!-- 在基本设置组中添加 -->
          <n-card title="基本设置" embedded>
            <!-- 现有设置保持不变 -->

            <n-form-item label-placement="left">
              <template #label>
                <span>开机自启动</span>
              </template>
              <n-switch v-model:value="settings.autoStart" />
            </n-form-item>

            <n-form-item label-placement="left">
              <template #label>
                <span>启动时运行 Ollama</span>
              </template>
              <n-switch v-model:value="settings.autoRunOllama" />
            </n-form-item>
          </n-card>
          <!-- 基本设置组 -->
          <n-card title="基本设置" embedded>
            <n-form-item label-placement="left">
              <template #label>
                调试模式
                <n-tag type="info" size="small" class="env-tag">OLLAMA_DEBUG</n-tag>
              </template>
              <n-switch v-model:value="settings.debug">
                <template #checked>开启</template>
                <template #unchecked>关闭</template>
              </n-switch>
            </n-form-item>

            <n-form-item>
              <template #label>
                服务器地址
                <n-tag type="info" size="small" class="env-tag">OLLAMA_HOST</n-tag>
              </template>
              <n-input v-model:value="settings.host" placeholder="默认: 127.0.0.1:11434" />
            </n-form-item>

            <n-form-item>
              <template #label>
                模型缓存时间
                <n-tag type="info" size="small" class="env-tag">OLLAMA_KEEP_ALIVE</n-tag>
              </template>
              <n-input-group>
                <n-input v-model:value="settings.keepAlive" placeholder="默认: 5" />
                <n-select v-model:value="settings.keepAliveUnit" :options="timeUnitOptions" style="width: 100px" />
              </n-input-group>
            </n-form-item>
          </n-card>

          <!-- 性能设置组 -->
          <n-card title="性能设置" embedded>
            <n-form-item>
              <template #label>
                最大加载模型数
                <n-tag type="info" size="small" class="env-tag">OLLAMA_MAX_LOADED_MODELS</n-tag>
              </template>
              <n-input-number v-model:value="settings.maxLoadedModels" placeholder="每个GPU可加载的最大模型数" />
            </n-form-item>

            <n-form-item>
              <template #label>
                最大队列长度
                <n-tag type="info" size="small" class="env-tag">OLLAMA_MAX_QUEUE</n-tag>
              </template>
              <n-input-number v-model:value="settings.maxQueue" placeholder="最大请求队列长度" />
            </n-form-item>

            <n-form-item>
              <template #label>
                最大并行请求数
                <n-tag type="info" size="small" class="env-tag">OLLAMA_NUM_PARALLEL</n-tag>
              </template>
              <n-input-number v-model:value="settings.numParallel" placeholder="最大并行请求数" />
            </n-form-item>

            <n-form-item>
              <template #label>
                Flash Attention
                <n-tag type="info" size="small" class="env-tag">OLLAMA_FLASH_ATTENTION</n-tag>
              </template>
              <n-switch v-model:value="settings.flashAttention" />
            </n-form-item>
          </n-card>

          <!-- 存储设置组 -->
          <n-card title="存储设置" embedded>
            <n-form-item>
              <template #label>
                模型存储路径
                <n-tag type="info" size="small" class="env-tag">OLLAMA_MODELS</n-tag>
              </template>
              <n-input-group>
                <n-input v-model:value="settings.models" placeholder="模型存储路径" />
                <n-button @click="selectModelPath">选择目录</n-button>
              </n-input-group>
            </n-form-item>

            <n-form-item>
              <template #label>
                临时文件路径
                <n-tag type="info" size="small" class="env-tag">OLLAMA_TMPDIR</n-tag>
              </template>
              <n-input-group>
                <n-input v-model:value="settings.tmpdir" placeholder="临时文件存储路径" />
                <n-button @click="selectTmpPath">选择目录</n-button>
              </n-input-group>
            </n-form-item>
          </n-card>

          <!-- 高级设置组 -->
          <n-card title="高级设置" embedded>
            <n-form-item>
              <template #label>
                GPU设备选择
                <n-tag type="info" size="small" class="env-tag">ONEAPI_DEVICE_SELECTOR</n-tag>
              </template>
              <n-input v-model:value="settings.deviceSelector" placeholder="level_zero:0, level_zero:1..." />
            </n-form-item>
            <n-form-item>
              <template #label>
                禁用模型清理
                <n-tag type="info" size="small" class="env-tag">OLLAMA_NOPRUNE</n-tag>
              </template>
              <n-switch v-model:value="settings.noprune" />
            </n-form-item>

            <n-form-item>
              <template #label>
                允许的源<br />
                <n-tag type="info" size="small" class="env-tag">OLLAMA_ORIGINS</n-tag>
              </template>
              <n-input v-model:value="settings.origins" placeholder="允许的源（逗号分隔）" />
            </n-form-item>

            <n-form-item>
              <template #label>
                跨 GPU 调度
                <n-tag type="info" size="small" class="env-tag">OLLAMA_SCHED_SPREAD</n-tag>
              </template>
              <n-switch v-model:value="settings.schedSpread" />
            </n-form-item>

            <n-form-item>
              <template #label>
                K/V 缓存类型
                <n-tag type="info" size="small" class="env-tag">OLLAMA_KV_CACHE_TYPE</n-tag>
              </template>
              <n-select v-model:value="settings.kvCacheType" :options="kvCacheOptions" />
            </n-form-item>

            <n-form-item>
              <template #label>
                LLM 库
                <n-tag type="info" size="small" class="env-tag">OLLAMA_LLM_LIBRARY</n-tag>
              </template>
              <n-input v-model:value="settings.llmLibrary" placeholder="设置 LLM 库以绕过自动检测" />
            </n-form-item>

            <n-form-item>
              <template #label>
                GPU 内存预留
                <n-tag type="info" size="small" class="env-tag">OLLAMA_GPU_OVERHEAD</n-tag>
              </template>
              <n-input-number v-model:value="settings.gpuOverhead" placeholder="每个 GPU 预留的 VRAM (字节)" />
            </n-form-item>

            <n-form-item>
              <template #label>
                加载超时时间
                <n-tag type="info" size="small" class="env-tag">OLLAMA_LOAD_TIMEOUT</n-tag>
              </template>
              <n-input-group>
                <n-input v-model:value="settings.loadTimeout" placeholder="默认: 5" />
                <n-select v-model:value="settings.loadTimeoutUnit" :options="timeUnitOptions" style="width: 100px" />
              </n-input-group>
            </n-form-item>
          </n-card>
        </n-space>

        <n-space justify="end" style="margin-top: 24px">
          <n-button @click="resetSettings">
            重置
          </n-button>
          <n-button type="primary" @click="saveSettings">
            保存
          </n-button>
        </n-space>
      </n-form>
    </n-card>
  </div>
</template>

<script setup>
import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart';
import { ref } from 'vue'
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event'
import * as store from "../store";
import { useNotification } from 'naive-ui'
import { open } from '@tauri-apps/plugin-dialog';
const notification = useNotification();
const formRef = ref(null)
const defSettings={
  autoStart: false,
  autoRunOllama: true,
  debug: false,
  host: '127.0.0.1:11434',
  keepAlive: '5',
  keepAliveUnit: 'm',
  maxLoadedModels: null,
  maxQueue: null,
  models: '',
  numParallel: null,
  noprune: false,
  origins: '',
  schedSpread: false,
  tmpdir: '',
  kvCacheType: 'f16',
  llmLibrary: '',
  gpuOverhead: null,
  loadTimeout: '5',
  loadTimeoutUnit: 'm',
  deviceSelector:''
}
const settings = ref(defSettings)

const rules = {
  // ollamaPath: {
  //   required: true,
  //   message: '请输入Ollama安装路径',
  //   trigger: ['blur', 'input']
  // },
  // pythonPath: {
  //   required: true,
  //   message: '请输入Python路径',
  //   trigger: ['blur', 'input']
  // }
}

async function loadSettings() {
  try {
    const savedSettings = await store.loadSettings()
    settings.value = savedSettings ?? defSettings

  } catch (error) {
    console.error('加载设置失败:', error)
  }
}

async function saveSettings() {
  try {
    await formRef.value?.validate()
    await store.saveSettings(settings.value)
    await saveEvn();
    notification.success({
      content: `保存设置成功`,
      duration:3000
    })
  } catch (error) {
    notification.warning({
      content: `保存设置失败: ${error}`,
      duration:3000
    })
    console.error('保存设置失败:', error)
  }

  try{
    if (settings.value.autoStart) {
      await enable();
    } else {
      await disable();
    }
  }catch(e){}
}

function resetSettings() {
  settings.value = defSettings
}

const timeUnitOptions = [
  { label: '分钟', value: 'm' },
  { label: '小时', value: 'h' },
  { label: '秒', value: 's' }
]

const kvCacheOptions = [
  { label: 'f16', value: 'f16' },
  { label: 'f32', value: 'f32' },
  { label: 'q8_0', value: 'q8_0' }
]

async function selectModelPath() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: settings.value.models
    })
    if (selected) {
      settings.value.models = selected
    }
  } catch (error) {
    console.error('选择目录失败:', error)
  }
}

async function selectTmpPath() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: settings.value.tmpdir
    })
    if (selected) {
      settings.value.tmpdir = selected
    }
  } catch (error) {
    console.error('选择目录失败:', error)
  }
}


onMounted(() => {
  loadSettings()
  // resetSettings()
})

async function saveEvn() {
  const envSettings = {
    ONEAPI_DEVICE_SELECTOR: settings.value.deviceSelector|| '',
    OLLAMA_DEBUG: settings.value.debug ? '1' : '0',
    OLLAMA_HOST: settings.value.host,
    OLLAMA_KEEP_ALIVE: settings.value.keepAlive ? `${settings.value.keepAlive}${settings.value.keepAliveUnit}` : '',
    OLLAMA_MAX_LOADED_MODELS: settings.value.maxLoadedModels?.toString() || '',
    OLLAMA_MAX_QUEUE: settings.value.maxQueue?.toString() || '',
    OLLAMA_MODELS: settings.value.models,
    OLLAMA_NUM_PARALLEL: settings.value.numParallel?.toString() || '',
    OLLAMA_NOPRUNE: settings.value.noprune ? '1' : '',
    OLLAMA_ORIGINS: settings.value.origins,
    OLLAMA_SCHED_SPREAD: settings.value.schedSpread ? '1' : '',
    OLLAMA_TMPDIR: settings.value.tmpdir,
    OLLAMA_FLASH_ATTENTION: settings.value.flashAttention ? '1' : '',
    OLLAMA_KV_CACHE_TYPE: settings.value.kvCacheType,
    OLLAMA_LLM_LIBRARY: settings.value.llmLibrary,
    OLLAMA_GPU_OVERHEAD: settings.value.gpuOverhead?.toString() || '',
    OLLAMA_LOAD_TIMEOUT: settings.value.loadTimeout ? `${settings.value.loadTimeout}${settings.value.loadTimeoutUnit}` : ''
  }
  await store.saveEnvVariables(envSettings);
}
</script>

<style scoped>
.settings-container {
  padding: 24px;
  max-width: 800px;
  margin: 0 auto;
}
</style>