// Tauri IPC 统一调用封装：命令调用、错误归一化、事件订阅
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

/** 统一 IPC 调用入口，后端 AppError 序列化为 { code, message } */
export async function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (err) {
    const message =
      typeof err === 'string'
        ? err
        : (err as { message?: string })?.message ?? 'unknown error';
    throw new Error(message);
  }
}

/** 订阅后端事件，返回取消订阅函数 */
export async function onEvent<T = unknown>(
  eventName: string,
  handler: (payload: T) => void
): Promise<UnlistenFn> {
  return listen<T>(eventName, (event) => handler(event.payload));
}

/** 后端事件通道名（与 src-tauri lib.rs relay 一致） */
export const EVENT_CHANNEL = 'aetheros://event';
