<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { notifications } from "../../../lib/notifications.svelte";
  import type { Supplier } from "../../../types";
  import { onMount } from "svelte";

  let { onClose, onSuccess, initialEditSupplier } = $props<{
    onClose: () => void;
    onSuccess?: () => void;
    initialEditSupplier?: Supplier | null;
  }>();

  // Estados para agregar
  let nameToAdd = $state("");
  let colorToAdd = $state("BLACK");
  let isAdding = $state(false);

  // Estados para editar
  let allSuppliers = $state<Supplier[]>([]);
  let isLoadingSuppliers = $state(true);

  let selectedSupplierId = $state<number | "">("");
  let nameToEdit = $state("");
  let colorToEdit = $state("BLACK");
  let isEditing = $state(false);

  const SUPPLIER_COLORS = [
    { id: "BLACK", class: "bg-slate-800" },
    { id: "RED", class: "bg-red-500" },
    { id: "BLUE", class: "bg-blue-500" },
    { id: "GREEN", class: "bg-emerald-500" },
    { id: "ORANGE", class: "bg-orange-500" },
  ];

  async function loadSuppliers() {
    try {
      allSuppliers = await invoke<Supplier[]>("get_suppliers");
      allSuppliers.sort((a, b) => a.name.localeCompare(b.name));

      if (initialEditSupplier) {
        selectedSupplierId = initialEditSupplier.id;
      }
    } catch (e) {
      notifications.show(
        "Error",
        "No se pudieron cargar los proveedores",
        "error",
      );
    } finally {
      isLoadingSuppliers = false;
    }
  }

  onMount(() => {
    loadSuppliers();
  });

  // Reaccionar al cambio de proveedor seleccionado
  $effect(() => {
    if (selectedSupplierId !== "") {
      const found = allSuppliers.find((s) => s.id === selectedSupplierId);
      if (found) {
        nameToEdit = found.name;
        colorToEdit = found.color || "BLACK";
      }
    } else {
      nameToEdit = "";
      colorToEdit = "BLACK";
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
      await invoke("create_supplier", {
        name: nameToAdd.trim(),
        color: colorToAdd,
      });
      notifications.show(
        "¡Éxito!",
        `Proveedor "${nameToAdd.trim()}" creado correctamente.`,
        "success",
      );

      // Limpiar y recargar
      nameToAdd = "";
      colorToAdd = "BLACK";
      await loadSuppliers();

      if (onSuccess) onSuccess();
    } catch (err) {
      notifications.show("Error al guardar", String(err), "error");
    } finally {
      isAdding = false;
    }
  }

  async function handleEdit() {
    if (selectedSupplierId === "") return;
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
      // Usamos el comando update_supplier
      await invoke("update_supplier", {
        id: selectedSupplierId,
        name: nameToEdit.trim(),
        active: 1,
        color: colorToEdit,
      });
      notifications.show(
        "¡Éxito!",
        `Proveedor "${nameToEdit.trim()}" modificado correctamente.`,
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
</script>

<div class="flex flex-col h-full overflow-y-auto pr-2">
  <div class="flex items-center justify-between mb-4">
    <h3 class="font-bold text-sm uppercase opacity-70">
      Gestionar Proveedores
    </h3>
  </div>
  <!-- SECCIÓN: AGREGAR PROVEEDOR -->
  <div class="bg-base-200 p-4 rounded-box mb-6 border border-base-300">
    <h4 class="font-bold text-sm mb-3">Agregar Nuevo Proveedor</h4>

    <div class="form-control w-full mb-3">
      <input
        type="text"
        placeholder="Ej. Distribuidora XYZ"
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
        {#each SUPPLIER_COLORS as c}
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

  <!-- SECCIÓN: EDITAR PROVEEDOR -->
  <div class="bg-base-200 p-4 rounded-box border border-base-300">
    <h4 class="font-bold text-sm mb-3">Editar Proveedor</h4>

    {#if isLoadingSuppliers}
      <div class="flex justify-center p-4">
        <span class="loading loading-spinner loading-md text-primary"></span>
      </div>
    {:else}
      <div class="form-control w-full mb-4">
        <label class="label pt-0 pb-1" for="supplierSelect">
          <span class="label-text text-xs font-medium opacity-70"
            >Selecciona un proveedor</span
          >
        </label>
        <select
          id="supplierSelect"
          class="select select-bordered select-sm w-full"
          bind:value={selectedSupplierId}
          disabled={isEditing}
        >
          <option value="">-- Elige uno --</option>
          {#each allSuppliers as s (s.id)}
            <option value={s.id}>{s.name}</option>
          {/each}
        </select>
      </div>

      {#if selectedSupplierId !== ""}
        <div class="form-control w-full mb-3">
          <label class="label pt-0 pb-1" for="supplierEditName">
            <span class="label-text text-xs font-medium opacity-70"
              >Nuevo Nombre</span
            >
          </label>
          <input
            id="supplierEditName"
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
            {#each SUPPLIER_COLORS as c}
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

        <div class="flex gap-2">
          <button
            class="btn btn-success btn-sm flex-1 text-success-content"
            onclick={handleEdit}
            disabled={isEditing || !nameToEdit.trim()}
          >
            {#if isEditing}
              <span class="loading loading-spinner loading-xs"></span>
            {/if}
            Guardar
          </button>
        </div>
      {/if}
    {/if}
  </div>

  <button
    class="btn btn-ghost btn-sm flex-1"
    onclick={onClose}
    disabled={isAdding}
  >
    Cancelar
  </button>
</div>
