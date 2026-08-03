// AI 分析命令 API 封装
import type { AIAnalysisResult, AIProviderConfig } from '@/types';
import { call } from './invoke';

export function getAIProviders(): Promise<AIProviderConfig[]> {
  return call('get_ai_providers');
}
export function saveAIProvider(provider: AIProviderConfig): Promise<AIProviderConfig> {
  return call('save_ai_provider', { provider });
}
export function deleteAIProvider(id: string): Promise<void> {
  return call('delete_ai_provider', { id });
}
export function testAIProvider(provider: AIProviderConfig): Promise<boolean> {
  return call('test_ai_provider', { provider });
}
export function runAIAnalysis(providerId?: string): Promise<AIAnalysisResult> {
  return call('run_ai_analysis', { providerId });
}
export function aiStatus(): Promise<{ configured: boolean; count: number }> {
  return call('ai_status');
}
