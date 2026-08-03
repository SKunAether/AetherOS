// 功能模块占位页面：未实现模块的通用空态展示
import type { LucideIcon } from 'lucide-react';
import { Card, CardContent } from '@/components/ui/card';

interface PlaceholderPageProps {
  title: string;
  description: string;
  icon: LucideIcon;
}

/** 占位页：标识功能域与规划能力，Phase 3-5 将逐个落地 */
export function PlaceholderPage({ title, description, icon: Icon }: PlaceholderPageProps) {
  return (
    <div className="space-y-6">
      <header>
        <h1 className="text-2xl font-bold text-foreground">{title}</h1>
        <p className="mt-1 text-sm text-muted-foreground">{description}</p>
      </header>
      <Card>
        <CardContent className="flex flex-col items-center justify-center gap-4 py-16">
          <Icon size={48} className="text-brand/60" />
          <p className="text-sm text-muted-foreground">
            This module is under construction — coming in a later phase.
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
