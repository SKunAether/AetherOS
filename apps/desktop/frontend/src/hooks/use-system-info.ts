// 系统信息 Hook：TanStack Query 缓存，30 秒自动刷新
import { useQuery } from '@tanstack/react-query';
import { getSystemInfo } from '@/api/system';
import type { SystemInfo } from '@/types';

export function useSystemInfo() {
  return useQuery<SystemInfo>({
    queryKey: ['system-info'],
    queryFn: getSystemInfo,
    refetchInterval: 30_000,
  });
}
