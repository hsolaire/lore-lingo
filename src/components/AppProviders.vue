<script setup lang="ts">
import { darkTheme, NConfigProvider, NMessageProvider, type GlobalThemeOverrides } from 'naive-ui'

const themeOverrides: GlobalThemeOverrides = {
  common: {
    primaryColor: '#56b9e9',
    primaryColorHover: '#72c9ef',
    primaryColorPressed: '#3fa4d4',
    primaryColorSuppl: '#56b9e9',
    borderRadius: '9px',
    borderColor: 'rgba(222, 235, 248, 0.14)',
    bodyColor: 'transparent',
    cardColor: 'rgba(22, 25, 34, 0.82)',
    modalColor: 'rgba(22, 25, 34, 0.96)',
    popoverColor: 'rgba(22, 25, 34, 0.96)',
    inputColor: 'rgba(18, 21, 29, 0.72)',
    textColorBase: '#f2f5f8',
    textColor1: '#f2f5f8',
    textColor2: '#c2c8d0',
    textColor3: '#929aa5',
  },
  Button: {
    borderRadiusMedium: '9px',
    fontWeight: '550',
  },
  Tabs: {
    tabTextColorActiveLine: '#f2f5f8',
    tabTextColorHoverLine: '#f2f5f8',
    barColor: '#56b9e9',
  },
}
</script>

<template>
  <NConfigProvider :theme="darkTheme" :theme-overrides="themeOverrides">
    <NMessageProvider>
      <slot />
    </NMessageProvider>
  </NConfigProvider>
</template>
