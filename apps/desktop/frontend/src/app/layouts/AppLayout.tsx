// 应用主布局：侧边栏导航 + 顶部栏 + 内容区
// 设计风格：深色玻璃拟态 + 卡片式布局 + 侧边栏导航
import { NavLink, Outlet } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import {
  LayoutDashboard,
  Sparkles,
  Gauge,
  Activity,
  History,
  Settings,
  Info,
  Moon,
  Sun,
  Globe,
} from 'lucide-react';
import { useAppStore } from '@/stores/app-store';
import { useProviders } from '@/hooks/use-providers';
import { SUPPORTED_LANGUAGES } from '@/i18n';

/** 侧边栏导航项定义 */
const NAV_ITEMS = [
  { path: '/dashboard', key: 'nav:dashboard', icon: LayoutDashboard },
  { path: '/cleaner', key: 'nav:cleaner', icon: Sparkles },
  { path: '/optimizer', key: 'nav:optimizer', icon: Gauge },
  { path: '/monitor', key: 'nav:monitor', icon: Activity },
  { path: '/recovery', key: 'nav:recovery', icon: History },
  { path: '/settings', key: 'nav:settings', icon: Settings },
  { path: '/about', key: 'nav:about', icon: Info },
];

export default function AppLayout() {
  const { t } = useTranslation(['nav', 'common']);
  const { theme, toggleTheme, setLanguage, language } = useAppStore();
  const { data: providers } = useProviders();

  return (
    <div className="flex h-full">
      {/* 侧边栏 */}
      <aside className="flex w-60 flex-col border-r px-3 py-4" style={{ borderColor: 'var(--border)', background: 'var(--sidebar)' }}>
        {/* Logo */}
        <div className="mb-8 flex items-center gap-3 px-3">
          <div className="flex h-9 w-9 items-center justify-center rounded-xl bg-brand text-lg font-bold text-white">
            A
          </div>
          <div>
            <div className="text-sm font-semibold text-foreground">{t('common:appName')}</div>
            <div className="text-[11px] text-muted-foreground">v2.0.0</div>
          </div>
        </div>

        {/* 导航 */}
        <nav className="flex flex-1 flex-col gap-1">
          {NAV_ITEMS.map(({ path, key, icon: Icon }) => (
            <NavLink
              key={path}
              to={path}
              className={({ isActive }) => (isActive ? 'nav-item is-active' : 'nav-item')}
            >
              <Icon size={18} />
              <span>{t(key)}</span>
            </NavLink>
          ))}
        </nav>

        {/* 侧边栏底部：Provider 状态 + 主题与语言切换 */}
        <div className="mt-4 space-y-3 border-t pt-4" style={{ borderColor: 'var(--border)' }}>
          <div className="flex items-center justify-between px-3 text-[11px] text-muted-foreground">
            <span>Providers</span>
            <span className="rounded-md bg-muted px-1.5 py-0.5 font-mono">
              {providers?.length ?? '–'}
            </span>
          </div>
        </div>
        <div className="mt-3 flex items-center gap-2">
          <button
            className="nav-item flex-1 justify-center"
            onClick={toggleTheme}
            title={theme === 'dark' ? 'Switch to light' : 'Switch to dark'}
          >
            {theme === 'dark' ? <Sun size={16} /> : <Moon size={16} />}
          </button>
          <button
            className="nav-item flex-1 justify-center"
            onClick={() => {
              const next =
                SUPPORTED_LANGUAGES[(SUPPORTED_LANGUAGES.indexOf(language) + 1) % SUPPORTED_LANGUAGES.length];
              setLanguage(next);
            }}
            title="切换语言 / Switch language"
          >
            <Globe size={16} />
            <span className="text-xs">{language === 'zh-CN' ? '中' : 'EN'}</span>
          </button>
        </div>
      </aside>

      {/* 内容区 */}
      <main className="flex-1 overflow-y-auto p-6">
        <div className="mx-auto max-w-6xl animate-fade-in">
          <Outlet />
        </div>
      </main>
    </div>
  );
}
