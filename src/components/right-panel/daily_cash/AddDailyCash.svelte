<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { notifications } from "../../../lib/notifications.svelte";
  import type { DailyCashRegister } from "../../../types";
  import { imask } from "svelte-imask";

  let { dateStr, existingRegister, onClose, onSuccess } = $props<{
    dateStr: string;
    existingRegister?: DailyCashRegister;
    onClose: () => void;
    onSuccess?: () => void;
  }>();

  // 1. MERCADO
  let marketSales = $state("");
  let marketVaultSaved = $state("");

  // 2. CASA
  let houseSales = $state("");

  // 3. OTROS INGRESOS
  let houseShopSavings = $state("");
  let chainIncome = $state("");
  let vaultLoan = $state("");

  // 4. AUTO CONSUMO
  let selfConsumption = $state("");

  // NOTAS (Compartidas por día)
  let notes = $state("");

  $effect(() => {
    // Cuando el usuario hace clic en otra celda, actualizamos los valores del panel
    const r = existingRegister;
    const _date = dateStr; 
    
    marketSales = r && r.market_sales !== 0 ? String(r.market_sales) : "";
    marketVaultSaved = r && r.market_vault_saved !== 0 ? String(r.market_vault_saved) : "";
    houseSales = r && r.house_sales !== 0 ? String(r.house_sales) : "";
    houseShopSavings = r && r.house_shop_savings !== 0 ? String(r.house_shop_savings) : "";
    chainIncome = r && r.chain_income !== 0 ? String(r.chain_income) : "";
    vaultLoan = r && r.vault_loan !== 0 ? String(r.vault_loan) : "";
    selfConsumption = r && r.self_consumption !== 0 ? String(r.self_consumption) : "";
    notes = r?.notes || "";
  });
  
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

  async function handleAceptar() {
    isLoading = true;

    try {
      const register: DailyCashRegister = {
        id: existingRegister?.id,
        entry_date: dateStr,
        market_sales: parseFloat(marketSales) || 0,
        market_vault_saved: parseFloat(marketVaultSaved) || 0,
        house_sales: parseFloat(houseSales) || 0,
        house_shop_savings: parseFloat(houseShopSavings) || 0,
        chain_income: parseFloat(chainIncome) || 0,
        vault_loan: parseFloat(vaultLoan) || 0,
        self_consumption: parseFloat(selfConsumption) || 0,
        notes: notes.trim() || undefined,
      };

      if (existingRegister?.id) {
        await invoke("update_daily_cash_register", { register });
        notifications.show(
          "¡Actualizado!",
          "Flujo de caja actualizado correctamente",
          "success",
        );
      } else {
        await invoke("create_daily_cash_register", { register });
        notifications.show(
          "¡Éxito!",
          "Flujo de caja registrado correctamente",
          "success",
        );
      }

      if (onSuccess) onSuccess();
      onClose();
    } catch (err) {
      notifications.show("Error al guardar", String(err), "error");
    } finally {
      isLoading = false;
    }
  }

  function onMarketSalesAccept(e: any) { marketSales = e.detail.unmaskedValue; }
  function onMarketVaultSavedAccept(e: any) { marketVaultSaved = e.detail.unmaskedValue; }
  function onHouseSalesAccept(e: any) { houseSales = e.detail.unmaskedValue; }
  function onHouseShopSavingsAccept(e: any) { houseShopSavings = e.detail.unmaskedValue; }
  function onChainIncomeAccept(e: any) { chainIncome = e.detail.unmaskedValue; }
  function onVaultLoanAccept(e: any) { vaultLoan = e.detail.unmaskedValue; }
  function onSelfConsumptionAccept(e: any) { selfConsumption = e.detail.unmaskedValue; }
</script>

<svelte:window onkeydown={(e) => {
  if (e.key === 'Enter' && !isLoading) {
    if (document.activeElement?.tagName === 'TEXTAREA') return;
    e.preventDefault();
    handleAceptar();
  }
}} />

