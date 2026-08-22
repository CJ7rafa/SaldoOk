<script lang="ts">
    import { notifications } from "../lib/notifications.svelte";
    import { Info, CheckCircle, AlertTriangle, XCircle, X } from "@lucide/svelte";
    import { fly } from "svelte/transition";

    function getIcon(type: string) {
        switch (type) {
            case 'success': return CheckCircle;
            case 'warning': return AlertTriangle;
            case 'error': return XCircle;
            default: return Info;
        }
    }

    function getColorClass(type: string) {
        switch (type) {
            case 'success': return 'text-success';
            case 'warning': return 'text-warning';
            case 'error': return 'text-error';
            default: return 'text-info';
        }
    }
    
    function getBorderClass(type: string) {
        switch (type) {
            case 'success': return 'border-success';
            case 'warning': return 'border-warning';
            case 'error': return 'border-error';
            default: return 'border-info';
        }
    }
    
    function getBgClass(type: string) {
        switch (type) {
            case 'success': return 'bg-success';
            case 'warning': return 'bg-warning';
            case 'error': return 'bg-error';
            default: return 'bg-info';
        }
    }
</script>

<div class="toast toast-top toast-end z-[100000] mt-4 mr-2 md:mt-16 md:mr-4">
    {#each notifications.toasts as toast (toast.id)}
        {@const Icon = getIcon(toast.type)}
        <!-- Reemplazando tu CSS #toastGlobalPvox con Tailwind y Svelte Transitions -->
        <div 
            in:fly={{ x: 100, duration: 400 }} 
            out:fly={{ x: 100, duration: 400 }}
            class="flex items-stretch bg-base-200 text-base-content rounded-lg shadow-xl overflow-hidden w-max max-w-[90vw] md:max-w-[25vw] border-l-4 {getBorderClass(toast.type)} relative"
        >
            <div class="flex items-center gap-4 p-4 flex-grow">
                <!-- Se monta el componente dinámicamente -->
                <div class="{getColorClass(toast.type)}">
                    <Icon size={24} />
                </div>
                <div class="flex flex-col gap-1">
                    <strong class="text-base leading-tight break-words">{toast.title}</strong>
                    <span class="text-sm opacity-70 break-words">{toast.message}</span>
                </div>
            </div>
            
            <!-- Botón de cerrar de perfil bajo -->
            <button 
                class="px-4 flex items-center justify-center hover:bg-base-300 transition-colors opacity-50 hover:opacity-100 border-l border-base-300"
                onclick={() => notifications.close(toast.id)}
            >
                <X size={20} />
            </button>

            <!-- La Barra de Temporizador -->
            <div class="absolute bottom-0 left-0 h-1 {getBgClass(toast.type)} animate-[shrink_5s_linear_forwards]"></div>
        </div>
    {/each}
</div>

<style>
    /* La animación de la barra la definimos aquí para mantenerla simple y robusta */
    @keyframes shrink {
        from { width: 100%; }
        to { width: 0%; }
    }
</style>
