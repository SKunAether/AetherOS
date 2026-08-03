// Provider 列表 Hook
import { useQuery } from '@tanstack/react-query';
import { getProviders } from '@/api/runtime';
import type { ProviderDescriptor } from '@/types';

export function useProviders() {
  return useQuery<ProviderDescriptor[]>({
    queryKey: ['providers'],
    queryFn: getProviders,
  });
}
