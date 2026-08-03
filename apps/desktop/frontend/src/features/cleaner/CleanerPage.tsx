// 清理模块：垃圾扫描、项勾选、清理执行、结果统计
import { useCallback, useState } from 'react';
import { ScanLine, Trash2, ShieldAlert, Shield } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { cleanerScan, cleanerExecute } from '@/api/providers';
import type { CleanerItem, CleanerScanResult, CleanerExecutionResult } from '@/types';

function formatBytes(bytes: number): string {
  if (!bytes) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1);
  return `${(bytes / 1024 ** i).toFixed(1)} ${units[i]}`;
}

function riskColor(level: string): string {
  if (level === 'Low') return '#27B786';
  if (level === 'Medium') return '#D99020';
  return '#E5484D';
}

export default function CleanerPage() {
  const [scan, setScan] = useState<CleanerScanResult | null>(null);
  const [result, setResult] = useState<CleanerExecutionResult | null>(null);
  const [selected, setSelected] = useState<Set<string>>(new Set());
  const [loading, setLoading] = useState(false);
  const [executing, setExecuting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleScan = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await cleanerScan();
      setScan(res);
      setSelected(new Set(res.items.filter((i) => i.isSelectedByDefault).map((i) => i.id)));
      setResult(null);
    } catch (err) {
      setError((err as Error).message);
    } finally {
      setLoading(false);
    }
  }, []);

  const toggle = (id: string) => {
    setSelected((prev) => {
      const next = new Set(prev);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return next;
    });
  };

  const handleExecute = useCallback(async () => {
    setExecuting(true);
    setError(null);
    try {
      const res = await cleanerExecute([...selected]);
      setResult(res);
    } catch (err) {
      setError((err as Error).message);
    } finally {
      setExecuting(false);
    }
  }, [selected]);

  return (
    <div className="space-y-6">
      <header className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-foreground">清理与优化</h1>
          <p className="mt-1 text-sm text-muted-foreground">临时文件、Windows 更新缓存、缩略图缓存清理</p>
        </div>
        <div className="flex gap-2">
          <Button variant="secondary" onClick={handleScan} disabled={loading || executing}>
            <ScanLine size={16} />
            {loading ? '扫描中…' : '扫描'}
          </Button>
          <Button
            variant="primary"
            onClick={handleExecute}
            disabled={executing || selected.size === 0}
          >
            <Trash2 size={16} />
            {executing ? '清理中…' : `执行清理 (${selected.size})`}
          </Button>
        </div>
      </header>

      {error && (
        <Card>
          <CardContent className="py-4 text-sm text-danger">{error}</CardContent>
        </Card>
      )}

      {/* 扫描结果列表 */}
      <Card>
        <CardHeader>
          <CardTitle>可清理项</CardTitle>
          {scan && (
            <span className="text-sm text-muted-foreground">
              预计可释放 <b className="text-brand">{formatBytes(scan.totalEstimatedBytes)}</b>
            </span>
          )}
        </CardHeader>
        <CardContent className="space-y-2">
          {!scan && !loading && <p className="py-8 text-center text-sm text-muted-foreground">点击「扫描」开始分析可清理空间</p>}
          {loading && <p className="py-8 text-center text-sm text-muted-foreground">正在扫描…</p>}
          {scan?.items.map((item: CleanerItem) => (
            <label
              key={item.id}
              className="flex cursor-pointer items-center gap-4 rounded-xl border p-4 transition-colors hover:bg-accent/10"
              style={{ borderColor: 'var(--border)' }}
            >
              <input
                type="checkbox"
                checked={selected.has(item.id)}
                onChange={() => toggle(item.id)}
                className="h-4 w-4 accent-[#6C72F7]"
              />
              <div className="flex-1">
                <div className="flex items-center gap-2 text-sm font-medium text-foreground">
                  {item.name}
                  <span
                    className="rounded px-1.5 py-0.5 text-[10px]"
                    style={{ background: `${riskColor(item.riskLevel)}22`, color: riskColor(item.riskLevel) }}
                  >
                    {item.riskLevel}
                  </span>
                  {item.requiresAdministrator && (
                    <span className="flex items-center gap-0.5 text-[10px] text-warning">
                      <ShieldAlert size={11} /> 需管理员
                    </span>
                  )}
                </div>
                <div className="mt-0.5 text-xs text-muted-foreground">{item.description}</div>
                <div className="mt-0.5 font-mono text-[11px] text-muted-foreground/70">{item.path}</div>
              </div>
              <span className="text-sm font-semibold text-foreground">{formatBytes(item.estimatedBytes)}</span>
            </label>
          ))}
        </CardContent>
      </Card>

      {/* 执行结果 */}
      {result && (
        <Card>
          <CardHeader>
            <CardTitle>清理结果</CardTitle>
            <span className="text-sm font-semibold text-success">
              共释放 {formatBytes(result.totalReleasedBytes)}
            </span>
          </CardHeader>
          <CardContent className="space-y-2">
            {result.items.map((item) => (
              <div
                key={item.itemId}
                className="flex items-center gap-3 rounded-lg border px-3 py-2 text-sm"
                style={{ borderColor: 'var(--border)' }}
              >
                <Shield size={14} className={item.succeeded ? 'text-success' : 'text-danger'} />
                <span className="flex-1 text-foreground">{item.name}</span>
                <span className="text-xs text-muted-foreground">
                  {item.beforeBytes} → {item.afterBytes}
                </span>
                <span className="text-xs font-semibold text-success">
                  +{formatBytes(item.releasedBytes)}
                </span>
              </div>
            ))}
          </CardContent>
        </Card>
      )}
    </div>
  );
}
