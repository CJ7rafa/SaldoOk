export type ToastType = 'info' | 'success' | 'warning' | 'error';

export interface Toast {
    id: number;
    title: string;
    message: string;
    type: ToastType;
}

let toasts = $state<Toast[]>([]);
let nextId = 0;

export const notifications = {
    get toasts() {
        return toasts;
    },
    show(title: string, message: string, type: ToastType = 'info') {
        const id = nextId++;
        toasts.push({ id, title, message, type });
        
        // El auto-cierre
        setTimeout(() => {
            this.close(id);
        }, 5000);
    },
    close(id: number) {
        toasts = toasts.filter(t => t.id !== id);
    }
};
