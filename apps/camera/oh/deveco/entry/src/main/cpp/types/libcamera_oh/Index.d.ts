export const mount: (content: object, filesDir: string) => void;
export const tick: () => void;
export const nextRequest: () => string;
export const setTopInset: (top: number) => void;
export const handlePermissionResult: (name: string, granted: boolean) => void;
export const handleCaptureSaved: (path: string, ok: boolean) => void;
export const back: () => boolean;
