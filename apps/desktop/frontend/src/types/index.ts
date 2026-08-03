// TypeScript 全局类型定义，与后端 Rust 数据结构严格对齐
// （对应 crates/aether-core 中 serde camelCase 序列化契约）

/** 系统信息快照（对应 aether-system SystemInfo） */
export interface SystemInfo {
  machineName: string;
  osVersion: string;
  osBuild: number;
  processorCount: number;
  physicalMemoryBytes: number;
  availableMemoryBytes: number;
  systemDrive: string;
  systemDriveTotalBytes: number;
  systemDriveFreeBytes: number;
  uptimeMs: number;
  isAdministrator: boolean;
}

/** 能力类型枚举 */
export type CapabilityType = 'cleaner' | 'optimizer' | 'monitor' | 'recovery';

/** 风险等级 */
export type RiskLevel = 'Low' | 'Medium' | 'High' | 'Critical';

/** Provider 动作定义 */
export interface ActionDef {
  id: string;
  name: string;
  description: string;
  riskLevel: RiskLevel;
  requiresAdministrator: boolean;
  isReversible: boolean;
}

/** Provider 描述符 */
export interface ProviderDescriptor {
  id: string;
  name: string;
  description: string;
  capabilityType: CapabilityType;
  actions: ActionDef[];
  isBuiltin: boolean;
}

/** 应用设置（对应 AppSettings） */
export interface AppSettings {
  closeBehavior: 'MinimizeToTray' | 'Exit';
  runAtStartup: boolean;
  enableScheduledScan: boolean;
  scheduledScanTime: string;
  enableNotifications: boolean;
  checkUpdatesAutomatically: boolean;
  updateChannel: 'stable' | 'preview';
  language: 'zh-CN' | 'en-US';
  enableAIAnalysis: boolean;
  activeAIProviderId: string | null;
}

/** 统一事件负载（对应 AetherEvent） */
export interface AetherEvent<T = unknown> {
  kind: 'progress' | 'status-changed' | 'notification' | 'scan-completed' | 'execution-completed';
  source: string;
  timestampMs: number;
  payload: T;
}

/** 任务信息 */
export interface TaskInfo {
  id: string;
  providerId: string;
  kind: 'scan' | 'execute';
  status: 'pending' | 'running' | 'completed' | 'failed' | 'cancelled';
  startedAt: string;
  message: string;
}

/** 监控快照（对应 MonitorSnapshot） */
export interface MonitorSnapshot {
  timestamp: string;
  cpuUsagePercent: number;
  memoryTotalBytes: number;
  memoryUsedBytes: number;
  memoryUsagePercent: number;
  diskTotalBytes: number;
  diskFreeBytes: number;
  diskUsagePercent: number;
  uptimeMs: number;
  processorCount: number;
}

/** 清理扫描项（对应 CleanerItem） */
export interface CleanerItem {
  id: string;
  name: string;
  description: string;
  path: string;
  estimatedBytes: number;
  riskLevel: RiskLevel;
  requiresAdministrator: boolean;
  isSelectedByDefault: boolean;
}

/** 清理扫描结果（对应 CleanerScanResult） */
export interface CleanerScanResult {
  items: CleanerItem[];
  totalEstimatedBytes: number;
  completedAt: string;
}

/** 清理执行项结果 */
export interface CleanerExecutionItemResult {
  itemId: string;
  name: string;
  path: string;
  beforeBytes: number;
  afterBytes: number;
  releasedBytes: number;
  deletedFileCount: number;
  skippedFileCount: number;
  succeeded: boolean;
  requiresAdministrator: boolean;
  errorMessage?: string | null;
}

/** 清理执行结果（对应 CleanerExecutionResult） */
export interface CleanerExecutionResult {
  items: CleanerExecutionItemResult[];
  totalReleasedBytes: number;
  totalDeletedFileCount: number;
  totalSkippedFileCount: number;
  succeeded: boolean;
  completedAt: string;
}

/** 电源计划（对应 PowerPlanInfo） */
export interface PowerPlanInfo {
  id: string;
  name: string;
  isActive: boolean;
  isBuiltIn: boolean;
  category: string;
}

/** 电源计划状态（对应 PowerPlanState） */
export interface PowerPlanState {
  plans: PowerPlanInfo[];
  activePlanId?: string | null;
  checkedAt: string;
}

/** 休眠状态（对应 HibernateState） */
export interface HibernateState {
  isEnabled: boolean;
  isHibernateFilePresent: boolean;
  hibernateFileBytes: number;
  systemDrive: string;
  checkedAt: string;
}

/** 启动项（对应 StartupItem） */
export interface StartupItem {
  name: string;
  command: string;
  scope: string;
  isEnabled: boolean;
}

/** 操作审计记录（对应 SystemActionRecord） */
export interface SystemActionRecord {
  id: string;
  ruleId: string;
  displayName: string;
  action: string;
  previousState: string;
  resultState: string;
  succeeded: boolean;
  exitCode: number;
  errorMessage?: string | null;
  executedAt: string;
}

/** 还原点（对应 RestorePointInfo） */
export interface RestorePointInfo {
  sequenceNumber: number;
  description: string;
  creationTime: string;
  restorePointType: string;
}

/** 页面文件条目（对应 PageFileEntry） */
export interface PageFileEntry {
  path: string;
  initialSizeMb: number;
  maximumSizeMb: number;
  currentFileBytes: number;
  exists: boolean;
}

/** 页面文件配置（对应 PageFileConfiguration） */
export interface PageFileConfiguration {
  isAutomaticallyManaged: boolean;
  entries: PageFileEntry[];
  totalCurrentFileBytes: number;
  restartRequired: boolean;
  checkedAt: string;
}

/** AI 服务商类型 */
export type AIProviderType = 'OpenAICompatible' | 'AnthropicClaude';

/** AI 服务商配置 */
export interface AIProviderConfig {
  id: string;
  name: string;
  providerType: AIProviderType;
  apiBaseUrl: string;
  encryptedApiKey: string;
  modelId: string;
  isEnabled: boolean;
  isDefault: boolean;
}

/** AI 建议 */
export interface AIRecommendation {
  action: string;
  reason: string;
  impact: string;
  risk: string;
  priority: string;
  isSelected: boolean;
}

/** AI 分析章节 */
export interface AIAnalysisSection {
  module: string;
  title: string;
  analysis: string;
  recommendations: AIRecommendation[];
}

/** AI 分析结果 */
export interface AIAnalysisResult {
  summary: string;
  riskLevel: string;
  urgency: string;
  sections: AIAnalysisSection[];
  generatedAt: string;
  providerName: string;
  modelId: string;
  isSuccessful: boolean;
  errorMessage?: string | null;
}
