// 环形进度组件（健康评分等圆形指标展示）
import { cn } from '@/lib/utils';

interface CircularProgressProps {
  /** 0-100 */
  value: number;
  size?: number;
  strokeWidth?: number;
  className?: string;
  /** 颜色分档：按值自动着色 */
  tone?: 'auto' | 'brand' | 'success' | 'warning' | 'danger';
}

function toneColor(tone: NonNullable<CircularProgressProps['tone']>, value: number): string {
  if (tone === 'auto') {
    if (value >= 80) return '#27b786';
    if (value >= 60) return '#6c72f7';
    if (value >= 40) return '#d99020';
    return '#e5484d';
  }
  const map: Record<string, string> = {
    brand: '#6c72f7',
    success: '#27b786',
    warning: '#d99020',
    danger: '#e5484d',
  };
  return map[tone] ?? '#6c72f7';
}

/** 环形进度指示器 */
export function CircularProgress({
  value,
  size = 120,
  strokeWidth = 10,
  className,
  tone = 'auto',
}: CircularProgressProps) {
  const clamped = Math.max(0, Math.min(100, value));
  const radius = (size - strokeWidth) / 2;
  const circumference = 2 * Math.PI * radius;
  const offset = circumference - (clamped / 100) * circumference;
  const color = toneColor(tone, clamped);

  return (
    <div className={cn('relative inline-flex items-center justify-center', className)} style={{ width: size, height: size }}>
      <svg width={size} height={size} className="-rotate-90">
        <circle
          cx={size / 2}
          cy={size / 2}
          r={radius}
          fill="none"
          stroke="var(--muted)"
          strokeWidth={strokeWidth}
        />
        <circle
          cx={size / 2}
          cy={size / 2}
          r={radius}
          fill="none"
          stroke={color}
          strokeWidth={strokeWidth}
          strokeLinecap="round"
          strokeDasharray={circumference}
          strokeDashoffset={offset}
          className="transition-all duration-700 ease-out"
        />
      </svg>
      <div className="absolute text-center">
        <div className="text-3xl font-bold" style={{ color }}>
          {clamped}
        </div>
        <div className="text-xs text-muted-foreground">/ 100</div>
      </div>
    </div>
  );
}
