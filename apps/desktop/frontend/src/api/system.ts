// 系统命令 API 封装
import type { SystemInfo } from '@/types';
import { call } from './invoke';

/** 获取系统信息 */
export function getSystemInfo(): Promise<SystemInfo> {
  return call<SystemInfo>('get_system_info');
}

/** 用系统默认浏览器打开外部链接 */
export function openExternal(url: string): Promise<void> {
  return call('open_external', { url });
}