<div class="flex flex-col h-full">
  <h3 class="font-bold mb-4 text-base uppercase opacity-70">
    {existingRegister ? "Editar Flujo de Caja" : "Registrar Flujo de Caja"}
  </h3>

  <div class="flex-1 overflow-y-auto pr-2 space-y-6">
    <!-- INFO DÍA -->
    <div class="bg-base-200 p-3 rounded-lg border border-base-300">
      <p class="text-sm uppercase opacity-70 font-bold mb-1">Fecha de Registro</p>
      <p class="text-xl font-mono text-primary font-bold">{dateStr}</p>
    </div>

    <!-- MERCADO -->
    <div class="space-y-3">
      <h4 class="text-sm font-bold text-success uppercase border-b border-base-300 pb-1">Mercado</h4>
      <div class="grid grid-cols-2 gap-4">
        <div class="form-control w-full">
          <label class="label" for="marketSales">
            <span class="label-text font-medium text-sm">Flujo/Efectivo Diario</span>
          </label>
          <input
            id="marketSales"
            type="text"
            placeholder="0.00"
            class="input text-lg input-bordered w-full font-mono text-right"
            use:imask={currencyOptions}
            onaccept={onMarketSalesAccept}
            value={marketSales}
            disabled={isLoading}
          />
        </div>
        <div class="form-control w-full">
          <label class="label" for="marketVaultSaved">
            <span class="label-text font-medium text-sm">Ahorro</span>
          </label>
          <input
            id="marketVaultSaved"
            type="text"
            placeholder="0.00"
            class="input text-lg input-bordered w-full font-mono text-right"
            use:imask={currencyOptions}
            onaccept={onMarketVaultSavedAccept}
            value={marketVaultSaved}
            disabled={isLoading}
          />
        </div>
      </div>
    </div>

    <!-- CASA -->
    <div class="space-y-3">
      <h4 class="text-sm font-bold text-warning uppercase border-b border-base-300 pb-1">Casa</h4>
      <div class="form-control w-full">
        <label class="label" for="houseSales">
          <span class="label-text font-medium text-sm">F/E Ventas del día</span>
        </label>
        <input
          id="houseSales"
          type="text"
          placeholder="0.00"
          class="input text-lg input-bordered w-full font-mono text-right"
          use:imask={currencyOptions}
          onaccept={onHouseSalesAccept}
          value={houseSales}
          disabled={isLoading}
        />
      </div>
    </div>

    <!-- OTROS INGRESOS -->
    <div class="space-y-3">
      <h4 class="text-sm font-bold text-info uppercase border-b border-base-300 pb-1">Otros Ingresos</h4>
      
      <div class="form-control w-full">
        <label class="label" for="houseShopSavings">
          <span class="label-text font-medium text-sm">F/E Ahorros (Tienda Casa)</span>
        </label>
        <input
          id="houseShopSavings"
          type="text"
          placeholder="0.00"
          class="input text-lg input-bordered w-full font-mono text-right"
          use:imask={currencyOptions}
          onaccept={onHouseShopSavingsAccept}
          value={houseShopSavings}
          disabled={isLoading}
        />
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div class="form-control w-full">
          <label class="label" for="chainIncome">
            <span class="label-text font-medium text-sm">Ahorro (Mercado)</span>
          </label>
          <input
            id="chainIncome"
            type="text"
            placeholder="0.00"
            class="input text-lg input-bordered w-full font-mono text-right"
            use:imask={currencyOptions}
            onaccept={onChainIncomeAccept}
            value={chainIncome}
            disabled={isLoading}
          />
        </div>
        <div class="form-control w-full">
          <label class="label" for="vaultLoan">
            <span class="label-text font-medium text-sm">Préstamo</span>
          </label>
          <input
            id="vaultLoan"
            type="text"
            placeholder="0.00"
            class="input text-lg input-bordered w-full font-mono text-right"
            use:imask={currencyOptions}
            onaccept={onVaultLoanAccept}
            value={vaultLoan}
            disabled={isLoading}
          />
        </div>
      </div>
    </div>

    <!-- AUTO CONSUMO -->
    <div class="space-y-3">
      <h4 class="text-sm font-bold uppercase border-b border-base-300 pb-1">Auto Consumo</h4>
      <div class="form-control w-full">
        <label class="label" for="selfConsumption">
          <span class="label-text font-medium text-sm">Consumo Rafa - Casa</span>
        </label>
        <input
          id="selfConsumption"
          type="text"
          placeholder="0.00"
          class="input text-lg input-bordered w-full font-mono text-right"
          use:imask={currencyOptions}
          onaccept={onSelfConsumptionAccept}
          value={selfConsumption}
          disabled={isLoading}
        />
      </div>
    </div>

    <!-- NOTAS GLOBALES -->
    <div class="form-control w-full">
      <label class="label" for="notes">
        <span class="label-text font-medium text-sm">Notas Adicionales (Del día)</span>
      </label>
      <textarea
        id="notes"
        class="textarea textarea-bordered w-full resize-none h-20 text-sm"
        placeholder="Anotaciones generales de este día..."
        bind:value={notes}
        disabled={isLoading}
      ></textarea>
    </div>
  </div>

  <div class="mt-4 flex gap-2 pt-4 border-t border-base-300">
    <button class="btn btn-ghost flex-1 text-sm" onclick={onClose} disabled={isLoading}>
      Cancelar
    </button>
    <button
      class="btn btn-primary flex-1 text-sm"
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
