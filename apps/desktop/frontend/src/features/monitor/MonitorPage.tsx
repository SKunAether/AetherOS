// 监控模块：CPU / 内存 / 磁盘实时监控与数据可视化
import { useCallback, useEffect, useState, type ComponentType } from 'react';
import { Cpu, MemoryStick, HardDrive, Activity } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Sparkline } from '@/components/charts/Sparkline';
import { useInterval } from '@/hooks/use-interval';
import { getMonitorSnapshot } from '@/api/providers';
import type { MonitorSnapshot } from '@/types';

function formatBytes(bytes: number): string {
  if (!bytes) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1);
  return `${(bytes / 1024 ** i).toFixed(1)} ${units[i]}`;
}

function formatUptime(ms: number): string {
  const totalMin = Math.floor(ms / 60000);
  const days = Math.floor(totalMin / 1440);
  const hours = Math.floor((totalMin % 1440) / 60);
  const mins = totalMin % 60;
  return `${days}天 ${hours}小时 ${mins}分钟`;
}

/** 单个监控指标卡 */
function MetricCard({
  title,
  icon: Icon,
  value,
  unit,
  color,
  history,
}: {
  title: string;
  icon: ComponentType<{ size?: number; className?: string }>;
  value: string;
  unit: string;
  color: string;
  history: number[];
}) {
  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Icon size={18} className="text-brand" />
          {title}
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div className="flex items-baseline gap-1">
          <span className="text-3xl font-bold" style={{ color }}>
            {value}
          </span>
          <span className="text-sm text-muted-foreground">{unit}</span>
        </div>
        <div className="mt-3">
          <Sparkline data={history} color={color} />
        </div>
      </CardContent>
    </Card>
  );
}

export default function MonitorPage() {
  const [snapshot, setSnapshot] = useState<MonitorSnapshot | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [cpuHistory, setCpuHistory] = useState<number[]>([]);
  const [memHistory, setMemHistory] = useState<number[]>([]);
  const [diskHistory, setDiskHistory] = useState<number[]>([]);

  const tick = useCallback(async () => {
    try {
      const snap = await getMonitorSnapshot();
      setSnapshot(snap);
      setCpuHistory((h) => [...h.slice(-59), snap.cpuUsagePercent]);
      setMemHistory((h) => [...h.slice(-59), snap.memoryUsagePercent]);
      setDiskHistory((h) => [...h.slice(-59), snap.diskUsagePercent]);
      setError(null);
    } catch (err) {
      setError((err as Error).message);
    }
  }, []);

  useInterval(tick, 2000);

  // 初始加载
  useEffect(() => {
    tick();
  }, [tick]);

  return (
    <div className="space-y-6">
      <header className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-foreground">状态监控</h1>
          <p className="mt-1 text-sm text-muted-foreground">CPU / 内存 / 磁盘实时监控 · 每 2 秒刷新</p>
        </div>
        {snapshot && (
          <div className="flex items-center gap-2 text-xs text-muted-foreground">
            <Activity size={14} className="text-success" />
            已运行 {formatUptime(snapshot.uptimeMs)} · {snapshot.processorCount} 核
          </div>
        )}
      </header>

      {error && (
        <Card>
          <CardContent className="py-4 text-sm text-danger">{error}</CardContent>
        </Card>
      )}

      <div className="grid grid-cols-3 gap-6">
        <MetricCard
          title="CPU 使用率"
          icon={Cpu}
          value={snapshot ? snapshot.cpuUsagePercent.toFixed(1) : '–'}
          unit="%"
          color="#6C72F7"
          history={cpuHistory}
        />
        <MetricCard
          title="内存使用率"
          icon={MemoryStick}
          value={snapshot ? snapshot.memoryUsagePercent.toFixed(1) : '–'}
          unit="%"
          color="#27B786"
          history={memHistory}
        />
        <MetricCard
          title="磁盘使用率"
          icon={HardDrive}
          value={snapshot ? snapshot.diskUsagePercent.toFixed(1) : '–'}
          unit="%"
          color="#D99020"
          history={diskHistory}
        />
      </div>

      <Card>
        <CardHeader>
          <CardTitle>内存详情</CardTitle>
        </CardHeader>
        <CardContent className="text-sm text-muted-foreground">
          {snapshot ? (
            <div className="flex gap-6">
              <span>已用：<b className="text-foreground">{formatBytes(snapshot.memoryUsedBytes)}</b></span>
              <span>总计：<b className="text-foreground">{formatBytes(snapshot.memoryTotalBytes)}</b></span>
              <span>磁盘可用：<b className="text-foreground">{formatBytes(snapshot.diskFreeBytes)}</b></span>
              <span>磁盘总计：<b className="text-foreground">{formatBytes(snapshot.diskTotalBytes)}</b></span>
            </div>
          ) : (
            '加载中…'
          )}
        </CardContent>
      </Card>
    </div>
  );
}
