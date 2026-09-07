<script lang="ts">
  import { notifications } from "../lib/notifications.svelte";
  import { onDestroy } from "svelte";
  import { AlertTriangle } from "@lucide/svelte";

  let ticksRestantes = $state(0);
  let totalTicks = $state(0);
  let interval: ReturnType<typeof setInterval> | undefined;
  let isProcessing = $state(false);

  // Reaccionar cuando aparece una confirmación
  $effect(() => {
    const req = notifications.confirmation;
    if (req) {
      isProcessing = false;
      ticksRestantes = req.seconds * 10;
      totalTicks = req.seconds * 10;
      
      clearInterval(interval);
      interval = setInterval(() => {
        if (ticksRestantes > 0) {
          ticksRestantes--;
        }
      }, 100);
    } else {
      clearInterval(interval);
    }
  });

  onDestroy(() => {
    clearInterval(interval);
  });

  let porcentaje = $derived(
    totalTicks > 0 ? ((totalTicks - ticksRestantes) / totalTicks) * 100 : 0
  );

  let segundosVisuales = $derived(Math.ceil(ticksRestantes / 10));

  function handleCancel() {
    notifications.closeConfirmation();
  }

  async function handleConfirm() {
    if (ticksRestantes > 0 || isProcessing) return;
    isProcessing = true;
    if (notifications.confirmation) {
      await notifications.confirmation.onConfirm();
    }
  }
</script>

{#if notifications.confirmation}
  <div class="fixed inset-0 bg-black/60 backdrop-blur-sm z-[9999] flex items-center justify-center p-4">
    <div class="bg-base-100 rounded-box max-w-md w-full p-6 shadow-2xl flex flex-col gap-4 border border-error/20">
      <h3 class="text-xl font-bold text-error flex items-center gap-2">
        <AlertTriangle size={24} />
        {notifications.confirmation.title}
      </h3>
      
      <p class="text-base-content/80">
        {notifications.confirmation.message}
      </p>

      <div class="w-full bg-base-300 rounded-full h-2 mt-2 overflow-hidden relative">
        <div 
          class="bg-error h-full transition-all duration-100 ease-linear absolute left-0 top-0" 
          style="width: {porcentaje}%"
        ></div>
      </div>

      <div class="flex justify-end gap-3 mt-4">
        <button class="btn btn-ghost" onclick={handleCancel} disabled={isProcessing}>
          Cancelar
        </button>
        
        <button 
          class="btn btn-error" 
          disabled={ticksRestantes > 0 || isProcessing}
          onclick={handleConfirm}
        >
          {#if isProcessing}
            <span class="loading loading-spinner loading-sm"></span>
            Procesando...
          {:else if ticksRestantes > 0}
            Espere {segundosVisuales}s...
          {:else}
            Confirmar acción
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}
