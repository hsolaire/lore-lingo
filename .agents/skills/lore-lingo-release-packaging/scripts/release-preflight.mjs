#!/usr/bin/env node

import { execFileSync } from 'node:child_process'
import { existsSync, readFileSync } from 'node:fs'
import { platform as hostPlatform } from 'node:os'
import { resolve } from 'node:path'
import process from 'node:process'

const args = process.argv.slice(2)
const option = name => {
  const index = args.indexOf(name)
  return index >= 0 ? args[index + 1] : undefined
}

if (args.includes('--help')) {
  console.log(`用法: release-preflight.mjs [选项]

选项:
  --mode verify|release              默认 verify
  --platform auto|macos|windows|all  默认 auto；all 仅用于 verify
  --allow-dirty                      仅允许 verify 模式跳过干净工作区要求
  --help                             显示帮助`)
  process.exit(0)
}

const mode = option('--mode') ?? 'verify'
const requestedPlatform = option('--platform') ?? 'auto'
const allowDirty = args.includes('--allow-dirty')
const validModes = new Set(['verify', 'release'])
const validPlatforms = new Set(['auto', 'macos', 'windows', 'all'])

if (!validModes.has(mode) || !validPlatforms.has(requestedPlatform)) {
  console.error('参数错误：--mode 或 --platform 的值无效。使用 --help 查看用法。')
  process.exit(2)
}
if (mode === 'release' && allowDirty) {
  console.error('参数错误：release 模式禁止使用 --allow-dirty。')
  process.exit(2)
}

const normalizedHost = hostPlatform() === 'darwin'
  ? 'macos'
  : hostPlatform() === 'win32' ? 'windows' : 'unsupported'
const targetPlatform = requestedPlatform === 'auto' ? normalizedHost : requestedPlatform

if (mode === 'release' && targetPlatform === 'all') {
  console.error('参数错误：release 模式必须明确指定 macos 或 windows。')
  process.exit(2)
}

const root = process.cwd()
const results = []
const pass = message => results.push({ level: 'PASS', message })
const warn = message => results.push({ level: 'WARN', message })
const fail = message => results.push({ level: 'FAIL', message })
const requireFile = path => {
  if (existsSync(resolve(root, path))) pass(`存在 ${path}`)
  else fail(`缺少 ${path}`)
}
const commandOutput = (command, commandArgs) => {
  try {
    return execFileSync(command, commandArgs, {
      cwd: root,
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'pipe'],
    }).trim()
  } catch {
    return null
  }
}
const readJson = path => JSON.parse(readFileSync(resolve(root, path), 'utf8'))
const readCargoVersion = () => {
  const cargo = readFileSync(resolve(root, 'src-tauri/Cargo.toml'), 'utf8')
  return cargo.match(/^version\s*=\s*"([^"]+)"/m)?.[1]
}
const envPresent = name => typeof process.env[name] === 'string' && process.env[name].length > 0
let packageVersion

for (const path of [
  'package.json',
  'pnpm-lock.yaml',
  'src-tauri/Cargo.toml',
  'src-tauri/Cargo.lock',
  'src-tauri/tauri.conf.json',
  'src-tauri/Info.plist',
]) {
  requireFile(path)
}

if (!results.some(result => result.level === 'FAIL')) {
  const packageJson = readJson('package.json')
  const tauri = readJson('src-tauri/tauri.conf.json')
  const versions = {
    'package.json': packageJson.version,
    'Cargo.toml': readCargoVersion(),
    'tauri.conf.json': tauri.version,
  }
  packageVersion = packageJson.version
  const uniqueVersions = new Set(Object.values(versions))

  if (uniqueVersions.size === 1) pass(`版本一致：${packageJson.version}`)
  else fail(`版本不一致：${Object.entries(versions).map(([file, value]) => `${file}=${value}`).join(', ')}`)

  if (/^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$/.test(packageJson.version)) {
    pass(`版本符合 SemVer：${packageJson.version}`)
  } else {
    fail(`版本不符合 SemVer：${packageJson.version}`)
  }

  if (mode === 'release' && packageJson.version === '0.0.0') {
    fail('正式发布版本不能为 0.0.0')
  } else if (packageJson.version === '0.0.0') {
    warn('当前版本为 0.0.0，只适合开发验证')
  }

  if (typeof tauri.identifier === 'string' && !tauri.identifier.endsWith('.app')) {
    pass(`应用标识符可用于 macOS bundle：${tauri.identifier}`)
  } else {
    fail('Tauri identifier 不应以 .app 结尾；这会与 macOS bundle 扩展冲突')
  }

  if (tauri.app?.security?.csp) pass('生产 CSP 已配置')
  else fail('Tauri CSP 当前为空；正式打包前必须配置最小生产 CSP')

  if (targetPlatform === 'macos' || targetPlatform === 'all') {
    const minimumMacos = tauri.bundle?.macOS?.minimumSystemVersion
    if (minimumMacos === '13.0') pass('macOS 最低版本为 13.0')
    else fail('bundle.macOS.minimumSystemVersion 应明确设置为 13.0')
  }

  if (targetPlatform === 'windows' || targetPlatform === 'all') {
    const webviewMode = tauri.bundle?.windows?.webviewInstallMode?.type
    if (webviewMode) pass(`Windows WebView2 模式已配置：${webviewMode}`)
    else fail('尚未明确 bundle.windows.webviewInstallMode')
  }
}

