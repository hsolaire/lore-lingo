import { ref } from 'vue'

// 单例设置抽屉状态：打开悬浮字幕时也需要关闭主面板设置。
const drawerOpen = ref(false)

export function useSettings() {
  function toggleDrawer() {
    drawerOpen.value = !drawerOpen.value
  }
  function closeDrawer() {
    drawerOpen.value = false
  }
  return { drawerOpen, toggleDrawer, closeDrawer }
}
