<script lang="ts">
  import { ArrowLeft, UserPlus } from "@lucide/svelte";

  let {
    canGoBack,
    onBack,
    title,
    currentView,
    onRegisterSupplier,
    onAddSupplier,
    onManageCategories,
    onManageSpenders,
    onAddSpenderCategory,
  } = $props<{
    canGoBack: boolean;
    onBack: () => void;
    title: string;
    currentView?: string;
    onRegisterSupplier?: () => void;
    onAddSupplier?: () => void;
    onManageCategories?: () => void;
    onManageSpenders?: () => void;
    onAddSpenderCategory?: () => void;
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
    <div class="flex gap-2">
      <button class="btn btn-primary btn-sm" onclick={onRegisterSupplier}>
        <UserPlus size={16} />
        Gestionar Proveedores
      </button>

      <button class="btn btn-primary btn-sm" onclick={onAddSupplier}>
        <UserPlus size={16} />
        Agregar proveedor a la tabla
      </button>
    </div>
  {:else if currentView === "tablagastos"}
    <div class="flex gap-2">
      <button class="btn btn-primary btn-sm btn-outline" onclick={onManageCategories}>
        Gestionar Categoría
      </button>

      <button class="btn btn-primary btn-sm btn-outline" onclick={onManageSpenders}>
        Gestionar Persona
      </button>

      <button class="btn btn-primary btn-sm" onclick={onAddSpenderCategory}>
        Agregar a la tabla
      </button>
    </div>
  {/if}
</header>
