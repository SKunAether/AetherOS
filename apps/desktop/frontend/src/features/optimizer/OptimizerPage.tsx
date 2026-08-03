// 优化模块：电源计划管理、休眠开关、启动项管理、虚拟内存
import { useCallback, useEffect, useState } from 'react';
import { Zap, Moon, Rocket, RefreshCw, Trash2, MemoryStick } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import {
  getPowerPlans,
  activatePowerPlan,
  getHibernateState,
  setHibernate,
  getStartupItems,
  deleteStartupItem,
  getVirtualMemory,
  applyVirtualMemory,
} from '@/api/providers';
import type {
  HibernateState,
  PageFileConfiguration,
  PowerPlanState,
  StartupItem,
} from '@/types';
import { cn } from '@/lib/utils';

function formatBytes(bytes: number): string {
  if (!bytes) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1);
  return `${(bytes / 1024 ** i).toFixed(1)} ${units[i]}`;
}

const CATEGORY_LABEL: Record<string, string> = {
  Balanced: '平衡',
  HighPerformance: '高性能',
  PowerSaver: '节能',
  Custom: '自定义',
};

export default function OptimizerPage() {
  const [plans, setPlans] = useState<PowerPlanState | null>(null);
  const [hibernate, setHibernateState] = useState<HibernateState | null>(null);
  const [startup, setStartup] = useState<StartupItem[]>([]);
  const [virtualMemory, setVirtualMemory] = useState<PageFileConfiguration | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [toast, setToast] = useState<string | null>(null);

  const showToast = (msg: string) => {
    setToast(msg);
    setTimeout(() => setToast(null), 3000);
  };

  const refresh = useCallback(async () => {
    setError(null);
    try {
      const [p, h, s, v] = await Promise.all([
        getPowerPlans(),
        getHibernateState(),
        getStartupItems(),
        getVirtualMemory(),
      ]);
      setPlans(p);
      setHibernateState(h);
      setStartup(s);
      setVirtualMemory(v);
    } catch (err) {
      setError((err as Error).message);
    }
  }, []);

  useEffect(() => {
    refresh();
  }, [refresh]);

  const handleActivate = async (planId: string) => {
    setBusy(true);
    try {
      const record = await activatePowerPlan(planId);
      showToast(record.succeeded ? '电源计划已切换' : `切换失败：${record.errorMessage ?? ''}`);
      await refresh();
    } catch (err) {
      setError((err as Error).message);
    } finally {
      setBusy(false);
    }
  };

  const handleHibernate = async (enabled: boolean) => {
    setBusy(true);
    try {
      const record = await setHibernate(enabled);
      showToast(
        record.succeeded
          ? enabled
            ? '已启用休眠'
            : '已禁用休眠'
          : `操作失败：${record.errorMessage ?? ''}`
      );
      await refresh();
    } catch (err) {
      setError((err as Error).message);
    } finally {
      setBusy(false);
    }
  };

  const handleDeleteStartup = async (item: StartupItem) => {
    try {
      await deleteStartupItem(item.name, item.scope);
      showToast(`已移除启动项「${item.name}」`);
      await refresh();
    } catch (err) {
      setError((err as Error).message);
    }
  };

  const handleApplyVM = async (automatic: boolean) => {
    setBusy(true);
    try {
      await applyVirtualMemory(automatic, []);
      showToast(automatic ? '已切换为自动管理虚拟内存（重启生效）' : '配置已保存（重启生效）');
      await refresh();
    } catch (err) {
      setError((err as Error).message);
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="space-y-6">
      <header className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-foreground">性能优化</h1>
          <p className="mt-1 text-sm text-muted-foreground">电源计划、休眠、启动项、虚拟内存管理</p>
        </div>
        <Button variant="secondary" onClick={refresh} disabled={busy}>
          <RefreshCw size={16} /> 刷新
        </Button>
      </header>

      {error && (
        <Card>
          <CardContent className="py-4 text-sm text-danger">{error}</CardContent>
        </Card>
      )}
      {toast && (
        <Card>
          <CardContent className="py-3 text-sm text-success">{toast}</CardContent>
        </Card>
      )}

      <div className="grid grid-cols-2 gap-6">
        {/* 电源计划 */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Zap size={18} className="text-brand" /> 电源计划
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-2">
            {plans?.plans.map((plan) => (
              <div
                key={plan.id}
                className={cn(
                  'flex items-center gap-3 rounded-xl border p-3 transition-colors',
                  plan.isActive && 'border-brand/50 bg-brand/5'
                )}
                style={{ borderColor: plan.isActive ? 'rgba(108,114,247,0.5)' : 'var(--border)' }}
              >
                <div className="flex-1">
                  <div className="flex items-center gap-2 text-sm text-foreground">
                    {plan.name}
                    <span className="text-[10px] text-muted-foreground">
                      {CATEGORY_LABEL[plan.category] ?? plan.category}
                    </span>
                    {plan.isActive && (
                      <span className="rounded bg-brand/20 px-1.5 py-0.5 text-[10px] text-brand">
                        当前
                      </span>
                    )}
                  </div>
                </div>
                {!plan.isActive && (
                  <Button size="sm" variant="secondary" disabled={busy} onClick={() => handleActivate(plan.id)}>
                    启用
                  </Button>
                )}
              </div>
            ))}
            {plans && plans.plans.length === 0 && (
              <p className="py-6 text-center text-sm text-muted-foreground">暂无电源计划</p>
            )}
          </CardContent>
        </Card>

        {/* 休眠 */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Moon size={18} className="text-brand" /> 休眠
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex items-center justify-between">
              <div className="text-sm text-foreground">
                休眠功能
                <div className="text-xs text-muted-foreground">
                  {hibernate?.isEnabled ? '已启用' : '已禁用'}
                  {hibernate?.isHibernateFilePresent && (
                    <> · hiberfil.sys {formatBytes(hibernate.hibernateFileBytes)}</>
                  )}
                </div>
              </div>
              <div className="flex gap-2">
                <Button
                  size="sm"
                  variant={hibernate?.isEnabled ? 'secondary' : 'primary'}
                  disabled={busy}
                  onClick={() => handleHibernate(true)}
                >
                  启用
                </Button>
                <Button
                  size="sm"
                  variant={hibernate?.isEnabled ? 'danger' : 'secondary'}
                  disabled={busy}
                  onClick={() => handleHibernate(false)}
                >
                  禁用
                </Button>
              </div>
            </div>
            <p className="text-xs text-muted-foreground">
              启用/禁用休眠需要管理员权限，操作会触发 UAC 授权。
            </p>
          </CardContent>
        </Card>
      </div>

      {/* 虚拟内存 */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <MemoryStick size={18} className="text-brand" /> 虚拟内存
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-3">
          {virtualMemory ? (
            <>
              <div className="flex items-center justify-between">
                <div className="text-sm text-foreground">
                  自动管理所有驱动器的分页文件大小
                  <div className="text-xs text-muted-foreground">
                    {virtualMemory.isAutomaticallyManaged ? '当前：系统自动管理' : '当前：自定义配置'}
                    {virtualMemory.entries.length > 0 && (
                      <> · 分页文件共 {formatBytes(virtualMemory.totalCurrentFileBytes)}</>
                    )}
                  </div>
                </div>
                <Button
                  size="sm"
                  variant={virtualMemory.isAutomaticallyManaged ? 'primary' : 'secondary'}
                  disabled={busy}
                  onClick={() => handleApplyVM(true)}
                >
                  启用自动管理
                </Button>
              </div>
              {virtualMemory.entries.map((entry) => (
                <div
                  key={entry.path}
                  className="flex items-center gap-3 rounded-lg border px-3 py-2 text-sm"
                  style={{ borderColor: 'var(--border)' }}
                >
                  <span className="flex-1 font-mono text-xs text-foreground">{entry.path}</span>
                  <span className="text-xs text-muted-foreground">
                    {entry.initialSizeMb} - {entry.maximumSizeMb} MB
                  </span>
                  {entry.exists && (
                    <span className="text-xs text-success">{formatBytes(entry.currentFileBytes)}</span>
                  )}
                </div>
              ))}
              <p className="text-xs text-muted-foreground">
                修改虚拟内存需要管理员权限并重启系统后生效。
              </p>
            </>
          ) : (
            <p className="py-4 text-center text-sm text-muted-foreground">加载中…</p>
          )}
        </CardContent>
      </Card>

      {/* 启动项 */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Rocket size={18} className="text-brand" /> 启动项
          </CardTitle>
        </CardHeader>
        <CardContent>
          {startup.length === 0 ? (
            <p className="py-4 text-center text-sm text-muted-foreground">未读取到启动项</p>
          ) : (
            <div className="grid grid-cols-1 gap-2">
              {startup.slice(0, 40).map((item, idx) => (
                <div
                  key={`${item.name}-${idx}`}
                  className="flex items-center gap-3 rounded-lg border px-3 py-2 text-sm"
                  style={{ borderColor: 'var(--border)' }}
                >
                  <span className="w-28 shrink-0 truncate font-medium text-foreground">
                    {item.name}
                  </span>
                  <span className="flex-1 truncate font-mono text-xs text-muted-foreground">
                    {item.command}
                  </span>
                  <span className="rounded px-1.5 py-0.5 text-[10px] text-muted-foreground">
                    {item.scope}
                  </span>
                  <button
                    onClick={() => handleDeleteStartup(item)}
                    className="rounded p-1 text-muted-foreground transition-colors hover:bg-danger/10 hover:text-danger"
                    title="移除启动项"
                  >
                    <Trash2 size={14} />
                  </button>
                </div>
              ))}
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
