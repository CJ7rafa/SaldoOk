<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { notifications } from "../../../lib/notifications.svelte";
  import type { Spender, ExpenseCategory, Expense } from "../../../types";
  import { imask } from "svelte-imask";

  let { dateStr, spender, category, existingExpense, onClose, onSuccess } = $props<{
    dateStr: string;
    spender: Spender;
    category: ExpenseCategory;
    existingExpense?: Expense;
    onClose: () => void;
    onSuccess?: () => void;
  }>();

  let totalAmount = $state(
    existingExpense ? String(existingExpense.amount) : "",
  );
  let notes = $state(existingExpense?.notes || "");
  let isLoading = $state(false);

  const currencyOptions = {
    mask: Number,
    scale: 2,
    signed: false,
    thousandsSeparator: ",",
    padFractionalZeros: true,
    normalizeZeros: true,
    radix: ".",
    mapToRadix: ["."],
  };

  function handleAmountAccept(e: CustomEvent) {
    totalAmount = e.detail.unmaskedValue;
  }

  async function handleAceptar() {
    const amount = parseFloat(totalAmount);
    if (isNaN(amount) || amount <= 0) {
      notifications.show(
        "Monto inválido",
        "Por favor ingresa un monto mayor a 0",
        "warning",
      );
      return;
    }

    isLoading = true;

    try {
      const expense: Expense = {
        id: existingExpense?.id,
        spender_id: spender.id,
        category_id: category.id,
        amount: amount,
        expense_date: dateStr,
        notes: notes.trim() || undefined,
      };

      if (existingExpense?.id) {
        await invoke("update_expense", { expense });
        notifications.show(
          "¡Actualizado!",
          "Gasto actualizado correctamente",
          "success",
        );
      } else {
        await invoke("create_expense", { expense });
        notifications.show(
          "¡Éxito!",
          "Gasto registrado correctamente",
          "success",
        );
      }

      if (onSuccess) onSuccess();
      onClose(); // Todo salió bien, regresamos al calendario
    } catch (err) {
      notifications.show("Error al guardar", String(err), "error");
    } finally {
      isLoading = false;
    }
  }
</script>

<svelte:window onkeydown={(e) => {
  if (e.key === 'Enter' && !isLoading) {
    if (document.activeElement?.tagName === 'TEXTAREA') return;
    e.preventDefault();
    handleAceptar();
  }
}} />

<div class="flex flex-col h-full">
  <h3 class="font-bold mb-6 text-sm uppercase opacity-70">
    {existingExpense ? "Editar Gasto" : "Agregar Gasto"}
  </h3>

  <div class="flex-1 overflow-y-auto pr-2 space-y-4">
    <!-- Campos de sólo lectura (Visuales) -->
    <div class="grid grid-cols-2 gap-4">
      <div class="form-control w-full">
        <label class="label" for="spenderReadonly">
          <span class="label-text font-medium">Persona</span>
        </label>
        <input
          id="spenderReadonly"
          type="text"
          class="input input-bordered w-full"
          value={spender.name}
          disabled
        />
      </div>

      <div class="form-control w-full">
        <label class="label" for="categoryReadonly">
          <span class="label-text font-medium">Categoría</span>
        </label>
        <input
          id="categoryReadonly"
          type="text"
          class="input input-bordered w-full"
          value={category.name}
          disabled
        />
      </div>
    </div>

    <div class="form-control w-full">
      <label class="label" for="dateReadonly">
        <span class="label-text font-medium">Fecha</span>
      </label>
      <input
        id="dateReadonly"
        type="date"
        class="input input-bordered w-full"
        value={dateStr}
        disabled
      />
    </div>

    <div class="divider my-0"></div>

    <!-- Campos Editables -->
    <div class="form-control w-full">
      <label class="label" for="totalAmount">
        <span class="label-text font-medium"
          >Monto Total ($) <span class="text-error">*</span></span
        >
      </label>
      <input
        id="totalAmount"
        type="text"
        placeholder="Ej. 15.50"
        class="input input-bordered w-full font-mono text-right"
        use:imask={currencyOptions}
        onaccept={handleAmountAccept}
        value={totalAmount}
        disabled={isLoading}
      />
    </div>

    <div class="form-control w-full">
      <label class="label" for="notes">
        <span class="label-text font-medium">Notas Adicionales</span>
      </label>
      <textarea
        id="notes"
        class="textarea textarea-bordered w-full resize-none h-20"
        placeholder="Anotaciones sobre el gasto..."
        bind:value={notes}
        disabled={isLoading}
      ></textarea>
    </div>
  </div>

  <div class="mt-4 flex gap-2 pt-2 border-t border-base-300">
    <button class="btn btn-ghost flex-1" onclick={onClose} disabled={isLoading}>
      Cancelar
    </button>
    <button
      class="btn btn-primary flex-1"
      onclick={handleAceptar}
      disabled={isLoading}
    >
      {#if isLoading}
        <span class="loading loading-spinner loading-sm"></span>
      {/if}
      Guardar
    </button>
  </div>
</div>
