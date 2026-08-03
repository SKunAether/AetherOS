// 运行时命令 API 封装
import type { ProviderDescriptor } from '@/types';
import { call } from './invoke';

/** 列出所有已注册 Provider */
export function getProviders(): Promise<ProviderDescriptor[]> {
  return call<ProviderDescriptor[]>('get_providers');
}

/** 同步执行 Provider 动作 */
export function providerExecute(
  providerId: string,
  action: string,
  params: Record<string, unknown> = {}
): Promise<unknown> {
  return call('provider_execute', { providerId, action, params });
}

/** 执行 Provider 扫描 */
export function providerScan(
  providerId: string,
  params: Record<string, unknown> = {}
): Promise<unknown> {
  return call('provider_scan', { providerId, params });
}

/** 链路探测 */
export function ping(): Promise<string> {
  return call<string>('ping');
}
