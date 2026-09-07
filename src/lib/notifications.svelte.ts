export type ToastType = 'info' | 'success' | 'warning' | 'error';

export interface Toast {
    id: number;
    title: string;
    message: string;
    type: ToastType;
}

export interface ConfirmationRequest {
    title: string;
    message: string;
    seconds: number;
    onConfirm: () => void | Promise<void>;
}

let toasts = $state<Toast[]>([]);
let nextId = 0;
let currentConfirmation = $state<ConfirmationRequest | null>(null);

export const notifications = {
    get toasts() {
        return toasts;
    },
    get confirmation() {
        return currentConfirmation;
    },
    show(title: string, message: string, type: ToastType = 'info') {
        const id = nextId++;
        toasts.push({ id, title, message, type });

        // El auto-cierre
        setTimeout(() => {
            this.close(id);
        }, 4000);
    },
    close(id: number) {
        toasts = toasts.filter(t => t.id !== id);
    },
    confirmSafe(title: string, message: string, seconds: number, onConfirm: () => void | Promise<void>) {
        currentConfirmation = { title, message, seconds, onConfirm };
    },
    closeConfirmation() {
        currentConfirmation = null;
    }
};
