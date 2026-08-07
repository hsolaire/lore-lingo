import { readFile } from 'node:fs/promises'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

async function readSource(path: string): Promise<string> {
  return readFile(resolve(process.cwd(), path), 'utf8')
}

describe('共享应用 Provider', () => {
  it.each(['src/App.vue', 'src/SubtitleApp.vue', 'src/SettingsApp.vue'])(
    '%s 使用统一的 AppProviders 根组件',
    async (path) => {
      const source = await readSource(path)

      expect(source).toContain("import AppProviders from '@/components/AppProviders.vue'")
      expect(source).toMatch(/<AppProviders>[\s\S]*<\/AppProviders>/)
    },
  )

  it('业务组合式函数不直接访问 Tauri 窗口 API', async () => {
    const sources = await Promise.all([
      readSource('src/composables/useSubtitleBridge.ts'),
      readSource('src/composables/useTranslator.ts'),
    ])

    for (const source of sources) {
      expect(source).not.toMatch(/@tauri-apps\/api\/(window|webviewWindow)/)
    }
  })

  it('全局样式提供设置页所需的语义令牌和系统降级', async () => {
    const source = await readSource('src/styles/global.css')

    expect(source).toContain('--surface-muted:')
    expect(source).toContain('--focus:')
    expect(source).toContain('@media (prefers-reduced-transparency: reduce)')
    expect(source).toContain('@media (prefers-reduced-motion: reduce)')
  })
})
