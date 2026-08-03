// 迷你折线图：实时监控数据可视化（纯 SVG，无额外依赖）
interface SparklineProps {
  /** 数据点（0-100） */
  data: number[];
  color?: string;
  height?: number;
}

/** 迷你面积折线图 */
export function Sparkline({ data, color = '#6C72F7', height = 60 }: SparklineProps) {
  const width = 280;
  const pad = 4;
  const max = 100;

  const points = data.length > 1 ? data : [0, 0];
  const stepX = (width - pad * 2) / Math.max(points.length - 1, 1);
  const coords = points.map((v, i) => {
    const x = pad + i * stepX;
    const y = height - pad - (Math.min(v, max) / max) * (height - pad * 2);
    return { x, y };
  });

  const line = coords.map((p, i) => `${i === 0 ? 'M' : 'L'}${p.x.toFixed(1)},${p.y.toFixed(1)}`).join(' ');
  const area = `${line} L${coords[coords.length - 1].x.toFixed(1)},${height} L${coords[0].x.toFixed(1)},${height} Z`;
  const last = coords[coords.length - 1];

  return (
    <svg width="100%" height={height} viewBox={`0 0 ${width} ${height}`} preserveAspectRatio="none">
      <defs>
        <linearGradient id={`spark-${color.replace('#', '')}`} x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stopColor={color} stopOpacity="0.35" />
          <stop offset="100%" stopColor={color} stopOpacity="0.02" />
        </linearGradient>
      </defs>
      <path d={area} fill={`url(#spark-${color.replace('#', '')})`} />
      <path d={line} fill="none" stroke={color} strokeWidth="2" strokeLinejoin="round" />
      <circle cx={last.x} cy={last.y} r="3" fill={color} />
    </svg>
  );
}
