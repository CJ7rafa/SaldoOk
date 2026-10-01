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

    // 1. Cálculos de la sección superior
    let totalComprasMensual = $derived(
        allPurchases
            ? allPurchases
                  .filter((p) => p.issue_date.startsWith(prefix))
                  .reduce((sum, p) => sum + p.total_amount, 0)
            : 0,
    );

    let totalGastosMensual = $derived(
        allExpenses
            ? allExpenses
                  .filter((e) => e.expense_date.startsWith(prefix))
                  .reduce((sum, e) => sum + e.amount, 0)
            : 0,
    );

    let totalComprasYGastos = $derived(
        totalComprasMensual + totalGastosMensual,
    );

    let totalVentasCasa = $derived(
        allDailyCash
            ? allDailyCash
                  .filter((r) => r.entry_date.startsWith(prefix))
                  .reduce((sum, r) => sum + (r.house_sales || 0), 0)
            : 0,
    );

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

    let totalVentas = $derived(totalVentasMercado + totalVentasCasa);
    let diferencia = $derived(totalVentas - totalComprasYGastos);

    // 2. Cálculos de compras por proveedor
    let comprasPorProveedor = $derived.by(() => {
        if (!allPurchases || !allSuppliers) return [];
        const monthlyPurchases = allPurchases.filter((p) =>
            p.issue_date.startsWith(prefix),
        );
        const grouped = new Map<number, number>();
        for (const p of monthlyPurchases) {
            grouped.set(
                p.supplier_id,
                (grouped.get(p.supplier_id) || 0) + p.total_amount,
            );
        }
        return Array.from(grouped.entries())
            .map(([supplierId, total]) => {
                const supplier = allSuppliers.find((s) => s.id === supplierId);
                return {
                    name: supplier ? supplier.name : `Proveedor ${supplierId}`,
                    total,
                };
            })
            .filter((p) => p.total > 0)
            .sort((a, b) => b.total - a.total);
    });

    // 3. Cálculos de gastos por persona y categoría
    let gastosPorPersona = $derived.by(() => {
        if (!allExpenses || !allSpenders || !allCategoriesMaster) return [];
        const monthlyExpenses = allExpenses.filter((e) =>
            e.expense_date.startsWith(prefix),
        );
        const spendersMap = new Map<
            number,
            { total: number; categories: Map<number, number> }
        >();

        for (const e of monthlyExpenses) {
            if (!spendersMap.has(e.spender_id)) {
                spendersMap.set(e.spender_id, {
                    total: 0,
                    categories: new Map(),
                });
            }
            const s = spendersMap.get(e.spender_id)!;
            s.total += e.amount;
            s.categories.set(
                e.category_id,
                (s.categories.get(e.category_id) || 0) + e.amount,
            );
        }

        return Array.from(spendersMap.entries())
            .map(([spenderId, data]) => {
                const spender = allSpenders.find((s) => s.id === spenderId);
                const categories = Array.from(data.categories.entries())
                    .map(([categoryId, total]) => {
                        const cat = allCategoriesMaster.find(
                            (c) => c.id === categoryId,
                        );
                        return {
                            name: cat ? cat.name : `Categoría ${categoryId}`,
                            total,
                        };
                    })
                    .sort((a, b) => b.total - a.total);

                return {
                    name: spender ? spender.name : `Persona ${spenderId}`,
                    total: data.total,
                    categories,
                };
            })
            .sort((a, b) => b.total - a.total);
    });
</script>

