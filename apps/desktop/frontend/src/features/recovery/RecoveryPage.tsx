// 恢复模块：操作历史、系统还原点管理
import { useCallback, useEffect, useState } from 'react';
import { History, ShieldPlus, CheckCircle2, XCircle } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { getHistory, getRestorePoints, createRestorePoint } from '@/api/providers';
import type { RestorePointInfo, SystemActionRecord } from '@/types';

export default function RecoveryPage() {
  const [history, setHistory] = useState<SystemActionRecord[]>([]);
  const [points, setPoints] = useState<RestorePointInfo[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [creating, setCreating] = useState(false);

  const refresh = useCallback(async () => {
    setError(null);
    try {
      const [h, p] = await Promise.all([getHistory(30), getRestorePoints()]);
      setHistory(h);
      setPoints(p);
    } catch (err) {
      setError((err as Error).message);
    }
  }, []);

  useEffect(() => {
    refresh();
  }, [refresh]);

  const handleCreateRestorePoint = async () => {
    setCreating(true);
    try {
      const res = await createRestorePoint('AetherOS 手动还原点');
      if (res.exitCode === 0) {
        setError(null);
        await refresh();
      } else {
        setError('创建还原点失败，请以管理员身份运行。');
      }
    } catch (err) {
      setError((err as Error).message);
    } finally {
      setCreating(false);
    }
  };

  return (
    <div className="space-y-6">
      <header className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-foreground">恢复中心</h1>
          <p className="mt-1 text-sm text-muted-foreground">操作历史审计、系统还原点管理</p>
        </div>
        <Button onClick={handleCreateRestorePoint} disabled={creating}>
          <ShieldPlus size={16} />
          {creating ? '创建中…' : '创建还原点'}
        </Button>
      </header>

      {error && (
        <Card>
          <CardContent className="py-4 text-sm text-warning">{error}</CardContent>
        </Card>
      )}

      {/* 操作历史 */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <History size={18} className="text-brand" /> 操作历史
          </CardTitle>
        </CardHeader>
        <CardContent>
          {history.length === 0 ? (
            <p className="py-6 text-center text-sm text-muted-foreground">
              暂无操作记录。执行清理、电源计划或休眠操作后会自动记录。
            </p>
          ) : (
            <div className="space-y-2">
              {history.map((record) => (
                <div
                  key={record.id}
                  className="flex items-center gap-3 rounded-lg border px-3 py-2 text-sm"
                  style={{ borderColor: 'var(--border)' }}
                >
                  {record.succeeded ? (
                    <CheckCircle2 size={16} className="shrink-0 text-success" />
                  ) : (
                    <XCircle size={16} className="shrink-0 text-danger" />
                  )}
                  <div className="flex-1">
                    <div className="text-foreground">{record.displayName}</div>
                    <div className="text-xs text-muted-foreground">
                      {record.action} · {new Date(record.executedAt).toLocaleString()}
                    </div>
                  </div>
                  <span className="font-mono text-[11px] text-muted-foreground">exit {record.exitCode}</span>
                </div>
              ))}
            </div>
          )}
        </CardContent>
      </Card>

      {/* 系统还原点 */}
      <Card>
        <CardHeader>
          <CardTitle>系统还原点</CardTitle>
        </CardHeader>
        <CardContent>
          {points.length === 0 ? (
            <p className="py-6 text-center text-sm text-muted-foreground">
              未发现还原点，或系统保护未开启。
            </p>
          ) : (
            <div className="space-y-2">
              {points.map((point) => (
                <div
                  key={point.sequenceNumber}
                  className="flex items-center gap-3 rounded-lg border px-3 py-2 text-sm"
                  style={{ borderColor: 'var(--border)' }}
                >
                  <span className="font-mono text-xs text-muted-foreground">#{point.sequenceNumber}</span>
                  <span className="flex-1 text-foreground">{point.description}</span>
                  <span className="text-xs text-muted-foreground">
                    {new Date(point.creationTime).toLocaleString()}
                  </span>
                </div>
              ))}
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
