// 应用根组件：路由定义 + 主题/语言副作用绑定
import { useEffect } from 'react';
import { HashRouter, Navigate, Route, Routes } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { useAppStore, applyTheme } from '@/stores/app-store';
import AppLayout from '@/app/layouts/AppLayout';
import DashboardPage from '@/features/dashboard/DashboardPage';
import CleanerPage from '@/features/cleaner/CleanerPage';
import OptimizerPage from '@/features/optimizer/OptimizerPage';
import MonitorPage from '@/features/monitor/MonitorPage';
import RecoveryPage from '@/features/recovery/RecoveryPage';
import SettingsPage from '@/features/settings/SettingsPage';
import AboutPage from '@/features/settings/AboutPage';

export default function App() {
  const { theme, language } = useAppStore();
  const { i18n } = useTranslation();

  // 主题副作用
  useEffect(() => {
    applyTheme(theme);
  }, [theme]);

  // 语言副作用
  useEffect(() => {
    i18n.changeLanguage(language);
  }, [language, i18n]);

  return (
    <HashRouter>
      <Routes>
        <Route path="/" element={<AppLayout />}>
          <Route index element={<Navigate to="/dashboard" replace />} />
          <Route path="dashboard" element={<DashboardPage />} />
          <Route path="cleaner" element={<CleanerPage />} />
          <Route path="optimizer" element={<OptimizerPage />} />
          <Route path="monitor" element={<MonitorPage />} />
          <Route path="recovery" element={<RecoveryPage />} />
          <Route path="settings" element={<SettingsPage />} />
          <Route path="about" element={<AboutPage />} />
        </Route>
      </Routes>
    </HashRouter>
  );
}
