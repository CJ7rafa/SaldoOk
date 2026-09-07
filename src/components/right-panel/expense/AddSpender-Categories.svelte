<script lang="ts">
  import { appState } from "../../../lib/appState.svelte";
  import { notifications } from "../../../lib/notifications.svelte";

  let { onClose } = $props<{ onClose: () => void }>();

  let selectedSpenderId = $state<number | "">("");
  let selectedCategoryId = $state<number | "">("");

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
      >
        <option value="">-- Elige una persona --</option>
        {#each appState.allSpenders as s (s.id)}
          <option value={s.id}>{s.name}</option>
        {/each}
      </select>
    </div>

    <div class="form-control w-full mb-4">
      <label class="label pt-0 pb-1" for="categorySelect">
        <span class="label-text text-xs font-medium opacity-70">Categoría</span>
      </label>
      <select
        id="categorySelect"
        class="select select-bordered select-sm w-full"
        bind:value={selectedCategoryId}
      >
        <option value="">-- Elige una categoría --</option>
        {#each appState.allCategoriesMaster as c (c.id)}
          <option value={c.id}>{c.name}</option>
        {/each}
      </select>
    </div>

    <div class="flex gap-2">
      <button
        class="btn btn-primary btn-sm flex-1"
        onclick={handleAdd}
      >
        Agregar a la tabla
      </button>
    </div>
  </div>

  <button
    class="btn btn-ghost btn-sm mt-auto w-full flex-shrink-0"
    onclick={onClose}
  >
    Cancelar
  </button>
</div>
