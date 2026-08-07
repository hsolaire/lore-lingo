<script setup lang="ts">
import AppProviders from '@/components/AppProviders.vue'
import SettingsShell from '@/components/settings/SettingsShell.vue'
import { useBootstrap } from '@/composables/useBootstrap'

const { bootstrap, reloadBootstrap } = useBootstrap()
</script>

<template>
  <AppProviders>
    <SettingsShell v-if="bootstrap.status === 'ready'" :snapshot="bootstrap.snapshot" />
    <main v-else class="bootstrap-state" aria-live="polite">
      <p v-if="bootstrap.status === 'loading'">正在加载设置…</p>
      <template v-else>
        <p>{{ bootstrap.error }}</p>
        <button v-if="bootstrap.status === 'database_error'" type="button" @click="reloadBootstrap">
          重试
        </button>
      </template>
    </main>
  </AppProviders>
</template>

<style scoped>
.bootstrap-state { min-height: 100vh; display: grid; place-content: center; justify-items: center; gap: 12px; padding: 24px; color: var(--fg); text-align: center; }
.bootstrap-state button { border: 1px solid var(--border); border-radius: 8px; padding: 8px 14px; background: var(--glass-hi); color: inherit; cursor: pointer; }
</style>
