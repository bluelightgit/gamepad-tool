import { ref, computed } from 'vue'
import { enMessages, zhMessages, languages, type LanguageCode, type Messages } from './messages'

// 当前语言
const currentLanguage = ref<LanguageCode>('en')

// 消息映射
const messageMap: Record<LanguageCode, Messages> = {
  en: enMessages,
  zh: zhMessages
}

// 获取系统语言
const getSystemLanguage = (): LanguageCode => {
  const browserLang = navigator.language.toLowerCase()
  if (browserLang.startsWith('zh')) {
    return 'zh'
  }
  return 'en'
}

// 初始化语言（从localStorage读取，如果没有则使用系统语言）
const initLanguage = () => {
  const savedLang = localStorage.getItem('gamepad-tool-language') as LanguageCode
  if (savedLang && languages.some(lang => lang.code === savedLang)) {
    currentLanguage.value = savedLang
  } else {
    currentLanguage.value = getSystemLanguage()
  }
}

// 设置语言
const setLanguage = (lang: LanguageCode) => {
  currentLanguage.value = lang
  localStorage.setItem('gamepad-tool-language', lang)
}

// 当前消息
const messages = computed(() => messageMap[currentLanguage.value])

// 获取消息的辅助函数
const t = (key: string): string => {
  const keys = key.split('.')
  let value: any = messages.value
  
  for (const k of keys) {
    if (value && typeof value === 'object' && k in value) {
      value = value[k]
    } else {
      console.warn(`Translation key not found: ${key}`)
      return key // 返回key作为fallback
    }
  }
  
  return typeof value === 'string' ? value : key
}

// 组合式函数
export const useI18n = () => {
  return {
    currentLanguage: computed(() => currentLanguage.value),
    languages,
    messages,
    t,
    setLanguage,
    initLanguage
  }
}
