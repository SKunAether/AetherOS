// 国际化初始化：基于 react-i18next，支持中英双语无缝切换
// 语言资源按功能模块拆分（common/nav/dashboard/cleaner/...），默认跟随系统语言。
import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';

import commonZh from '@/locales/zh-CN/common.json';
import navZh from '@/locales/zh-CN/nav.json';
import dashboardZh from '@/locales/zh-CN/dashboard.json';

import commonEn from '@/locales/en-US/common.json';
import navEn from '@/locales/en-US/nav.json';
import dashboardEn from '@/locales/en-US/dashboard.json';

/** 支持的语言列表 */
export const SUPPORTED_LANGUAGES = ['zh-CN', 'en-US'] as const;
export type Language = (typeof SUPPORTED_LANGUAGES)[number];

/** 根据系统语言推断默认语言 */
function detectDefaultLanguage(): Language {
  const sysLang = navigator.language.toLowerCase();
  if (sysLang.startsWith('zh')) return 'zh-CN';
  return 'en-US';
}

const resources = {
  'zh-CN': {
    common: commonZh,
    nav: navZh,
    dashboard: dashboardZh,
  },
  'en-US': {
    common: commonEn,
    nav: navEn,
    dashboard: dashboardEn,
  },
};

i18n.use(initReactI18next).init({
  resources,
  lng: detectDefaultLanguage(),
  fallbackLng: 'en-US',
  ns: ['common', 'nav', 'dashboard'],
  defaultNS: 'common',
  interpolation: {
    escapeValue: false,
  },
});

export default i18n;
