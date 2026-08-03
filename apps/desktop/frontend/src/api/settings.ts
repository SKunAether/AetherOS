// 设置命令 API 封装
import type { AppSettings } from '@/types';
import { call } from './invoke';

/** 获取当前设置 */
export function getSettings(): Promise<AppSettings> {
  return call<AppSettings>('get_settings');
}

/** 保存设置 */
export function saveSettings(settings: AppSettings): Promise<AppSettings> {
  return call<AppSettings>('save_settings', { settings });
}
