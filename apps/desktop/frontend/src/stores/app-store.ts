// Zustand 全局状态管理：跨页面共享的全局状态（主题、语言）
import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { Language } from '@/i18n';

export type Theme = 'dark' | 'light';

interface AppState {
  theme: Theme;
  language: Language;
  setTheme: (theme: Theme) => void;
  toggleTheme: () => void;
  setLanguage: (language: Language) => void;
}

export const useAppStore = create<AppState>()(
  persist(
    (set, get) => ({
      theme: 'dark',
      language: (navigator.language.toLowerCase().startsWith('zh') ? 'zh-CN' : 'en-US') as Language,
      setTheme: (theme) => set({ theme }),
      toggleTheme: () => set({ theme: get().theme === 'dark' ? 'light' : 'dark' }),
      setLanguage: (language) => set({ language }),
    }),
    { name: 'aetheros-app-settings' }
  )
);

/**
 * 将主题应用到 <html>（.dark 切换深色），并同步 Tauri 原生窗口标题栏主题。
 * 修复深色模式下窗口顶部原生标题栏仍为白色的问题。
 */
export async function applyTheme(theme: Theme) {
  document.documentElement.classList.toggle('dark', theme === 'dark');

  try {
    const { getCurrentWindow } = await import('@tauri-apps/api/window');
    const win = getCurrentWindow();
    await win.setTheme(theme === 'dark' ? 'dark' : 'light');
    await win.setBackgroundColor(
      theme === 'dark'
        ? { red: 20, green: 22, blue: 31, alpha: 1 }
        : { red: 246, green: 247, blue: 251, alpha: 1 }
    );
  } catch {
    // Web 开发环境（无 Tauri）忽略
  }
}
