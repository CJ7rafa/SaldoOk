<script lang="ts">
  import { ArrowLeft, UserPlus } from "@lucide/svelte";

  let {
    canGoBack,
    onBack,
    title,
    currentView,
    onRegisterSupplier,
    onAddSupplier,
  } = $props<{
    canGoBack: boolean;
    onBack: () => void;
    title: string;
    currentView?: string;
    onRegisterSupplier?: () => void;
    onAddSupplier?: () => void;
  }>();
</script>

<header
  class="w-full h-16 bg-base-100 border-b border-base-300 flex items-center px-6 shrink-0 z-10 shadow-sm justify-between"
>
  <div class="flex items-center">
    {#if canGoBack}
      <button class="btn btn-ghost btn-sm mr-4" onclick={onBack}>
        <ArrowLeft size={18} />
        Volver
      </button>
    {/if}
    <h2 class="text-xl font-bold text-base-content">{title}</h2>
  </div>

  {#if currentView === "tablacompras"}
    <button class="btn btn-primary btn-sm" onclick={onRegisterSupplier}>
      <UserPlus size={16} />
      Gestionar Proveedores
    </button>

    <button class="btn btn-primary btn-sm" onclick={onAddSupplier}>
      <UserPlus size={16} />
      Agregar proveedor a la tabla
    </button>
  {/if}
</header>
