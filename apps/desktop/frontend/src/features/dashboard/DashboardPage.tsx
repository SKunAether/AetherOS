// 仪表盘模块：系统健康概览、核心数据展示、快捷操作入口
// 接入真实系统信息（useSystemInfo，30s 轮询）
import { useTranslation } from 'react-i18next';
import { useNavigate } from 'react-router-dom';
import { HardDrive, Zap, Trash2, Search, Settings } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { CircularProgress } from '@/components/ui/progress';
import { Button } from '@/components/ui/button';
import { useSystemInfo } from '@/hooks/use-system-info';

function formatBytes(bytes: number): string {
  if (!bytes) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1);
  return `${(bytes / 1024 ** i).toFixed(1)} ${units[i]}`;
}

export default function DashboardPage() {
  const { t } = useTranslation(['dashboard', 'common']);
  const navigate = useNavigate();
  const { data: info, isLoading, isError } = useSystemInfo();

  // 健康评分：基于磁盘占用与资源使用（Phase 3 接入完整评分模型）
  const usedPercent = info ? Math.round((1 - info.systemDriveFreeBytes / info.systemDriveTotalBytes) * 100) : 62;
  const healthScore = info ? Math.max(20, 100 - usedPercent - (info.physicalMemoryBytes ? 0 : 10)) : 78;

  // 快捷操作 → 对应功能页路由（不展示任何主机名/用户名等隐私信息）
  const quickActions = [
    { key: 'quickScan', icon: Zap, to: '/cleaner' },
    { key: 'cleaner', icon: Trash2, to: '/cleaner' },
    { key: 'deepScan', icon: Search, to: '/cleaner' },
    { key: 'settings', icon: Settings, to: '/settings' },
  ];

  return (
    <div className="space-y-6">
      {/* 欢迎头部 */}
      <header className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-foreground">{t('welcome')}</h1>
          <p className="mt-1 text-sm text-muted-foreground">
            {t('subtitle')}
            {info ? ` · ${info.osVersion}` : ''}
          </p>
        </div>
      </header>

      <div className="grid grid-cols-3 gap-6">
        {/* 健康评分 */}
        <Card>
          <CardHeader>
            <CardTitle>{t('healthScore')}</CardTitle>
          </CardHeader>
          <CardContent className="flex items-center justify-center py-6">
            <CircularProgress value={healthScore} tone="auto" />
          </CardContent>
        </Card>

        {/* 系统盘 */}
        <Card className="col-span-2">
          <CardHeader>
            <CardTitle>{t('systemDrive')}</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4 pt-4">
            {isLoading ? (
              <p className="py-8 text-center text-sm text-muted-foreground">{t('common:status.loading')}</p>
            ) : isError || !info ? (
              <p className="py-8 text-center text-sm text-muted-foreground">{t('common:status.error')}</p>
            ) : (
              <div className="flex items-center gap-4">
                <HardDrive size={28} className="text-brand" />
                <div className="flex-1">
                  <div className="flex items-center justify-between text-sm">
                    <span className="text-foreground">
                      {t('used')} · {info.systemDrive}
                    </span>
                    <span className="text-muted-foreground">
                      {t('free')}: {formatBytes(info.systemDriveFreeBytes)} / {t('total')}:{' '}
                      {formatBytes(info.systemDriveTotalBytes)}
                    </span>
                  </div>
                  <div className="mt-2 h-2.5 overflow-hidden rounded-full bg-muted">
                    <div
                      className="h-full rounded-full bg-gradient-to-r from-brand to-success transition-all duration-700"
                      style={{ width: `${usedPercent}%` }}
                    />
                  </div>
                  <div className="mt-1 text-xs text-muted-foreground">{usedPercent}%</div>
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      </div>

      {/* 快捷操作 */}
      <Card>
        <CardHeader>
          <CardTitle>{t('quickActions')}</CardTitle>
        </CardHeader>
        <CardContent className="grid grid-cols-4 gap-4 pt-2">
          {quickActions.map(({ key, icon: Icon, to }) => (
            <Button
              key={key}
              variant="secondary"
              size="lg"
              className="h-24 flex-col gap-3"
              onClick={() => navigate(to)}
            >
              <Icon size={22} className="text-brand" />
              {t(`action.${key}`)}
            </Button>
          ))}
        </CardContent>
      </Card>
    </div>
  );
}
