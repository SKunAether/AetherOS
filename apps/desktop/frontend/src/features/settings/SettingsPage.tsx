// 设置模块：外观、关闭行为、开机自启、AI 服务商配置
import { useCallback, useEffect, useState } from 'react';
import { Moon, Sun, Globe, Power, Rocket, Bot, Plus, Trash2, PlugZap } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { useAppStore } from '@/stores/app-store';
import { useSettings } from '@/hooks/use-settings';
import { getAIProviders, saveAIProvider, deleteAIProvider, testAIProvider } from '@/api/ai';
import type { AIProviderConfig, AppSettings } from '@/types';

const EMPTY_PROVIDER: AIProviderConfig = {
  id: '',
  name: '',
  providerType: 'OpenAICompatible',
  apiBaseUrl: '',
  encryptedApiKey: '',
  modelId: '',
  isEnabled: true,
  isDefault: false,
};

export default function SettingsPage() {
  const { theme, toggleTheme, language, setLanguage } = useAppStore();
  const { query, mutation } = useSettings();
  const saved = query.data;
  const [providers, setProviders] = useState<AIProviderConfig[]>([]);
  const [draft, setDraft] = useState<AIProviderConfig>(EMPTY_PROVIDER);
  const [toast, setToast] = useState<string | null>(null);
  const [testing, setTesting] = useState<string | null>(null);

  const showToast = (msg: string) => {
    setToast(msg);
    setTimeout(() => setToast(null), 3000);
  };

  const loadProviders = useCallback(async () => {
    try {
      setProviders(await getAIProviders());
    } catch {
      // 忽略
    }
  }, []);

  useEffect(() => {
    loadProviders();
  }, [loadProviders]);

  const patchSetting = (patch: Partial<AppSettings>) => {
    if (saved) mutation.mutate({ ...saved, ...patch });
  };

  const handleSaveProvider = async () => {
    try {
      const saved = await saveAIProvider({
        ...draft,
        id: draft.id || `ai-${Date.now().toString(36)}`,
      });
      showToast(`已保存服务商「${saved.name}」`);
      setDraft(EMPTY_PROVIDER);
      await loadProviders();
    } catch (err) {
      showToast(`保存失败：${(err as Error).message}`);
    }
  };

  const handleDeleteProvider = async (id: string) => {
    await deleteAIProvider(id);
    await loadProviders();
  };

  const handleTest = async (p: AIProviderConfig) => {
    setTesting(p.id);
    try {
      const ok = await testAIProvider(p);
      showToast(ok ? '连接成功' : '连接失败');
    } catch (err) {
      showToast(`连接失败：${(err as Error).message}`);
    } finally {
      setTesting(null);
    }
  };

  return (
    <div className="space-y-6">
      <header>
        <h1 className="text-2xl font-bold text-foreground">设置</h1>
        <p className="mt-1 text-sm text-muted-foreground">外观、启动行为、AI 服务商配置</p>
      </header>

      {toast && (
        <Card>
          <CardContent className="py-3 text-sm text-success">{toast}</CardContent>
        </Card>
      )}

      {/* 外观 */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Moon size={18} className="text-brand" /> 外观
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-5 pt-4">
          <div className="flex items-center justify-between">
            <div>
              <div className="text-sm font-medium text-foreground">主题</div>
              <div className="text-xs text-muted-foreground">深浅主题切换</div>
            </div>
            <Button variant="secondary" onClick={toggleTheme}>
              {theme === 'dark' ? <Sun size={16} /> : <Moon size={16} />}
              {theme === 'dark' ? '深色 / Dark' : '浅色 / Light'}
            </Button>
          </div>
          <div className="flex items-center justify-between">
            <div>
              <div className="text-sm font-medium text-foreground">语言</div>
              <div className="text-xs text-muted-foreground">中英双语无缝切换</div>
            </div>
            <div className="flex gap-2">
              <Button
                variant={language === 'zh-CN' ? 'primary' : 'secondary'}
                onClick={() => {
                  setLanguage('zh-CN');
                  patchSetting({ language: 'zh-CN' });
                }}
              >
                <Globe size={16} /> 中文
              </Button>
              <Button
                variant={language === 'en-US' ? 'primary' : 'secondary'}
                onClick={() => {
                  setLanguage('en-US');
                  patchSetting({ language: 'en-US' });
                }}
              >
                <Globe size={16} /> English
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* 启动行为 */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Power size={18} className="text-brand" /> 启动行为
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-5 pt-4">
          <div className="flex items-center justify-between">
            <div>
              <div className="text-sm font-medium text-foreground">关闭窗口时</div>
              <div className="text-xs text-muted-foreground">最小化到托盘继续运行，或直接退出</div>
            </div>
            <div className="flex gap-2">
              <Button
                size="sm"
                variant={saved?.closeBehavior === 'MinimizeToTray' ? 'primary' : 'secondary'}
                onClick={() => patchSetting({ closeBehavior: 'MinimizeToTray' })}
              >
                最小化到托盘
              </Button>
              <Button
                size="sm"
                variant={saved?.closeBehavior === 'Exit' ? 'primary' : 'secondary'}
                onClick={() => patchSetting({ closeBehavior: 'Exit' })}
              >
                直接退出
              </Button>
            </div>
          </div>
          <div className="flex items-center justify-between">
            <div>
              <div className="text-sm font-medium text-foreground">开机自启</div>
              <div className="text-xs text-muted-foreground">登录 Windows 时静默启动到托盘</div>
            </div>
            <Button
              size="sm"
              variant={saved?.runAtStartup ? 'primary' : 'secondary'}
              onClick={() => patchSetting({ runAtStartup: !saved?.runAtStartup })}
            >
              <Rocket size={14} />
              {saved?.runAtStartup ? '已启用' : '已禁用'}
            </Button>
          </div>
        </CardContent>
      </Card>

      {/* AI 服务商 */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Bot size={18} className="text-brand" /> AI 分析（可选）
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4 pt-4">
          <p className="text-xs text-muted-foreground">
            配置 OpenAI 兼容接口（DeepSeek / Qwen / Groq / Gemini 等）或 Anthropic Claude，
            对系统状态进行智能优化建议。默认关闭，仅在你主动分析时发送系统数据。
          </p>

          {/* 已保存服务商 */}
          <div className="space-y-2">
            {providers.map((p) => (
              <div
                key={p.id}
                className="flex items-center gap-3 rounded-lg border px-3 py-2 text-sm"
                style={{ borderColor: 'var(--border)' }}
              >
                <Bot size={16} className="text-brand" />
                <div className="flex-1">
                  <div className="text-foreground">
                    {p.name}
                    <span className="ml-2 text-xs text-muted-foreground">{p.providerType === 'AnthropicClaude' ? 'Claude' : 'OpenAI 兼容'}</span>
                  </div>
                  <div className="font-mono text-[11px] text-muted-foreground">
                    {p.apiBaseUrl} · {p.modelId}
                  </div>
                </div>
                <Button
                  size="sm"
                  variant="ghost"
                  onClick={() => handleTest(p)}
                  disabled={testing === p.id}
                >
                  <PlugZap size={14} /> {testing === p.id ? '测试中…' : '测试'}
                </Button>
                <Button size="sm" variant="ghost" onClick={() => handleDeleteProvider(p.id)}>
                  <Trash2 size={14} className="text-danger" />
                </Button>
              </div>
            ))}
          </div>

          {/* 新增/编辑表单 */}
          <div className="rounded-xl border p-4 space-y-3" style={{ borderColor: 'var(--border)' }}>
            <div className="text-sm font-medium text-foreground">新增服务商</div>
            <div className="grid grid-cols-2 gap-3">
              <input
                className="rounded-lg border bg-transparent px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-brand/40"
                style={{ borderColor: 'var(--border)' }}
                placeholder="名称（如 DeepSeek）"
                value={draft.name}
                onChange={(e) => setDraft({ ...draft, name: e.target.value })}
              />
              <input
                className="rounded-lg border bg-transparent px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-brand/40"
                style={{ borderColor: 'var(--border)' }}
                placeholder="模型 ID（如 deepseek-chat）"
                value={draft.modelId}
                onChange={(e) => setDraft({ ...draft, modelId: e.target.value })}
              />
              <input
                className="rounded-lg border bg-transparent px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-brand/40"
                style={{ borderColor: 'var(--border)' }}
                placeholder="API Base URL（如 https://api.deepseek.com）"
                value={draft.apiBaseUrl}
                onChange={(e) => setDraft({ ...draft, apiBaseUrl: e.target.value })}
              />
              <input
                className="rounded-lg border bg-transparent px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-brand/40"
                style={{ borderColor: 'var(--border)' }}
                placeholder="API Key"
                type="password"
                value={draft.encryptedApiKey}
                onChange={(e) => setDraft({ ...draft, encryptedApiKey: e.target.value })}
              />
              <select
                className="rounded-lg border bg-transparent px-3 py-2 text-sm text-foreground focus:outline-none"
                style={{ borderColor: 'var(--border)' }}
                value={draft.providerType}
                onChange={(e) =>
                  setDraft({ ...draft, providerType: e.target.value as 'OpenAICompatible' | 'AnthropicClaude' })
                }
              >
                <option value="OpenAICompatible">OpenAI 兼容</option>
                <option value="AnthropicClaude">Anthropic Claude</option>
              </select>
              <div />
            </div>
            <Button onClick={handleSaveProvider} disabled={!draft.name || !draft.apiBaseUrl}>
              <Plus size={16} /> 保存服务商
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
