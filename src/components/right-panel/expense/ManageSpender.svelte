<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { notifications } from "../../../lib/notifications.svelte";
  import { appState } from "../../../lib/appState.svelte";
  import type { Spender } from "../../../types";
  import { onMount } from "svelte";

  let { onClose, onSuccess } = $props<{
    onClose: () => void;
    onSuccess?: () => void;
  }>();

  // Estados para agregar
  let nameToAdd = $state("");
  let isAdding = $state(false);

  // Estados para editar
  let allSpenders = $state<Spender[]>([]);
  let isLoadingSpenders = $state(true);

  let selectedSpenderId = $state<number | "">("");
  let nameToEdit = $state("");
  let isEditing = $state(false);
  let isDeleting = $state(false);

  async function loadSpenders() {
    try {
      allSpenders = await invoke<Spender[]>("get_spenders");
      allSpenders.sort((a, b) => a.name.localeCompare(b.name));

      if (appState.selectedSpenderToEdit) {
        selectedSpenderId = appState.selectedSpenderToEdit.id;
      }
    } catch (e) {
      notifications.show(
        "Error",
        "No se pudieron cargar las personas",
        "error",
      );
    } finally {
      isLoadingSpenders = false;
    }
  }

  onMount(() => {
    loadSpenders();
  });

  // Reaccionar al cambio seleccionado
  $effect(() => {
    if (selectedSpenderId !== "") {
      const found = allSpenders.find((s) => s.id === selectedSpenderId);
      if (found) {
        nameToEdit = found.name;
      }
    } else {
      nameToEdit = "";
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
      await invoke("create_spender", {
        name: nameToAdd.trim(),
      });
      notifications.show(
        "¡Éxito!",
        `Persona "${nameToAdd.trim()}" creada correctamente.`,
        "success",
      );

      nameToAdd = "";
      await loadSpenders();

      if (onSuccess) onSuccess();
    } catch (err) {
      notifications.show("Error al guardar", String(err), "error");
    } finally {
      isAdding = false;
    }
  }

  async function handleEdit() {
    if (selectedSpenderId === "") return;
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
      await invoke("update_spender", {
        spender: {
          id: selectedSpenderId,
          name: nameToEdit.trim(),
          active: 1,
        }
      });
      notifications.show(
        "¡Éxito!",
        `Persona "${nameToEdit.trim()}" modificada correctamente.`,
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
    if (selectedSpenderId === "") return;

    // Verificar si la persona tiene gastos
    const hasExpenses = appState.allExpenses.some(
      (e) => e.spender_id === selectedSpenderId,
    );

    if (hasExpenses) {
      notifications.show(
        "No se puede eliminar",
        "La persona tiene registros de gastos en la base de datos.",
        "warning",
      );
      return;
    }

    notifications.confirmSafe(
      "Eliminar Persona",
      `¿Estás seguro de que quieres eliminar a "${nameToEdit.trim()}" permanentemente de la base de datos?`,
      3,
      async () => {
        isDeleting = true;
        try {
          await invoke("delete_spender", { id: selectedSpenderId });
          notifications.show(
            "Eliminado",
            "La persona ha sido eliminada correctamente.",
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
      Gestionar Personas
    </h3>
  </div>
  <!-- SECCIÓN: AGREGAR PERSONA -->
  <div class="bg-base-200 p-4 rounded-box mb-6 border border-base-300">
    <h4 class="font-bold text-sm mb-3">Agregar Nueva Persona</h4>

    <div class="form-control w-full mb-4">
      <input
        type="text"
        placeholder="Ej. Juan Pérez"
        class="input input-bordered input-sm w-full"
        bind:value={nameToAdd}
        disabled={isAdding}
        onkeydown={(e) => e.key === "Enter" && handleAdd()}
      />
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

  <!-- SECCIÓN: EDITAR PERSONA -->
  <div class="bg-base-200 p-4 rounded-box border border-base-300">
    <h4 class="font-bold text-sm mb-3">Editar Persona</h4>

    {#if isLoadingSpenders}
      <div class="flex justify-center p-4">
        <span class="loading loading-spinner loading-md text-primary"></span>
      </div>
    {:else}
      <div class="form-control w-full mb-4">
        <label class="label pt-0 pb-1" for="spenderSelect">
          <span class="label-text text-xs font-medium opacity-70"
            >Selecciona una persona</span
          >
        </label>
        <select
          id="spenderSelect"
          class="select select-bordered select-sm w-full"
          bind:value={selectedSpenderId}
          disabled={isEditing}
        >
          <option value="">-- Elige uno --</option>
          {#each allSpenders as s (s.id)}
            <option value={s.id}>{s.name}</option>
          {/each}
        </select>
      </div>

      {#if selectedSpenderId !== ""}
        <div class="form-control w-full mb-5">
          <label class="label pt-0 pb-1" for="spenderEditName">
            <span class="label-text text-xs font-medium opacity-70"
              >Nuevo Nombre</span
            >
          </label>
          <input
            id="spenderEditName"
            type="text"
            class="input input-bordered input-sm w-full"
            bind:value={nameToEdit}
            disabled={isEditing}
            onkeydown={(e) => e.key === "Enter" && handleEdit()}
          />
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
