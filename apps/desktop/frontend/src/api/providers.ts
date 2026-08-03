// 业务功能 API 封装：基于通用 provider_execute 调用各能力 Provider
import type {
  CleanerExecutionResult,
  CleanerScanResult,
  HibernateState,
  MonitorSnapshot,
  PageFileConfiguration,
  PowerPlanState,
  RestorePointInfo,
  StartupItem,
  SystemActionRecord,
} from '@/types';
import { call } from './invoke';

/** 调用 Provider 执行动作 */
function execute<T>(
  providerId: string,
  action: string,
  params: Record<string, unknown> = {}
): Promise<T> {
  return call<T>('provider_execute', { providerId, action, params });
}

// ---- 监控 ----
export function getMonitorSnapshot(): Promise<MonitorSnapshot> {
  return execute('aether.monitor', 'snapshot');
}

// ---- 清理 ----
export function cleanerScan(): Promise<CleanerScanResult> {
  return execute('aether.cleaner', 'scan');
}
export function cleanerExecute(selectedIds: string[]): Promise<CleanerExecutionResult> {
  return execute('aether.cleaner', 'execute', { selectedIds });
}

// ---- 优化 ----
export function getPowerPlans(): Promise<PowerPlanState> {
  return execute('aether.optimizer', 'get_power_plans');
}
export function activatePowerPlan(planId: string): Promise<SystemActionRecord> {
  return execute('aether.optimizer', 'activate_power_plan', { planId });
}
export function getHibernateState(): Promise<HibernateState> {
  return execute('aether.optimizer', 'get_hibernate_state');
}
export function setHibernate(enabled: boolean): Promise<SystemActionRecord> {
  return execute('aether.optimizer', 'set_hibernate', { enabled });
}
export function getStartupItems(): Promise<StartupItem[]> {
  return execute('aether.optimizer', 'get_startup_items');
}
export function setStartupItem(name: string, command: string): Promise<{ ok: boolean }> {
  return execute('aether.optimizer', 'set_startup_item', { name, command });
}
export function deleteStartupItem(name: string, scope: string): Promise<{ ok: boolean }> {
  return execute('aether.optimizer', 'delete_startup_item', { name, scope });
}

// ---- 虚拟内存 ----
export function getVirtualMemory(): Promise<PageFileConfiguration> {
  return execute('aether.optimizer', 'get_virtual_memory');
}
export function applyVirtualMemory(
  automatic: boolean,
  entries: string[]
): Promise<SystemActionRecord> {
  return execute('aether.optimizer', 'apply_virtual_memory', { automatic, entries });
}

// ---- 恢复 ----
export function getHistory(max = 50): Promise<SystemActionRecord[]> {
  return execute('aether.recovery', 'history', { max });
}
export function getRestorePoints(): Promise<RestorePointInfo[]> {
  return execute('aether.recovery', 'restore_points');
}
export function createRestorePoint(description: string): Promise<{ exitCode: number }> {
  return execute('aether.recovery', 'create_restore_point', { description });
}
