// 后端事件订阅 Hook：订阅事件总线转发到前端的通道
import { useEffect } from 'react';
import { onEvent, EVENT_CHANNEL } from '@/api/invoke';
import type { AetherEvent } from '@/types';

/** 订阅后端事件，组件卸载时自动取消 */
export function useBackendEvent(handler: (event: AetherEvent) => void) {
  useEffect(() => {
    let unlisten: (() => void) | undefined;
    let cancelled = false;

    onEvent<AetherEvent>(EVENT_CHANNEL, (event) => {
      if (!cancelled) handler(event);
    }).then((un) => {
      if (cancelled) un();
      else unlisten = un;
    });

    return () => {
      cancelled = true;
      unlisten?.();
    };
  }, [handler]);
}
