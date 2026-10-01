<script lang="ts">
    import type { DailyCashRegister, Purchase, Expense } from "../types";

    let props: {
        selectedDate: Date;
        allDailyCash: DailyCashRegister[];
        allPurchases: Purchase[];
        allExpenses: Expense[];
        onCellClick?: (
            dateStr: string,
            existingRegister?: DailyCashRegister,
        ) => void;
    } = $props();

    // Calculamos los días del mes reactivamente
    let dias = $derived(generarDias(props.selectedDate));

    let currentMonthName = $derived(() => {
        const monthName = props.selectedDate.toLocaleDateString("es-ES", {
            month: "long",
        });
        return monthName.charAt(0).toUpperCase() + monthName.slice(1);
    });

    function generarDias(fecha: Date) {
        const year = fecha.getFullYear();
        const month = fecha.getMonth();
        const daysInMonth = new Date(year, month + 1, 0).getDate();

        return Array.from({ length: daysInMonth }, (_, i) => {
            const date = new Date(year, month, i + 1);
            const dayName = date.toLocaleDateString("es-ES", {
                weekday: "long",
            });
            const capitalizedDay =
                dayName.charAt(0).toUpperCase() + dayName.slice(1);
            return { dayName: capitalizedDay, dayNumber: i + 1 };
        });
    }

    function getRegister(dateStr: string): DailyCashRegister | undefined {
        if (!props.allDailyCash) return undefined;
        return props.allDailyCash.find((r) => r.entry_date === dateStr);
    }

    function formatCurrency(amount: number): string {
        return new Intl.NumberFormat("en-US", {
            style: "currency",
            currency: "USD",
        }).format(amount);
    }

    function getPreviousDayMarketSales(dateStr: string): number {
        if (!props.allDailyCash) return 0;
        const [y, m, d] = dateStr.split("-").map(Number);
        const prevDate = new Date(y, m - 1, d - 1);
        const prevDateStr = `${prevDate.getFullYear()}-${String(prevDate.getMonth() + 1).padStart(2, "0")}-${String(prevDate.getDate()).padStart(2, "0")}`;
        const prevReg = getRegister(prevDateStr);
        return prevReg ? prevReg.market_sales : 0;
    }

    function getDailyPurchases(dateStr: string): number {
        if (!props.allPurchases) return 0;
        return props.allPurchases
            .filter((p) => p.issue_date === dateStr)
            .reduce((sum, p) => sum + p.total_amount, 0);
    }

    function getDailyExpenses(dateStr: string): number {
        if (!props.allExpenses) return 0;
        return props.allExpenses
            .filter((e) => e.expense_date === dateStr)
            .reduce((sum, e) => sum + e.amount, 0);
    }
    function getTotal(
        field:
            | "market_sales"
            | "market_vault_saved"
            | "house_sales"
            | "house_shop_savings"
            | "chain_income"
            | "vault_loan"
            | "self_consumption",
    ): number {
        if (!props.allDailyCash) return 0;
        const y = props.selectedDate.getFullYear();
        const m = String(props.selectedDate.getMonth() + 1).padStart(2, "0");
        const prefix = `${y}-${m}-`;

        return props.allDailyCash
            .filter((r) => r.entry_date.startsWith(prefix))
            .reduce((sum, r) => sum + (r[field] || 0), 0);
    }

    let calculatedTotals = $derived.by(() => {
        let totalVentasMercado = 0;
        let totalFlujoEfectivoFinal = 0;
        let totalGananciaDiaria = 0;

        const y = props.selectedDate.getFullYear();
        const m = String(props.selectedDate.getMonth() + 1).padStart(2, "0");
        const daysInMonth = new Date(
            y,
            props.selectedDate.getMonth() + 1,
            0,
        ).getDate();

        for (let i = 1; i <= daysInMonth; i++) {
            const d = String(i).padStart(2, "0");
            const dateStr = `${y}-${m}-${d}`;
            const reg = getRegister(dateStr);
            const prevMarketSales = getPreviousDayMarketSales(dateStr);
            const dailyPurchases = getDailyPurchases(dateStr);
            const dailyExpenses = getDailyExpenses(dateStr);
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
                const flujoEfectivoFinal =
                    ventasMercado +
                    prevMarketSales +
                    feAhorrosTiendaCasa +
                    (reg?.house_sales || 0) -
                    (dailyPurchases + dailyExpenses) +
                    (reg?.chain_income || 0) +
                    (reg?.vault_loan || 0);
                const gananciaDiaria = ventasMercado * 0.13;

                totalVentasMercado += ventasMercado;
                totalFlujoEfectivoFinal += flujoEfectivoFinal;
                totalGananciaDiaria += gananciaDiaria;
            }
        }

        return {
            totalVentasMercado,
            totalFlujoEfectivoFinal,
            totalGananciaDiaria,
        };
    });
