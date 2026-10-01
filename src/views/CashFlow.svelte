<script lang="ts">
    import type {
        DailyCashRegister,
        Purchase,
        Expense,
        Supplier,
        Spender,
        ExpenseCategory,
    } from "../types";

    interface Props {
        selectedDate: Date;
        allDailyCash: DailyCashRegister[];
        allPurchases: Purchase[];
        allExpenses: Expense[];
        allSuppliers: Supplier[];
        allSpenders: Spender[];
        allCategoriesMaster: ExpenseCategory[];
    }

    let {
        selectedDate,
        allDailyCash,
        allPurchases,
        allExpenses,
        allSuppliers,
        allSpenders,
        allCategoriesMaster,
    }: Props = $props();

    // Funciones utilitarias
    function formatCurrency(amount: number): string {
        return new Intl.NumberFormat("en-US", {
            style: "currency",
            currency: "USD",
        }).format(amount);
    }

    function getPreviousDayMarketSales(dateStr: string): number {
        if (!allDailyCash) return 0;
        const [y, m, d] = dateStr.split("-").map(Number);
        const prevDate = new Date(y, m - 1, d - 1);
        const prevDateStr = `${prevDate.getFullYear()}-${String(prevDate.getMonth() + 1).padStart(2, "0")}-${String(prevDate.getDate()).padStart(2, "0")}`;
        const prevReg = allDailyCash.find((r) => r.entry_date === prevDateStr);
        return prevReg ? prevReg.market_sales : 0;
    }

    let currentMonthName = $derived(() => {
        const monthName = selectedDate.toLocaleDateString("es-ES", {
            month: "long",
        });
        return monthName.charAt(0).toUpperCase() + monthName.slice(1);
    });
    
    let currentYear = $derived(selectedDate.getFullYear());

    let prefix = $derived.by(() => {
        const y = selectedDate.getFullYear();
        const m = String(selectedDate.getMonth() + 1).padStart(2, "0");
        return `${y}-${m}-`;
    });


    // ============================================
    // CÁLCULOS: PASO A PASO
    // ============================================

    // PASO 1: INGRESOS
    let totalVentasMercado = $derived.by(() => {
        if (!allDailyCash) return 0;
        let total = 0;
        const y = selectedDate.getFullYear();
        const m = String(selectedDate.getMonth() + 1).padStart(2, "0");
        const daysInMonth = new Date(y, selectedDate.getMonth() + 1, 0).getDate();

        for (let i = 1; i <= daysInMonth; i++) {
            const d = String(i).padStart(2, "0");
            const dateStr = `${y}-${m}-${d}`;
            const reg = allDailyCash.find((r) => r.entry_date === dateStr);
            const prevMarketSales = getPreviousDayMarketSales(dateStr);
            
            const dailyPurchases = allPurchases
                ? allPurchases.filter((p) => p.issue_date === dateStr).reduce((sum, p) => sum + p.total_amount, 0)
                : 0;
            const dailyExpenses = allExpenses
                ? allExpenses.filter((e) => e.expense_date === dateStr).reduce((sum, e) => sum + e.amount, 0)
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

    let totalVentasCasa = $derived(
        allDailyCash
            ? allDailyCash
                  .filter((r) => r.entry_date.startsWith(prefix))
                  .reduce((sum, r) => sum + (r.house_sales || 0), 0)
            : 0,
    );

    let totalIngresos = $derived(totalVentasMercado + totalVentasCasa);

    // PASO 2: EGRESOS
    let totalProveedores = $derived(
        allPurchases
            ? allPurchases
                  .filter((p) => p.issue_date.startsWith(prefix))
                  .reduce((sum, p) => sum + p.total_amount, 0)
            : 0,
    );
    
    let saldo1 = $derived(totalIngresos - totalProveedores);

    // PASO 3: OTROS INGRESOS
    let prestamosMercado = $derived(
        allDailyCash ? allDailyCash.filter(r => r.entry_date.startsWith(prefix)).reduce((sum, r) => sum + (r.vault_loan || 0), 0) : 0
    );
    let ahorroTiendaCasaConsumido = $derived(
        allDailyCash ? allDailyCash.filter(r => r.entry_date.startsWith(prefix)).reduce((sum, r) => sum + (r.house_shop_savings || 0), 0) : 0
    );
    let ahorroMercadoConsumido = $derived(
        allDailyCash ? allDailyCash.filter(r => r.entry_date.startsWith(prefix)).reduce((sum, r) => sum + (r.market_vault_saved || 0), 0) : 0
    );
    
    let totalOtrosIngresos = $derived(prestamosMercado + ahorroTiendaCasaConsumido + ahorroMercadoConsumido);
    
    let saldo2 = $derived(saldo1 + totalOtrosIngresos);

    // PASO 4: OTROS EGRESOS
    let pagoPrestamoMercado = $derived(prestamosMercado); // Igual que préstamos del mercado
    
    let totalOtrosEgresos = $derived(pagoPrestamoMercado);
    
    let saldo3 = $derived(saldo2 - totalOtrosEgresos);

    // PASO 5: EGRESOS PERSONALES
    let egresosPersonalesPorPersona = $derived.by(() => {
        if (!allExpenses || !allSpenders) return [];
        const monthlyExpenses = allExpenses.filter((e) =>
            e.expense_date.startsWith(prefix),
        );
        const spendersMap = new Map<number, number>();

        for (const e of monthlyExpenses) {
            spendersMap.set(e.spender_id, (spendersMap.get(e.spender_id) || 0) + e.amount);
        }

        return Array.from(spendersMap.entries())
            .map(([spenderId, total]) => {
                const spender = allSpenders.find((s) => s.id === spenderId);
                return {
                    name: spender ? spender.name : `Persona ${spenderId}`,
                    total,
                };
            })
            .sort((a, b) => b.total - a.total);
    });

    let totalEgresosPersonales = $derived(egresosPersonalesPorPersona.reduce((sum, p) => sum + p.total, 0));

    let saldo4 = $derived(saldo3 - totalEgresosPersonales);


    // ============================================
    // RESUMEN FINAL
    // ============================================
    let comprobacionGastosCompras = $derived(totalProveedores + totalOtrosEgresos + totalEgresosPersonales);
    let saldoPrestamoPorDevolver = $derived(totalOtrosIngresos - totalOtrosEgresos);

</script>

<div class="w-full flex-1 flex flex-col min-h-0">
    <div class="bg-base-100 shadow-md border-b border-base-300 w-full flex-1 min-h-0 overflow-y-auto flex flex-col items-center">
        <!-- HEADER -->
        <div class="w-full bg-base-200/50 sticky top-0 z-10 p-6 flex flex-col items-center border-b border-base-300 backdrop-blur-md">
            <h2 class="text-2xl font-black text-primary uppercase tracking-widest mb-1">Flujo de Efectivo</h2>
            <div class="flex gap-4 text-primary font-bold text-lg">
                <span>{currentMonthName()}</span>
                <span>{currentYear}</span>
            </div>
        </div>

        <div class="w-full max-w-4xl p-8 flex flex-col gap-10">
            
            <!-- CONTENEDOR ESTILO "ESTADO DE CUENTA" -->
            <div class="bg-base-100 rounded-xl border border-base-300 shadow-sm overflow-hidden flex flex-col">
                
                <!-- BLOQUE 1: INGRESOS -->
                <div class="p-6">
                    <div class="flex justify-between items-end border-b-2 border-success pb-2 mb-4">
                        <h3 class="text-xl font-bold uppercase text-success flex items-center gap-2">
                            <span class="bg-success/20 w-8 h-8 rounded-full flex items-center justify-center">+</span> 
                            Ingresos
                        </h3>
                        <span class="text-xl font-mono font-bold text-success">{formatCurrency(totalIngresos)}</span>
                    </div>
                    <div class="pl-10 space-y-2">
                        <div class="flex justify-between items-center py-1 border-b border-base-200 border-dashed">
                            <span class="text-base text-base-content/80">Ventas del mercado</span>
                            <span class="font-mono">{formatCurrency(totalVentasMercado)}</span>
                        </div>
                        <div class="flex justify-between items-center py-1 border-b border-base-200 border-dashed">
                            <span class="text-base text-base-content/80">Ventas de casa</span>
                            <span class="font-mono">{formatCurrency(totalVentasCasa)}</span>
                        </div>
                    </div>
                </div>

                <!-- BLOQUE 2: EGRESOS Y SALDO 1 -->
                <div class="p-6 bg-base-200/30 border-t border-base-300">
                    <div class="flex justify-between items-end border-b-2 border-error pb-2 mb-4">
                        <h3 class="text-xl font-bold uppercase text-error flex items-center gap-2">
                            <span class="bg-error/20 w-8 h-8 rounded-full flex items-center justify-center">-</span> 
                            Egresos
                        </h3>
                        <span class="text-xl font-mono font-bold text-error">{formatCurrency(totalProveedores)}</span>
                    </div>
                    <div class="pl-10 space-y-2 mb-6">
                        <div class="flex justify-between items-center py-1 border-b border-base-200 border-dashed">
                            <span class="text-base text-base-content/80">Proveedores</span>
                            <span class="font-mono">{formatCurrency(totalProveedores)}</span>
                        </div>
                    </div>
                    
                    <!-- SALDO 1 -->
                    <div class="bg-base-100 p-4 rounded-lg border border-base-300 flex justify-between items-center shadow-inner mt-4">
                        <span class="font-bold uppercase tracking-widest opacity-70">Saldo</span>
                        <span class="text-2xl font-mono font-black" class:text-success={saldo1 >= 0} class:text-error={saldo1 < 0}>{formatCurrency(saldo1)}</span>
                    </div>
                </div>

                <!-- BLOQUE 3: OTROS INGRESOS Y SALDO 2 -->
                <div class="p-6 border-t border-base-300">
                    <div class="flex justify-between items-end border-b-2 border-success pb-2 mb-4">
                        <h3 class="text-xl font-bold uppercase text-success flex items-center gap-2">
                            <span class="bg-success/20 w-8 h-8 rounded-full flex items-center justify-center">+</span> 
                            Otros Ingresos
                        </h3>
                        <span class="text-xl font-mono font-bold text-success">{formatCurrency(totalOtrosIngresos)}</span>
                    </div>
                    <div class="pl-10 space-y-2 mb-6">
                        <div class="flex justify-between items-center py-1 border-b border-base-200 border-dashed">
                            <span class="text-base text-base-content/80">Préstamos del mercado</span>
                            <span class="font-mono">{formatCurrency(prestamosMercado)}</span>
                        </div>
                        <div class="flex justify-between items-center py-1 border-b border-base-200 border-dashed">
                            <span class="text-base text-base-content/80">Ahorro de tienda casa (consumido)</span>
                            <span class="font-mono">{formatCurrency(ahorroTiendaCasaConsumido)}</span>
                        </div>
                        <div class="flex justify-between items-center py-1 border-b border-base-200 border-dashed">
                            <span class="text-base text-base-content/80">Ahorro del mercado (consumido)</span>
                            <span class="font-mono">{formatCurrency(ahorroMercadoConsumido)}</span>
                        </div>
                    </div>

                    <!-- SALDO 2 -->
                    <div class="bg-base-100 p-4 rounded-lg border border-base-300 flex justify-between items-center shadow-inner mt-4">
                        <span class="font-bold uppercase tracking-widest opacity-70">Saldo</span>
                        <span class="text-2xl font-mono font-black" class:text-success={saldo2 >= 0} class:text-error={saldo2 < 0}>{formatCurrency(saldo2)}</span>
                    </div>
                </div>

                <!-- BLOQUE 4: OTROS EGRESOS Y SALDO 3 -->
                <div class="p-6 bg-base-200/30 border-t border-base-300">
                    <div class="flex justify-between items-end border-b-2 border-error pb-2 mb-4">
                        <h3 class="text-xl font-bold uppercase text-error flex items-center gap-2">
                            <span class="bg-error/20 w-8 h-8 rounded-full flex items-center justify-center">-</span> 
                            Otros Egresos
                        </h3>
                        <span class="text-xl font-mono font-bold text-error">{formatCurrency(totalOtrosEgresos)}</span>
                    </div>
                    <div class="pl-10 space-y-2 mb-6">
                        <div class="flex justify-between items-center py-1 border-b border-base-200 border-dashed">
                            <span class="text-base text-base-content/80">Pago a préstamo del mercado</span>
                            <span class="font-mono">{formatCurrency(pagoPrestamoMercado)}</span>
                        </div>
                    </div>

                    <!-- SALDO 3 -->
                    <div class="bg-base-100 p-4 rounded-lg border border-base-300 flex justify-between items-center shadow-inner mt-4">
                        <span class="font-bold uppercase tracking-widest opacity-70">Saldo</span>
                        <span class="text-2xl font-mono font-black" class:text-success={saldo3 >= 0} class:text-error={saldo3 < 0}>{formatCurrency(saldo3)}</span>
                    </div>
                </div>

                <!-- BLOQUE 5: EGRESOS PERSONALES Y SALDO FINAL (4) -->
                <div class="p-6 border-t border-base-300">
                    <div class="flex justify-between items-end border-b-2 border-error pb-2 mb-4">
                        <h3 class="text-xl font-bold uppercase text-error flex items-center gap-2">
                            <span class="bg-error/20 w-8 h-8 rounded-full flex items-center justify-center">-</span> 
                            Egresos Personales
                        </h3>
                        <span class="text-xl font-mono font-bold text-error">{formatCurrency(totalEgresosPersonales)}</span>
                    </div>
                    <div class="pl-10 space-y-2 mb-6">
                        {#each egresosPersonalesPorPersona as persona}
                            <div class="flex justify-between items-center py-1 border-b border-base-200 border-dashed hover:bg-base-200/50 transition-colors">
                                <span class="text-base text-base-content/80">{persona.name}</span>
                                <span class="font-mono">{formatCurrency(persona.total)}</span>
                            </div>
                        {/each}
                        {#if egresosPersonalesPorPersona.length === 0}
                            <div class="py-4 text-center opacity-50 italic">No hay egresos personales este mes.</div>
                        {/if}
                    </div>

                    <!-- SALDO FINAL -->
                    <div class="bg-primary p-6 rounded-xl shadow-lg mt-8 flex flex-col items-center justify-center relative overflow-hidden">
                        <!-- Efecto visual decorativo de la tarjeta -->
                        <div class="absolute top-0 right-0 -mr-16 -mt-16 w-48 h-48 rounded-full bg-primary-content/10 blur-3xl"></div>
                        <div class="absolute bottom-0 left-0 -ml-16 -mb-16 w-32 h-32 rounded-full bg-primary-content/10 blur-2xl"></div>
                        
                        <span class="text-primary-content/80 font-bold uppercase tracking-[0.2em] mb-2 z-10">Saldo Final del Mes</span>
                        <span class="text-5xl font-mono font-black text-primary-content z-10 tracking-tight">
                            {formatCurrency(saldo4)}
                        </span>
                    </div>
                </div>

            </div>

            <!-- RESUMEN FINAL (FUERA DEL DESGLOSE) -->
            <div class="grid grid-cols-2 gap-6 mt-4 mb-16">
                <!-- Comprobando total de Gastos y Compras -->
                <div class="bg-base-200 border border-base-300 rounded-xl p-6 flex flex-col items-center shadow-inner hover:shadow-md transition-shadow">
                    <span class="text-sm font-bold uppercase tracking-wider opacity-70 text-center mb-2">
                        Comprobando Total<br>Gastos y Compras
                    </span>
                    <span class="text-3xl font-mono font-black">
                        {formatCurrency(comprobacionGastosCompras)}
                    </span>
                    <span class="text-xs opacity-50 mt-3 text-center">Proveedores + Pago a préstamo + Egresos Personales</span>
                </div>

                <!-- Saldo del préstamo por devolver -->
                <div class="bg-base-200 border border-base-300 rounded-xl p-6 flex flex-col items-center shadow-inner hover:shadow-md transition-shadow">
                    <span class="text-sm font-bold uppercase tracking-wider opacity-70 text-center mb-2">
                        Saldo del préstamo<br>por devolver
                    </span>
                    <span class="text-3xl font-mono font-black" class:text-success={saldoPrestamoPorDevolver <= 0} class:text-warning={saldoPrestamoPorDevolver > 0}>
                        {formatCurrency(saldoPrestamoPorDevolver)}
                    </span>
                    <span class="text-xs opacity-50 mt-3 text-center">Otros ingresos - Otros egresos</span>
                </div>
            </div>

        </div>
    </div>
</div>
