<script lang="ts">
  import { appState } from "../../../lib/appState.svelte";
  import { notifications } from "../../../lib/notifications.svelte";

  let { onClose } = $props<{ onClose: () => void }>();

  let selectedSpenderId = $state<number | "">("");
  let selectedCategoryId = $state<number | "">("");

  // Categorías disponibles (que no están ya asignadas a la persona seleccionada en el mes actual)
  let availableCategories = $derived(() => {
    if (selectedSpenderId === "") return [];

    const group = appState.activeSpenderGroups.find(
      (g) => g.spender.id === selectedSpenderId
    );

    if (!group) return appState.allCategoriesMaster;

    const activeCategoryIds = new Set(group.categories.map((c) => c.id));
    return appState.allCategoriesMaster.filter(
      (c) => !activeCategoryIds.has(c.id)
    );
  });

  function handleAdd() {
    if (selectedSpenderId === "" || selectedCategoryId === "") {
      notifications.show("Error", "Debes seleccionar una persona y una categoría", "warning");
      return;
    }

    const spender = appState.allSpenders.find(s => s.id === selectedSpenderId);
    const category = appState.allCategoriesMaster.find(c => c.id === selectedCategoryId);

    if (spender && category) {
      appState.addManualSpenderCategory(spender, category);
      notifications.show("Éxito", "Columna añadida a la tabla", "success");
      selectedCategoryId = ""; // Reiniciar la categoría para agregar otra rápidamente si quiere
    }
  }
</script>

<div class="flex flex-col h-full overflow-y-auto pr-2">
  <div class="flex items-center justify-between mb-4">
    <h3 class="font-bold text-sm uppercase opacity-70">
      Agregar a la Tabla
    </h3>
  </div>

  <div class="bg-base-200 p-4 rounded-box mb-6 border border-base-300">
    <h4 class="font-bold text-sm mb-3">Vincular Categoría a Persona</h4>

    <div class="form-control w-full mb-3">
      <label class="label pt-0 pb-1" for="spenderSelect">
        <span class="label-text text-xs font-medium opacity-70">Persona</span>
      </label>
      <select
        id="spenderSelect"
        class="select select-bordered select-sm w-full"
        bind:value={selectedSpenderId}
        onchange={() => { selectedCategoryId = ""; }}
      >
        <option value="">-- Elige una persona --</option>
        {#each appState.allSpenders as s (s.id)}
          <option value={s.id}>{s.name}</option>
        {/each}
      </select>
    </div>

    <!-- Mostrar opciones de categoría SOLO si ya seleccionó una Persona -->
    {#if selectedSpenderId !== ""}
      <div class="form-control w-full mb-4">
        <label class="label pt-0 pb-1" for="categorySelect">
          <span class="label-text text-xs font-medium opacity-70">Categoría</span>
        </label>
        
        {#if availableCategories().length === 0}
          <div class="text-xs text-error font-medium mt-1">
            Esta persona ya tiene todas las categorías disponibles asignadas en la tabla.
          </div>
        {:else}
          <select
            id="categorySelect"
            class="select select-bordered select-sm w-full"
            bind:value={selectedCategoryId}
          >
            <option value="">-- Elige una categoría --</option>
            {#each availableCategories() as c (c.id)}
              <option value={c.id}>{c.name}</option>
            {/each}
          </select>
        {/if}
      </div>

      <div class="flex gap-2">
        <button
          class="btn btn-primary btn-sm flex-1"
          onclick={handleAdd}
          disabled={selectedCategoryId === "" || availableCategories().length === 0}
        >
          Agregar a la tabla
        </button>
      </div>
    {/if}
  </div>

  <button
    class="btn btn-ghost btn-sm mt-auto w-full flex-shrink-0"
    onclick={onClose}
  >
    Cancelar
  </button>
</div>
