import { createRouter, createWebHashHistory } from 'vue-router'
import { isInited } from './store'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: '/',
      redirect: '/control'
    },
    {
      path: '/init',
      component: () => import('./views/InitSetup.vue')
    },
    {
      path: '/control',
      component: () => import('./views/OllamaControl.vue')
    },
    {
      path: '/settings',
      component: () => import('./views/Settings.vue')
    }
  ]
})

router.beforeEach(async (to, _from, next) => {
    if (to.path === '/init') {
      next()
      return
    }
  
    try {
      const status = await isInited()
      console.log(status);
      
      if (!status && to.path !== '/init') {
        next('/init')
      } else {
        next()
      }
    } catch (error) {
      console.error('检查环境失败:', error)
      next('/init')
    }
  })
  
  export default router