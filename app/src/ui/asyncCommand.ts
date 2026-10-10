export async function invokeWithTimeout<T>(
  command: string,
  invokeCommand: () => Promise<T>,
  timeoutMs = 15_000,
): Promise<T> {
  let timeout: ReturnType<typeof setTimeout> | undefined;
  try {
    return await Promise.race([
      invokeCommand(),
      new Promise<never>((_, reject) => {
        timeout = setTimeout(() => {
          reject(new Error(`O comando ${command} ainda não respondeu após ${Math.ceil(timeoutMs / 1000)} segundos.`));
        }, timeoutMs);
      }),
    ]);
  } finally {
    if (timeout !== undefined) clearTimeout(timeout);
  }
}