</script>

<div class="w-full flex-1 flex flex-col min-h-0 pb-2">
    <div
        class="overflow-auto bg-base-100 shadow-xl rounded-box border border-base-300 w-full flex-1 min-h-0"
    >
        <table class="table table-pin-rows table-pin-cols w-full table-sm">
            <thead>
                <!-- FILA 1 -->
                <tr class="bg-base-200" style="z-index: 50;">
                    <th
                        rowspan="3"
                        class="bg-base-300 border-r border-base-300 min-w-[120px] max-w-[120px] text-center"
                        style="position: sticky; left: 0; width: 120px; z-index: 60; vertical-align: middle;"
                    >
                        <div class="text-xs uppercase opacity-70">Fecha</div>
                        <div class="text-sm font-black mt-1 text-primary">
                            {currentMonthName()}
                        </div>
                    </th>
                    <!-- MERCADO -->
                    <th
                        colspan="3"
                        class="bg-success/20 border-r border-base-300 text-center font-bold text-base-content text-xs uppercase tracking-wider py-1"
                    >
                        Mercado
                    </th>
                    <!-- CASA -->
                    <th
                        colspan="1"
                        class="bg-warning/20 border-r border-base-300 text-center font-bold text-base-content text-xs uppercase tracking-wider py-1"
                    >
                        Casa
                    </th>
                    <!-- OTROS INGRESOS -->
                    <th
                        colspan="3"
                        class="bg-info/20 border-r border-base-300 text-center font-bold text-base-content text-xs uppercase tracking-wider py-1"
                    >
                        Otros Ingresos
                    </th>
                    <!-- CALCULADOS (Rowspan 3) -->
                    <th
                        rowspan="3"
                        class="bg-base-200 border-r border-base-300 text-center text-xs font-bold text-base-content uppercase"
                        style="vertical-align: middle;"
                    >
                        Ventas Netas<br />(Mercado)
                    </th>
                    <th
                        rowspan="3"
                        class="bg-base-200 border-r border-base-300 text-center text-xs font-bold text-base-content uppercase"
                        style="vertical-align: middle;"
                    >
                        Flujo Final<br />Diario
                    </th>
                    <th
                        rowspan="3"
                        class="bg-base-200 border-r border-base-300 text-center text-xs font-bold text-base-content uppercase"
                        style="vertical-align: middle;"
                    >
                        Ganancia Diaria<br />(13%)
                    </th>
                    <!-- AUTO CONSUMO (Rowspan 3) -->
                    <th
                        rowspan="3"
                        class="bg-base-200 border-r border-base-300 text-center text-xs font-bold text-base-content uppercase"
                        style="vertical-align: middle;"
                    >
                        Autoconsumo<br />Rafa/Casa
                    </th>
                </tr>

                <!-- FILA 2 -->
                <tr class="bg-base-200 shadow-sm" style="z-index: 49;">
                    <!-- MERCADO -->
                    <th
                        colspan="2"
                        class="bg-success/10 border-r border-base-300 text-center text-xs font-bold text-base-content py-1"
                    >
                        Flujo de Efectivo
                    </th>
                    <th
                        rowspan="2"
                        class="bg-success/20 border-r border-base-300 text-center text-xs font-bold text-base-content whitespace-normal max-w-[100px]"
                        style="vertical-align: middle;"
                    >
                        Efectivo Anterior
                    </th>

                    <!-- CASA -->
                    <th
                        rowspan="2"
                        class="bg-warning/20 border-r border-base-300 text-center text-xs font-bold text-base-content whitespace-normal max-w-[100px]"
                        style="vertical-align: middle;"
                    >
                        Ventas del Día
                    </th>

                    <!-- OTROS INGRESOS -->
                    <th
                        colspan="1"
                        class="bg-info/10 border-r border-base-300 text-center text-xs font-bold text-base-content py-1"
                    >
                        Tienda Casa
                    </th>
                    <th
                        colspan="2"
                        class="bg-info/10 border-r border-base-300 text-center text-xs font-bold text-base-content py-1"
                    >
                        Mercado
                    </th>
                </tr>

                <!-- FILA 3 -->
                <tr class="bg-base-200 shadow-sm" style="z-index: 48;">
                    <!-- Flujo de efectivo diario -->
                    <th
                        class="bg-success/10 border-r border-base-300 text-center text-xs font-medium text-base-content py-1"
                        >Ingreso</th
                    >
                    <th
                        class="bg-success/10 border-r border-base-300 text-center text-xs font-medium text-base-content py-1"
                        >Ahorro</th
                    >

                    <!-- tienda casa -->
                    <th
                        class="bg-info/10 border-r border-base-300 text-center text-xs font-medium text-base-content py-1"
                        >Ahorro</th
                    >
                    <!-- mercado -->
                    <th
                        class="bg-info/10 border-r border-base-300 text-center text-xs font-medium text-base-content py-1"
                        >Ahorro</th
                    >
                    <th
                        class="bg-info/10 border-r border-base-300 text-center text-xs font-medium text-base-content py-1"
                        >Préstamo</th
                    >
                </tr>
            </thead>

            <!-- CUERPO DE LA TABLA -->
            <tbody class="relative">
                {#each dias as dia, i (dia.dayNumber)}
                    {@const y = props.selectedDate.getFullYear()}
                    {@const m = String(
                        props.selectedDate.getMonth() + 1,
                    ).padStart(2, "0")}
                    {@const d = String(i + 1).padStart(2, "0")}
                    {@const dateStr = `${y}-${m}-${d}`}
                    {@const reg = getRegister(dateStr)}

                    {@const prevMarketSales =
                        getPreviousDayMarketSales(dateStr)}
                    {@const dailyPurchases = getDailyPurchases(dateStr)}
                    {@const dailyExpenses = getDailyExpenses(dateStr)}
                    {@const feAhorrosTiendaCasa = reg?.house_shop_savings || 0}
                    {@const hasMarketSales = reg && reg.market_sales > 0}

                    {@const ventasMercado = hasMarketSales ? 
                        dailyPurchases +
                        dailyExpenses +
                        (reg?.market_sales || 0) +
                        (reg?.market_vault_saved || 0) -
                        prevMarketSales -
                        (reg?.house_sales || 0) -
                        feAhorrosTiendaCasa -
                        (reg?.chain_income || 0) -
                        (reg?.vault_loan || 0) : 0}
                    {@const flujoEfectivoFinal = hasMarketSales ?
                        ventasMercado +
                        prevMarketSales +
                        feAhorrosTiendaCasa +
                        (reg?.house_sales || 0) -
                        (dailyPurchases + dailyExpenses) +
                        (reg?.chain_income || 0) +
                        (reg?.vault_loan || 0) : 0}
                    {@const gananciaDiaria = hasMarketSales ? ventasMercado * 0.13 : 0}

                    <tr
                        class="hover cursor-pointer"
                        onclick={() => {
                            if (props.onCellClick) {
                                props.onCellClick(dateStr, reg);
                            }
                        }}
                    >
                        <!-- Fecha Fija -->
                        <td
                            class="bg-base-200 border-r border-base-300 whitespace-nowrap min-w-[120px] max-w-[120px]"
                            style="position: sticky; left: 0; width: 120px;"
                        >
                            <div
                                class="flex justify-between items-center w-full {dia.dayName ===
                                'Domingo'
                                    ? 'text-error font-bold'
                                    : ''}"
                            >
                                <span>{dia.dayName}</span>
                                <span class="opacity-60">{dia.dayNumber}</span>
                            </div>
                        </td>

                        <!-- MERCADO / MES -->
                        <td
                            class="text-right border-r border-base-300 font-mono text-sm text-base-content group-hover:bg-success/10 transition-colors {reg && reg.market_sales > 0 ? 'bg-success/10' : ''}"
                        >
                            {reg && reg.market_sales !== 0
                                ? formatCurrency(reg.market_sales)
                                : "-"}
                        </td>
                        <td
                            class="text-right border-r border-base-300 font-mono text-sm text-base-content group-hover:bg-success/10 transition-colors {reg && reg.market_vault_saved > 0 ? 'bg-success/10' : ''}"
                        >
                            {reg && reg.market_vault_saved !== 0
                                ? formatCurrency(reg.market_vault_saved)
                                : "-"}
                        </td>
                        <td
                            class="text-right border-r border-base-300 font-mono text-sm text-base-content bg-base-200/50"
                        >
                            <!-- F/E día anterior o mes -->
                            {prevMarketSales !== 0
                                ? formatCurrency(prevMarketSales)
                                : "-"}
                        </td>

                        <!-- CASA -->
                        <td
                            class="text-right border-r border-base-300 font-mono text-sm text-base-content group-hover:bg-warning/10 transition-colors {reg && reg.house_sales > 0 ? 'bg-warning/10' : ''}"
                        >
                            {reg && reg.house_sales !== 0
                                ? formatCurrency(reg.house_sales)
                                : "-"}
                        </td>

                        <!-- OTROS INGRESOS -->
                        <td
                            class="text-right border-r border-base-300 font-mono text-sm text-base-content group-hover:bg-info/10 transition-colors {reg && reg.house_shop_savings > 0 ? 'bg-info/10' : ''}"
                        >
                            {reg && reg.house_shop_savings !== 0
                                ? formatCurrency(reg.house_shop_savings)
                                : "-"}
                        </td>
                        <td
                            class="text-right border-r border-base-300 font-mono text-sm text-base-content group-hover:bg-info/10 transition-colors {reg && reg.chain_income > 0 ? 'bg-info/10' : ''}"
                        >
                            {reg && reg.chain_income !== 0
                                ? formatCurrency(reg.chain_income)
                                : "-"}
                        </td>
                        <td
                            class="text-right border-r border-base-300 font-mono text-sm text-base-content group-hover:bg-info/10 transition-colors {reg && reg.vault_loan > 0 ? 'bg-info/10' : ''}"
                        >
                            {reg && reg.vault_loan !== 0
                                ? formatCurrency(reg.vault_loan)
                                : "-"}
                        </td>

                        <!-- CALCULADOS -->
                        <td
                            class="text-right border-r border-base-300 font-mono text-sm font-medium text-base-content bg-base-200 {hasMarketSales && ventasMercado >= 0 ? 'bg-success/10' : ''} {hasMarketSales && ventasMercado < 0 ? 'bg-error/10' : ''}"
                        >
                            <!-- Ventas mercado -->
                            {hasMarketSales
                                ? formatCurrency(ventasMercado)
                                : "-"}
                        </td>
                        <td
                            class="text-right border-r border-base-300 font-mono text-sm font-bold text-base-content {hasMarketSales ? 'bg-error/10' : ''}"
                        >
                            <!-- Flujo final -->
                            {hasMarketSales
                                ? formatCurrency(flujoEfectivoFinal)
                                : "-"}
                        </td>
                        <td
                            class="text-right border-r border-base-300 font-mono text-sm font-bold text-base-content {hasMarketSales ? 'bg-success/10' : ''}"
                        >
                            <!-- Ganancia diaria -->
                            {hasMarketSales
                                ? formatCurrency(gananciaDiaria)
                                : "-"}
                        </td>

                        <!-- AUTO CONSUMO -->
                        <td
                            class="text-right border-r border-base-300 font-mono text-sm text-base-content group-hover:bg-base-300 transition-colors"
                            class:font-medium={reg && reg.self_consumption > 0}
                        >
                            {reg && reg.self_consumption !== 0
                                ? formatCurrency(reg.self_consumption)
                                : "-"}
                        </td>
                    </tr>
                {/each}
            </tbody>

            <!-- FOOTER CON TOTALES MENSUALES -->
            <tfoot style="position: sticky; bottom: 0; z-index: 50;">
                <tr
                    class="bg-base-300 shadow-[0_-4px_6px_-1px_rgba(0,0,0,0.1)] h-10"
                >
                    <!-- Fecha / Total Mensual -->
                    <th
                        class="border-r border-t border-base-300 whitespace-nowrap font-bold uppercase min-w-[120px] max-w-[120px] bg-base-300 text-base-content"
                        style="position: sticky; left: 0; width: 120px; z-index: 60;"
                    >
                        Total Mensual
                    </th>

                    <!-- MERCADO -->
                    <th
                        class="text-right border-r border-t border-base-300 font-mono text-base font-bold text-base-content"
                    >
                        {formatCurrency(getTotal("market_sales"))}
                    </th>
                    <th
                        class="text-right border-r border-t border-base-300 font-mono text-base font-bold text-base-content"
                    >
                        {formatCurrency(getTotal("market_vault_saved"))}
                    </th>
                    <th
                        class="text-right border-r border-t border-base-300 bg-base-300/50"
                    ></th>

                    <!-- CASA -->
                    <th
                        class="text-right border-r border-t border-base-300 font-mono text-base font-bold text-base-content"
                    >
                        {formatCurrency(getTotal("house_sales"))}
                    </th>

                    <!-- OTROS INGRESOS -->
                    <th
                        class="text-right border-r border-t border-base-300 font-mono text-base font-bold text-base-content"
                    >
                        {formatCurrency(getTotal("house_shop_savings"))}
                    </th>
                    <th
                        class="text-right border-r border-t border-base-300 font-mono text-base font-bold text-base-content"
                    >
                        {formatCurrency(getTotal("chain_income"))}
                    </th>
                    <th
                        class="text-right border-r border-t border-base-300 font-mono text-base font-bold text-base-content"
                    >
                        {formatCurrency(getTotal("vault_loan"))}
                    </th>

                    <!-- CALCULADOS -->
                    <th
                        class="text-right border-r border-t border-base-300 font-mono text-base font-bold text-base-content"
                    >
                        {formatCurrency(calculatedTotals.totalVentasMercado)}
                    </th>
                    <th
                        class="text-right border-r border-t border-base-300 font-mono text-base font-bold text-base-content"
                    >
                        {formatCurrency(
                            calculatedTotals.totalFlujoEfectivoFinal,
                        )}
                    </th>
                    <th
                        class="text-right border-r border-t border-base-300 font-mono text-base font-bold text-base-content"
                    >
                        {formatCurrency(calculatedTotals.totalGananciaDiaria)}
                    </th>

                    <!-- AUTO CONSUMO -->
                    <th
                        class="text-right border-r border-t border-base-300 font-mono text-base font-bold text-base-content"
                    >
                        {formatCurrency(getTotal("self_consumption"))}
                    </th>
                </tr>
            </tfoot>
        </table>
    </div>
</div>