<div class="w-full flex-1 flex flex-col min-h-0 gap-4">
    <!-- SECCIÓN SUPERIOR: ESTADO DE RESULTADOS -->
    <div class="bg-base-100 shadow-md border-b border-base-300 p-4 h-[40%] min-h-[250px] flex flex-col items-center flex-none">
        <h2 class="text-xl font-black text-error uppercase tracking-widest mb-1">Estado de Resultados</h2>
        
        <div class="flex gap-4 mb-4 text-error font-bold text-base">
            <span>{currentMonthName()}</span>
            <span>{currentYear}</span>
        </div>

        <div class="w-full max-w-4xl flex items-center justify-between gap-8 flex-1 min-h-0">
            <!-- Izquierda: Ventas y Compras -->
            <div class="flex-1 flex flex-col justify-center gap-6">
                <!-- Ventas -->
                <div class="flex flex-col gap-1 w-full">
                    <div class="flex justify-between border-b-2 border-base-content pb-1">
                        <span class="text-base font-bold">Ventas</span>
                        <span class="text-base font-mono font-bold">{formatCurrency(totalVentas)}</span>
                    </div>
                    <div class="flex justify-between pl-8 border-b border-base-300 border-dotted py-1">
                        <span class="text-sm">Mercado</span>
                        <span class="text-sm font-mono">{formatCurrency(totalVentasMercado)}</span>
                    </div>
                    <div class="flex justify-between pl-8 border-b border-base-300 border-dotted py-1">
                        <span class="text-sm">Casa</span>
                        <span class="text-sm font-mono">{formatCurrency(totalVentasCasa)}</span>
                    </div>
                </div>

                <!-- Compras y Gastos -->
                <div class="flex flex-col gap-1 w-full">
                    <div class="flex justify-between border-b-2 border-base-content pb-1">
                        <span class="text-base font-bold">Compras y gastos personales</span>
                        <span class="text-base font-mono font-bold">{formatCurrency(totalComprasYGastos)}</span>
                    </div>
                </div>
            </div>

            <!-- Derecha: Diferencia -->
            <div class="flex-none bg-base-200 px-10 py-8 rounded-lg border border-base-300 flex flex-col items-center shadow-inner gap-2 justify-center">
                <span class="text-lg font-bold uppercase tracking-wider text-base-content/70">Diferencia</span>
                <span class="text-3xl font-mono font-black" class:text-success={diferencia >= 0} class:text-error={diferencia < 0}>
                    {formatCurrency(diferencia)}
                </span>
            </div>
        </div>
    </div>

    <!-- SECCIÓN INFERIOR: TABLAS DE COMPRAS Y GASTOS -->
    <div class="flex-1 grid grid-cols-2 gap-4 min-h-0">
        <!-- Izquierda: Compras -->
        <div class="bg-base-100 shadow-md overflow-hidden flex flex-col">
            <div class="p-4 bg-base-200 border-b border-base-300 flex justify-between items-center sticky top-0 z-10">
                <h3 class="font-bold text-lg uppercase opacity-80">Proveedores</h3>
                <span class="font-mono font-bold text-lg">{formatCurrency(totalComprasMensual)}</span>
            </div>
            
            <div class="flex-1 overflow-auto p-4">
                <table class="table table-sm w-full">
                    <tbody>
                        <!-- Espacio extra arriba solicitado por el usuario -->
                        <tr><td colspan="2" class="h-2 border-none"></td></tr>
                        
                        {#each comprasPorProveedor as p}
                            <tr class="hover">
                                <td class="text-base">{p.name}</td>
                                <td class="text-right font-mono text-base">{formatCurrency(p.total)}</td>
                            </tr>
                        {/each}
                        {#if comprasPorProveedor.length === 0}
                            <tr>
                                <td colspan="2" class="text-center opacity-50 py-8">No hay compras registradas en este mes.</td>
                            </tr>
                        {/if}
                    </tbody>
                </table>
            </div>
        </div>

        <!-- Derecha: Gastos -->
        <div class="bg-base-100 shadow-md overflow-hidden flex flex-col">
            <div class="p-4 bg-base-200 border-b border-base-300 flex justify-between items-center sticky top-0 z-10">
                <h3 class="font-bold text-lg uppercase opacity-80">Personas</h3>
                <span class="font-mono font-bold text-lg">{formatCurrency(totalGastosMensual)}</span>
            </div>
            
            <div class="flex-1 overflow-auto p-4">
                <table class="table table-sm w-full">
                    <tbody>
                        <!-- Espacio extra arriba -->
                        <tr><td colspan="2" class="h-2 border-none"></td></tr>
                        
                        {#each gastosPorPersona as persona}
                            <!-- Header de Persona -->
                            <tr class="bg-base-200/50">
                                <td class="text-base font-bold text-primary">{persona.name}</td>
                                <td class="text-right font-mono font-bold">{formatCurrency(persona.total)}</td>
                            </tr>
                            
                            <!-- Subcategorías -->
                            {#each persona.categories as cat}
                                <tr class="hover">
                                    <td class="text-sm pl-8 text-base-content/80">- {cat.name}</td>
                                    <td class="text-right font-mono text-sm">{formatCurrency(cat.total)}</td>
                                </tr>
                            {/each}
                            
                            <!-- Espacio entre personas -->
                            <tr><td colspan="2" class="h-2 border-none"></td></tr>
                        {/each}
                        
                        {#if gastosPorPersona.length === 0}
                            <tr>
                                <td colspan="2" class="text-center opacity-50 py-8">No hay gastos registrados en este mes.</td>
                            </tr>
                        {/if}
                    </tbody>
                </table>
            </div>
        </div>
    </div>
</div>
