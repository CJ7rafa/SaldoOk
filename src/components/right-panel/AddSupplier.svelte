<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import type { Supplier } from "../../types";

  let { activeSuppliers, onClose, onAdd } = $props<{
    activeSuppliers: Supplier[];
    onClose: () => void;
    onAdd: (s: Supplier) => void;
  }>();

  let allSuppliers = $state<Supplier[]>([]);
  let isLoading = $state(true);

  // Computed: proveedores que aún NO han sido agregados a la tabla
  let availableSuppliers = $derived(
    allSuppliers.filter(
      (s) => !activeSuppliers.some((a: Supplier) => a.id === s.id),
    ),
  );

  onMount(async () => {
    try {
      allSuppliers = await invoke<Supplier[]>("get_suppliers");
    } catch (e) {
      console.error(e);
    } finally {
      isLoading = false;
    }
  });

  function getTextColor(color: string) {
    switch (color) {
      case "RED":
        return "text-red-500";
      case "BLUE":
        return "text-blue-500";
      case "GREEN":
        return "text-emerald-500";
      case "ORANGE":
        return "text-orange-500";
      default:
        return "";
    }
  }
</script>

<div class="flex flex-col h-full">
  <h3 class="font-bold mb-4 text-sm uppercase opacity-70">
    Seleccionar Proveedor
  </h3>

  <div class="flex-1 overflow-y-auto space-y-2 mb-4 pr-2">
    {#if isLoading}
      <div class="flex justify-center p-4">
        <span class="loading loading-spinner loading-md"></span>
      </div>
    {:else if availableSuppliers.length === 0}
      <p class="text-sm opacity-50 text-center p-4">
        No hay proveedores disponibles o todos ya están en la tabla.
      </p>
    {:else}
      {#each availableSuppliers as supplier}
        <button
          class="w-full text-left p-3 rounded-lg bg-base-200 hover:bg-primary/20 transition-colors border border-transparent hover:border-primary/30"
          onclick={() => onAdd(supplier)}
        >
          <span class="font-medium {getTextColor(supplier.color)}"
            >{supplier.name}</span
          >
        </button>
      {/each}
    {/if}
  </div>

  <div class="mt-auto pt-4 flex">
    <button class="btn btn-ghost w-full" onclick={onClose}> Cancelar </button>
  </div>
</div>
