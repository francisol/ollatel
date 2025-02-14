<!-- App.vue -->
<template>
    <n-scrollbar style="max-height: 100%">
  <n-config-provider>
    <n-dialog-provider>
      <n-message-provider>
    <div class="content">
    <n-notification-provider>
    <n-layout>
      <n-layout-header bordered v-if="showHeader">
        <n-menu v-model:value="activeKey" mode="horizontal" :options="menuOptions" />
      </n-layout-header>
      
      <n-layout-content>
        <router-view></router-view>
      </n-layout-content>
    </n-layout>
    </n-notification-provider>
  </div>
  </n-message-provider>
    </n-dialog-provider>
  </n-config-provider>
  
  </n-scrollbar>
</template>

<script setup>
import { ref, computed, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'

const route = useRoute()
const router = useRouter()
const activeKey = ref(route.path.slice(1) || 'control')

const showHeader = computed(() => route.path !== '/init')

const menuOptions = [
  {
    label: 'Ollama控制',
    key: 'control'
  },
  {
    label: '设置',
    key: 'settings'
  }
]

watch(activeKey, (newKey) => {
  router.push(`/${newKey}`)
})
</script>
<style>
.content{
  padding: 24px  36px 10px 36px;
  height: 100%;
}
</style>