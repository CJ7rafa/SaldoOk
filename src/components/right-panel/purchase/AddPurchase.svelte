<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { notifications } from "../../../lib/notifications.svelte";
  import type { Supplier, Purchase } from "../../../types";
  import { imask } from "svelte-imask";

  let { dateStr, supplier, existingPurchase, onClose, onSuccess } = $props<{
    dateStr: string;
    supplier: Supplier;
    existingPurchase?: Purchase;
    onClose: () => void;
    onSuccess?: () => void;
  }>();

  let invoiceNumber = $state(existingPurchase?.invoice_number || "");
  let totalAmount = $state(
    existingPurchase ? String(existingPurchase.total_amount) : "",
  );
  let paymentType = $state(existingPurchase?.payment_type || "CASH");
  let status = $state(existingPurchase?.status || "PAID");
  let notes = $state(existingPurchase?.notes || "");
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
      const purchase: Purchase = {
        id: existingPurchase?.id,
        supplier_id: supplier.id,
        invoice_number: invoiceNumber.trim() || undefined,
        issue_date: dateStr,
        total_amount: amount,
        payment_type: paymentType,
        status: status,
        notes: notes.trim() || undefined,
      };

      if (existingPurchase?.id) {
        await invoke("update_purchase", { purchase });
        notifications.show(
          "¡Actualizado!",
          "Compra actualizada correctamente",
          "success",
        );
      } else {
        await invoke("create_purchase", { purchase });
        notifications.show(
          "¡Éxito!",
          "Compra registrada correctamente",
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
    {existingPurchase ? "Editar Compra" : "Agregar Compra"}
  </h3>

  <div class="flex-1 overflow-y-auto pr-2 space-y-4">
    <!-- Campos de sólo lectura (Visuales) -->
    <div class="grid grid-cols-2 gap-4">
      <div class="form-control w-full">
        <label class="label" for="supplierReadonly">
          <span class="label-text font-medium">Proveedor</span>
        </label>
        <input
          id="supplierReadonly"
          type="text"
          class="input input-bordered w-full"
          value={supplier.name}
          disabled
        />
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
        placeholder="Ej. 150.50"
        class="input input-bordered w-full font-mono text-right"
        use:imask={currencyOptions}
        onaccept={handleAmountAccept}
        value={totalAmount}
        disabled={isLoading}
      />
    </div>

    <div class="form-control w-full">
      <label class="label" for="invoiceNumber">
        <span class="label-text font-medium">Nº Factura / Recibo</span>
      </label>
      <input
        id="invoiceNumber"
        type="text"
        placeholder="Ej. 001-001-000000123"
        class="input input-bordered w-full"
        bind:value={invoiceNumber}
        disabled={isLoading}
      />
    </div>

    <div class="grid grid-cols-2 gap-4">
      <div class="form-control w-full">
        <label class="label" for="paymentType">
          <span class="label-text font-medium">Tipo de Pago</span>
        </label>
        <select
          id="paymentType"
          class="select select-bordered w-full"
          bind:value={paymentType}
          disabled={isLoading}
        >
          <option value="CASH">Contado</option>
          <option value="CREDIT">Crédito</option>
        </select>
      </div>

      <div class="form-control w-full">
        <label class="label" for="status">
          <span class="label-text font-medium">Estado</span>
        </label>
        <select
          id="status"
          class="select select-bordered w-full"
          bind:value={status}
          disabled={isLoading}
        >
          <option value="PAID">Pagado</option>
          <option value="PENDING">Pendiente</option>
        </select>
      </div>
    </div>

    <div class="form-control w-full">
      <label class="label" for="notes">
        <span class="label-text font-medium">Notas Adicionales</span>
      </label>
      <textarea
        id="notes"
        class="textarea textarea-bordered w-full resize-none h-20"
        placeholder="Anotaciones sobre la compra..."
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
