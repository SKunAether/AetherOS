// 设置 Hook：TanStack Query 缓存 + 变更保存
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { getSettings, saveSettings } from '@/api/settings';
import type { AppSettings } from '@/types';

export function useSettings() {
  const queryClient = useQueryClient();
  const query = useQuery<AppSettings>({
    queryKey: ['settings'],
    queryFn: getSettings,
  });

  const mutation = useMutation<AppSettings, Error, AppSettings>({
    mutationFn: saveSettings,
    onSuccess: (saved) => {
      queryClient.setQueryData(['settings'], saved);
    },
  });

  return { query, mutation };
}
