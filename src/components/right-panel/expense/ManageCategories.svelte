<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { notifications } from "../../../lib/notifications.svelte";
  import { appState } from "../../../lib/appState.svelte";
  import type { ExpenseCategory } from "../../../types";
  import { onMount } from "svelte";

  let { onClose, onSuccess } = $props<{
    onClose: () => void;
    onSuccess?: () => void;
  }>();

  // Estados para agregar
  let nameToAdd = $state("");
  let colorToAdd = $state("BLUE");
  let isAdding = $state(false);

  // Estados para editar
  let allCategories = $state<ExpenseCategory[]>([]);
  let isLoadingCategories = $state(true);

  let selectedCategoryId = $state<number | "">("");
  let nameToEdit = $state("");
  let colorToEdit = $state("BLUE");
  let isEditing = $state(false);
  let isDeleting = $state(false);

  const CATEGORY_COLORS = [
    { id: "BLACK", class: "bg-slate-800" },
    { id: "RED", class: "bg-red-500" },
    { id: "BLUE", class: "bg-blue-500" },
    { id: "GREEN", class: "bg-emerald-500" },
    { id: "ORANGE", class: "bg-orange-500" },
  ];

  async function loadCategories() {
    try {
      allCategories = await invoke<ExpenseCategory[]>("get_all_expense_categories");
      allCategories.sort((a, b) => a.name.localeCompare(b.name));

      if (appState.selectedCategoryToEdit) {
        selectedCategoryId = appState.selectedCategoryToEdit.id;
      }
    } catch (e) {
      notifications.show(
        "Error",
        "No se pudieron cargar las categorías",
        "error",
      );
    } finally {
      isLoadingCategories = false;
    }
  }

  onMount(() => {
    loadCategories();
  });

  // Reaccionar al cambio de categoría seleccionada
  $effect(() => {
    if (selectedCategoryId !== "") {
      const found = allCategories.find((c) => c.id === selectedCategoryId);
      if (found) {
        nameToEdit = found.name;
        colorToEdit = found.color || "BLUE";
      }
    } else {
      nameToEdit = "";
      colorToEdit = "BLUE";
    }
  });

  async function handleAdd() {
    if (!nameToAdd.trim()) {
      notifications.show(
        "Campo vacío",
        "El nombre no puede estar vacío",
        "warning",
      );
      return;
    }

    isAdding = true;
    try {
      await invoke("create_expense_category", {
        category: {
          id: 0,
          name: nameToAdd.trim(),
          color: colorToAdd,
          active: 1
        }
      });
      notifications.show(
        "¡Éxito!",
        `Categoría "${nameToAdd.trim()}" creada correctamente.`,
        "success",
      );

      // Limpiar y recargar
      nameToAdd = "";
      colorToAdd = "BLUE";
      await loadCategories();

      if (onSuccess) onSuccess();
    } catch (err) {
      notifications.show("Error al guardar", String(err), "error");
    } finally {
      isAdding = false;
    }
  }

  async function handleEdit() {
    if (selectedCategoryId === "") return;
    if (!nameToEdit.trim()) {
      notifications.show(
        "Campo vacío",
        "El nombre no puede estar vacío",
        "warning",
      );
      return;
    }

    isEditing = true;
    try {
      await invoke("update_expenses_categories", {
        category: {
          id: selectedCategoryId,
          name: nameToEdit.trim(),
          active: 1,
          color: colorToEdit,
        }
      });
      notifications.show(
        "¡Éxito!",
        `Categoría "${nameToEdit.trim()}" modificada correctamente.`,
        "success",
      );

      if (onSuccess) onSuccess();
      else onClose();
    } catch (err) {
      notifications.show("Error al guardar", String(err), "error");
    } finally {
      isEditing = false;
    }
  }

  async function handleDelete() {
    if (selectedCategoryId === "") return;

    // Verificar si la categoría tiene gastos asociados
    const hasExpenses = appState.allExpenses.some(
      (e) => e.category_id === selectedCategoryId,
    );

    if (hasExpenses) {
      notifications.show(
        "No se puede eliminar",
        "La categoría tiene registros de gastos en la base de datos.",
        "warning",
      );
      return;
    }

    notifications.confirmSafe(
      "Eliminar Categoría",
      `¿Estás seguro de que quieres eliminar la categoría "${nameToEdit.trim()}" permanentemente de la base de datos?`,
      3,
      async () => {
        isDeleting = true;
        try {
          await invoke("delete_expenses_categories", { id: selectedCategoryId });
          notifications.show(
            "Eliminado",
            "La categoría ha sido eliminada correctamente.",
            "success",
          );
          notifications.closeConfirmation();

          appState.loadData();
          if (onSuccess) onSuccess();
          else onClose();
        } catch (err) {
          notifications.show("Error al eliminar", String(err), "error");
          notifications.closeConfirmation();
        } finally {
          isDeleting = false;
        }
      },
    );
  }