for (const lockfile of ['package-lock.json', 'yarn.lock', 'npm-shrinkwrap.json']) {
  if (existsSync(resolve(root, lockfile))) fail(`发现额外锁文件 ${lockfile}，项目只允许 pnpm-lock.yaml`)
}

const status = commandOutput('git', ['status', '--porcelain'])
if (status === null) {
  fail('无法读取 Git 工作区状态')
} else if (status.length === 0) {
  pass('Git 工作区干净')
} else if (mode === 'verify' && allowDirty) {
  warn('工作区存在改动；已通过 --allow-dirty 允许本次验证')
} else {
  fail('Git 工作区不干净')
}

if (mode === 'release') {
  const branch = commandOutput('git', ['branch', '--show-current'])
  if (branch === 'main') pass('当前分支为 main')
  else fail(`正式发布必须位于 main，当前分支为 ${branch ?? '未知'}`)

  const expectedTag = packageVersion ? `v${packageVersion}` : null
  const tags = commandOutput('git', ['tag', '--points-at', 'HEAD'])?.split('\n') ?? []
  if (expectedTag && tags.includes(expectedTag)) pass(`HEAD 具有版本标签 ${expectedTag}`)
  else fail(`HEAD 缺少与文件版本一致的标签 ${expectedTag ?? '未知'}`)
}

for (const [command, commandArgs] of [
  ['node', ['--version']],
  ['pnpm', ['--version']],
  ['cargo', ['--version']],
  ['rustc', ['--version']],
]) {
  const version = commandOutput(command, commandArgs)
  if (version) pass(`${command} 可用：${version}`)
  else fail(`${command} 不可用`)
}

if (normalizedHost === 'unsupported') {
  fail('当前宿主不是受支持的 macOS 或 Windows 发布环境')
} else if (targetPlatform !== 'all' && targetPlatform !== normalizedHost) {
  fail(`目标平台为 ${targetPlatform}，当前宿主为 ${normalizedHost}；必须使用原生宿主构建`)
} else if (targetPlatform === 'all') {
  warn('all 仅检查配置；正式 Windows 与 macOS 产物仍须在各自原生宿主构建')
} else {
  pass(`原生宿主与目标平台一致：${normalizedHost}`)
}

if (mode === 'release') {
  if (targetPlatform === 'macos' || targetPlatform === 'all') {
    const apiKeyVariables = ['APPLE_API_ISSUER', 'APPLE_API_KEY', 'APPLE_API_KEY_PATH']
    const appleIdVariables = ['APPLE_ID', 'APPLE_PASSWORD', 'APPLE_TEAM_ID']

    if (envPresent('APPLE_SIGNING_IDENTITY')) {
      pass('已设置 APPLE_SIGNING_IDENTITY')
    } else if (envPresent('APPLE_CERTIFICATE') && envPresent('APPLE_CERTIFICATE_PASSWORD')) {
      pass('已设置 Apple PKCS#12 证书变量')
    } else {
      fail('未设置 APPLE_SIGNING_IDENTITY 或完整的 Apple PKCS#12 证书变量')
    }
    if (apiKeyVariables.every(envPresent)) pass('已设置 App Store Connect API key 公证变量')
    else if (appleIdVariables.every(envPresent)) pass('已设置 Apple ID 公证变量')
    else fail('未完整设置 App Store Connect API key 或 Apple ID 公证变量')
  }

  if (targetPlatform === 'windows' || targetPlatform === 'all') {
    const windowsSigningMarkers = [
      'WINDOWS_CERTIFICATE_THUMBPRINT',
      'WINDOWS_CERTIFICATE_PATH',
      'AZURE_TENANT_ID',
    ]
    if (windowsSigningMarkers.some(envPresent)) pass('检测到 Windows 签名配置标记')
    else fail('未检测到 Windows Authenticode 或 Azure Trusted Signing 配置标记')
  }
}

for (const result of results) {
  console.log(`[${result.level}] ${result.message}`)
}

const counts = Object.fromEntries(
  ['PASS', 'WARN', 'FAIL'].map(level => [level, results.filter(result => result.level === level).length]),
)
console.log(`\n汇总：${counts.PASS} PASS，${counts.WARN} WARN，${counts.FAIL} FAIL`)
process.exit(counts.FAIL > 0 ? 1 : 0)
