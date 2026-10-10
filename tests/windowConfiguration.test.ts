import { readFile } from 'node:fs/promises'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

type Capability = {
  windows: string[]
  permissions: string[]
}

async function readJson<T>(path: string): Promise<T> {
  return JSON.parse(await readFile(new URL(path, import.meta.url), 'utf8')) as T
}

describe('合并悬浮字幕构建配置', () => {
  it('Vite 包含独立设置入口', () => {
    return readFile(resolve(process.cwd(), 'vite.config.ts'), 'utf8').then((source) => {
      expect(source).toMatch(/main:\s*fileURLToPath\(new URL\('\.\/index\.html'/)
      expect(source).not.toMatch(/subtitle:\s*fileURLToPath/)
      expect(source).toMatch(/settings:\s*fileURLToPath\(new URL\('\.\/settings\.html'/)
    })
  })

  it('Tauri 声明默认隐藏的独立设置窗口', async () => {
    const config = await readJson<{ app: { windows: Array<Record<string, unknown>> } }>(
      '../src-tauri/tauri.conf.json',
    )
    const settings = config.app.windows.find((window) => window.label === 'settings')

    expect(settings).toMatchObject({
      url: 'settings.html',
      visible: false,
      decorations: false,
      width: 720,
      height: 612,
    })
    expect(config.app.windows.map(window => window.label)).toEqual(['main', 'settings'])
    expect(config.app.windows.find(window => window.label === 'main')).toMatchObject({
      alwaysOnTop: true, width: 900, minWidth: 520, minHeight: 104,
    })
  })

  it('三个窗口使用分离的最小权限能力文件', async () => {
    const [main, subtitle, settings] = await Promise.all([
      readJson<Capability>('../src-tauri/capabilities/main.json'),
      readJson<Capability>('../src-tauri/capabilities/subtitle.json'),
      readJson<Capability>('../src-tauri/capabilities/settings.json'),
    ])

    expect(main.windows).toEqual(['main'])
    expect(subtitle.windows).toEqual(['subtitle'])
    expect(settings.windows).toEqual(['settings'])
    expect(settings.permissions.some((permission) => permission.startsWith('shell:'))).toBe(false)
  })
})
