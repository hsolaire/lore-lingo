import { ref } from 'vue'
import { useToast } from './useToast'

export interface SubItem {
  id: number
  fmt: string
  name: string
  /** 索引状态文案，如「1,842 条 · 已索引」或「索引中…」 */
  status: string
}

let nextId = 100

// 单例字幕库：拖放/选择加载 + 移除 + 模拟索引。
const items = ref<SubItem[]>([
  { id: 1, fmt: 'SRT', name: 'Interstellar.2014.en.srt', status: '1,842 条 · 已索引' },
  { id: 2, fmt: 'ASS', name: 'YourName.jp-zh.ass', status: '1,204 条 · 已索引' },
])

export function useSubtitles() {
  const { toast } = useToast()

  function addFiles(files: FileList | File[]) {
    const list = Array.from(files)
    list.forEach((f) => {
      const ext = (f.name.split('.').pop() || 'srt').toUpperCase()
      const id = nextId++
      const item: SubItem = { id, fmt: ext, name: f.name, status: '索引中…' }
      items.value.push(item)
      const cnt = 800 + Math.floor(f.size % 1500)
      setTimeout(() => {
        const found = items.value.find((it) => it.id === id)
        if (found) found.status = `${cnt.toLocaleString()} 条 · 已索引`
      }, 900)
    })
    if (list.length) toast(`已加载 ${list.length} 个字幕文件`)
  }

  function remove(id: number) {
    items.value = items.value.filter((it) => it.id !== id)
  }

  return { items, addFiles, remove }
}
