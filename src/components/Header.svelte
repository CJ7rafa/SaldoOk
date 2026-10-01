<script lang="ts">
  import { ArrowLeft, UserPlus } from "@lucide/svelte";
  import { appState } from "../lib/appState.svelte";

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

  function formatCurrency(amount: number): string {
    return new Intl.NumberFormat("en-US", {
      style: "currency",
      currency: "USD",
    }).format(amount);
  }

  function getPreviousDayMarketSales(dateStr: string): number {
    if (!appState.allDailyCash) return 0;
    const [y, m, d] = dateStr.split("-").map(Number);
    const prevDate = new Date(y, m - 1, d - 1);
    const prevDateStr = `${prevDate.getFullYear()}-${String(prevDate.getMonth() + 1).padStart(2, "0")}-${String(prevDate.getDate()).padStart(2, "0")}`;
    const prevReg = appState.allDailyCash.find((r) => r.entry_date === prevDateStr);
    return prevReg ? prevReg.market_sales : 0;
  }

  let daysInMonth = $derived(
    new Date(appState.selectedDate.getFullYear(), appState.selectedDate.getMonth() + 1, 0).getDate()
  );

  let totalComprasMensual = $derived.by(() => {
    if (!appState.allPurchases) return 0;
    const y = appState.selectedDate.getFullYear();
    const m = String(appState.selectedDate.getMonth() + 1).padStart(2, "0");
    const prefix = `${y}-${m}-`;
    return appState.allPurchases
        .filter((p) => p.issue_date.startsWith(prefix))
        .reduce((sum, p) => sum + p.total_amount, 0);
  });

  let totalVentasMercado = $derived.by(() => {
      if (!appState.allDailyCash || appState.allDailyCash.length === 0) return 0;
      let total = 0;
      const y = appState.selectedDate.getFullYear();
      const m = String(appState.selectedDate.getMonth() + 1).padStart(2, "0");
      const days = new Date(y, appState.selectedDate.getMonth() + 1, 0).getDate();

      for (let i = 1; i <= days; i++) {
          const d = String(i).padStart(2, "0");
          const dateStr = `${y}-${m}-${d}`;
          const reg = appState.allDailyCash.find((r) => r.entry_date === dateStr);
          const prevMarketSales = getPreviousDayMarketSales(dateStr);
          
          const dailyPurchases = appState.allPurchases
              ? appState.allPurchases.filter((p) => p.issue_date === dateStr).reduce((sum, p) => sum + p.total_amount, 0)
              : 0;
          const dailyExpenses = appState.allExpenses
              ? appState.allExpenses.filter((e) => e.expense_date === dateStr).reduce((sum, e) => sum + e.amount, 0)
              : 0;
              
          const feAhorrosTiendaCasa = reg?.house_shop_savings || 0;
          const hasMarketSales = reg && reg.market_sales > 0;

          if (hasMarketSales) {
              const ventasMercado =
                  dailyPurchases +
                  dailyExpenses +
                  (reg?.market_sales || 0) +
                  (reg?.market_vault_saved || 0) -
                  prevMarketSales -
                  (reg?.house_sales || 0) -
                  feAhorrosTiendaCasa -
                  (reg?.chain_income || 0) -
                  (reg?.vault_loan || 0);

              total += ventasMercado;
          }
      }
      return total;
  });

  let promedioCompras = $derived(totalComprasMensual / daysInMonth);
  let promedioVentas = $derived(totalVentasMercado / daysInMonth);
  let gananciaDiariaPromedio = $derived(promedioVentas * 0.13);

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

  <div class="flex items-center gap-4">
    {#if currentView === "tablacompras"}
      <div class="bg-primary/10 border border-primary/20 rounded-md px-3 h-10 flex flex-col items-center justify-center shadow-sm">
        <span class="text-[10px] uppercase font-bold opacity-70 leading-none">Promedio compras ({daysInMonth} días)</span>
        <span class="font-mono font-bold text-sm text-primary mt-1 leading-none">{formatCurrency(promedioCompras)}</span>
      </div>

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
    {:else if currentView === "tablaingresos"}
      <div class="bg-success/10 border border-success/20 rounded-md px-3 h-10 flex flex-col items-center justify-center shadow-sm">
        <span class="text-[10px] uppercase font-bold opacity-70 leading-none">Prom. ventas diarias ({daysInMonth} días)</span>
        <span class="font-mono font-bold text-sm text-success mt-1 leading-none">{formatCurrency(promedioVentas)}</span>
      </div>
      <div class="bg-info/10 border border-info/20 rounded-md px-3 h-10 flex flex-col items-center justify-center shadow-sm">
        <span class="text-[10px] uppercase font-bold opacity-70 leading-none">Ganancia diaria prom. (13%)</span>
        <span class="font-mono font-bold text-sm text-info mt-1 leading-none">{formatCurrency(gananciaDiariaPromedio)}</span>
      </div>
    {/if}
  </div>
</header>
