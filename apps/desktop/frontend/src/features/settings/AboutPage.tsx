// 关于页面：产品信息、版本、技术栈、官网/仓库/更新日志/检查更新
// 注意：不展示机器名/主机名等隐私信息
import { useTranslation } from 'react-i18next';
import {
  Globe,
  Github,
  FileText,
  RefreshCw,
  Heart,
  Cpu,
  Shield,
  Boxes,
} from 'lucide-react';
import { Card } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { useSystemInfo } from '@/hooks/use-system-info';
import { openExternal } from '@/api/system';

const TECH_STACK = ['Rust', 'Tauri 2', 'React 19', 'TypeScript', 'Vite 6', 'TailwindCSS'];

const GITHUB_URL = 'https://github.com/SKunAether/AetherOS';
const RELEASES_URL = `${GITHUB_URL}/releases`;
const CHANGELOG_URL = `${GITHUB_URL}/blob/main/CHANGELOG.md`;

const INFO_ITEMS = [
  { icon: Cpu, label: '内核', value: 'Rust 原生 · Capability-Provider 平台' },
  { icon: Boxes, label: '模块', value: '清理 · 优化 · 监控 · 恢复 · AI' },
  { icon: Shield, label: '隐私', value: '本地优先 · 可审计 · 可回滚' },
  { icon: Globe, label: '语言', value: '中文 · English' },
];

export default function AboutPage() {
  const { t } = useTranslation(['nav', 'common']);
  const { data: info } = useSystemInfo();

  const links = [
    { icon: Globe, label: '官方网站', url: GITHUB_URL },
    { icon: Github, label: 'GitHub', url: GITHUB_URL },
    { icon: FileText, label: '更新日志', url: CHANGELOG_URL },
  ];

  return (
    <div className="space-y-6">
      <header>
        <h1 className="text-2xl font-bold text-foreground">{t('nav:about')}</h1>
      </header>

      {/* Hero 区：Logo + 产品名 + 版本（CCSwitch 风格横幅） */}
      <Card className="overflow-hidden">
        <div className="relative p-8">
          <div
            className="pointer-events-none absolute inset-0"
            style={{
              background:
                'radial-gradient(600px 200px at 20% 0%, rgba(108,114,247,0.18), transparent 60%), radial-gradient(400px 160px at 90% 100%, rgba(39,183,134,0.10), transparent 60%)',
            }}
          />
          <div className="relative flex items-center gap-6">
            <div className="flex h-20 w-20 shrink-0 items-center justify-center rounded-2xl bg-gradient-to-br from-brand to-brand-light text-4xl font-bold text-white shadow-lg shadow-brand/30">
              A
            </div>
            <div className="flex-1">
              <div className="text-2xl font-bold text-foreground">{t('common:appName')}</div>
              <div className="mt-1 text-sm text-muted-foreground">{t('common:tagline')}</div>
              <div className="mt-3 flex items-center gap-3 text-xs text-muted-foreground">
                <span className="rounded-lg bg-brand/15 px-2 py-1 font-mono text-brand">v2.0.0</span>
                <span>{info?.osVersion ?? 'Windows'}</span>
              </div>
            </div>
            <span className="hidden items-center gap-1.5 text-xs text-success sm:flex">
              <Heart size={13} /> 开源 · AGPL-3.0
            </span>
          </div>
        </div>
      </Card>

      {/* 外链：官网 / GitHub / 更新日志 / 检查更新 */}
      <Card>
        <div className="grid grid-cols-4 gap-3 p-5">
          {links.map(({ icon: Icon, label, url }) => (
            <Button key={label} variant="secondary" className="h-20 flex-col gap-2" onClick={() => openExternal(url)}>
              <Icon size={18} className="text-brand" />
              {label}
            </Button>
          ))}
          <Button variant="secondary" className="h-20 flex-col gap-2" onClick={() => openExternal(RELEASES_URL)}>
            <RefreshCw size={18} className="text-brand" />
            检查更新
          </Button>
        </div>
      </Card>

      {/* 技术栈 */}
      <Card>
        <div className="p-6">
          <div className="mb-4 text-sm font-semibold text-foreground">技术栈 / Tech Stack</div>
          <div className="flex flex-wrap gap-2">
            {TECH_STACK.map((tech) => (
              <span
                key={tech}
                className="rounded-lg border border-border bg-muted/50 px-3 py-1.5 text-xs text-foreground"
              >
                {tech}
              </span>
            ))}
          </div>
        </div>
      </Card>

      {/* 能力速览 */}
      <div className="grid grid-cols-4 gap-4">
        {INFO_ITEMS.map(({ icon: Icon, label, value }) => (
          <Card key={label} className="p-4">
            <Icon size={18} className="mb-3 text-brand" />
            <div className="text-xs text-muted-foreground">{label}</div>
            <div className="mt-0.5 text-sm text-foreground">{value}</div>
          </Card>
        ))}
      </div>

      <p className="pt-2 text-center text-xs text-muted-foreground">
        使用 ❤️ 和 Rust / Tauri / React 构建
      </p>
    </div>
  );
}
