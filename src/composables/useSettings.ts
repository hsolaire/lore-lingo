import { ref } from 'vue'

// 单例设置抽屉状态：主面板标题栏「设置」按钮与「进入歌词条」需共享开合。
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
