import { ref } from 'vue'

// 单例 toast：message + 显示标志 + 自动隐藏计时器
const message = ref('')
const visible = ref(false)
let timer: ReturnType<typeof setTimeout> | undefined

export function useToast() {
  function toast(msg: string) {
    message.value = msg
    visible.value = true
    clearTimeout(timer)
    timer = setTimeout(() => { visible.value = false }, 1800)
  }
  return { message, visible, toast }
}