</script>

<div class="flex flex-col h-full overflow-y-auto pr-2">
  <div class="flex items-center justify-between mb-4">
    <h3 class="font-bold text-sm uppercase opacity-70">
      Gestionar Categorías
    </h3>
  </div>
  <!-- SECCIÓN: AGREGAR CATEGORÍA -->
  <div class="bg-base-200 p-4 rounded-box mb-6 border border-base-300">
    <h4 class="font-bold text-sm mb-3">Agregar Nueva Categoría</h4>

    <div class="form-control w-full mb-3">
      <input
        type="text"
        placeholder="Ej. Transporte"
        class="input input-bordered input-sm w-full"
        bind:value={nameToAdd}
        disabled={isAdding}
        onkeydown={(e) => e.key === "Enter" && handleAdd()}
      />
    </div>

    <div class="form-control w-full mb-4">
      <label class="label pt-0 pb-2">
        <span class="label-text text-xs font-medium opacity-70">Color</span>
      </label>
      <div class="flex gap-3 px-1">
        {#each CATEGORY_COLORS as c}
          <button
            type="button"
            class="w-6 h-6 rounded-md shadow-sm {c.class} transition-all {colorToAdd ===
            c.id
              ? 'ring-2 ring-offset-2 ring-base-content scale-110'
              : 'opacity-50 hover:opacity-100 hover:scale-110'}"
            onclick={() => (colorToAdd = c.id)}
            disabled={isAdding}
            title={c.id}
          ></button>
        {/each}
      </div>
    </div>

    <div class="flex gap-2">
      <button
        class="btn btn-primary btn-sm flex-1"
        onclick={handleAdd}
        disabled={isAdding}
      >
        {#if isAdding}
          <span class="loading loading-spinner loading-xs"></span>
        {/if}
        Agregar
      </button>
    </div>
  </div>

  <div class="divider text-xs opacity-50 mt-0 mb-6">O</div>

  <!-- SECCIÓN: EDITAR CATEGORÍA -->
  <div class="bg-base-200 p-4 rounded-box border border-base-300">
    <h4 class="font-bold text-sm mb-3">Editar Categoría</h4>

    {#if isLoadingCategories}
      <div class="flex justify-center p-4">
        <span class="loading loading-spinner loading-md text-primary"></span>
      </div>
    {:else}
      <div class="form-control w-full mb-4">
        <label class="label pt-0 pb-1" for="categorySelect">
          <span class="label-text text-xs font-medium opacity-70"
            >Selecciona una categoría</span
          >
        </label>
        <select
          id="categorySelect"
          class="select select-bordered select-sm w-full"
          bind:value={selectedCategoryId}
          disabled={isEditing}
        >
          <option value="">-- Elige una --</option>
          {#each allCategories as c (c.id)}
            <option value={c.id}>{c.name}</option>
          {/each}
        </select>
      </div>

      {#if selectedCategoryId !== ""}
        <div class="form-control w-full mb-3">
          <label class="label pt-0 pb-1" for="categoryEditName">
            <span class="label-text text-xs font-medium opacity-70"
              >Nuevo Nombre</span
            >
          </label>
          <input
            id="categoryEditName"
            type="text"
            class="input input-bordered input-sm w-full"
            bind:value={nameToEdit}
            disabled={isEditing}
            onkeydown={(e) => e.key === "Enter" && handleEdit()}
          />
        </div>

        <div class="form-control w-full mb-5">
          <label class="label pt-0 pb-2">
            <span class="label-text text-xs font-medium opacity-70">Color</span>
          </label>
          <div class="flex gap-3 px-1">
            {#each CATEGORY_COLORS as c}
              <button
                type="button"
                class="w-6 h-6 rounded-md shadow-sm {c.class} transition-all {colorToEdit ===
                c.id
                  ? 'ring-2 ring-offset-2 ring-base-content scale-110'
                  : 'opacity-50 hover:opacity-100 hover:scale-110'}"
                onclick={() => (colorToEdit = c.id)}
                disabled={isEditing}
                title={c.id}
              ></button>
            {/each}
          </div>
        </div>

        <div class="flex flex-col gap-2">
          <button
            class="btn btn-success btn-sm w-full text-success-content"
            onclick={handleEdit}
            disabled={isEditing || !nameToEdit.trim() || isDeleting}
          >
            {#if isEditing}
              <span class="loading loading-spinner loading-xs"></span>
            {/if}
            Guardar
          </button>

          <button
            class="btn btn-outline btn-error btn-sm w-full"
            onclick={handleDelete}
            disabled={isEditing || isDeleting}
          >
            Eliminar
          </button>
        </div>
      {/if}
    {/if}
  </div>

  <button
    class="btn btn-ghost btn-sm mt-auto w-full flex-shrink-0"
    onclick={onClose}
    disabled={isAdding}
  >
    Cancelar
  </button>
</div>
