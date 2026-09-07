<script lang="ts">
  import type { Supplier, Purchase } from "../types";
  import { Trash2, Pencil } from "@lucide/svelte";
  import { notifications } from "../lib/notifications.svelte";

  let props: {
    selectedDate: Date;
    activeSuppliers: Supplier[];
    allPurchases: Purchase[];
    onCellClick?: (
      dateStr: string,
      supplier: Supplier,
      existingPurchase?: Purchase,
    ) => void;
    onRemoveSupplier?: (supplierId: number) => void;
    onEditSupplier?: (supplier: Supplier) => void;
  } = $props();

  function getTextColor(color: string) {
    switch (color) {
      case "RED":
        return "text-red-500";
      case "BLUE":
        return "text-blue-500";
      case "GREEN":
        return "text-emerald-500";
      case "ORANGE":
        return "text-orange-500";
      default:
        return "";
    }
  }

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
      const dayName = date.toLocaleDateString("es-ES", { weekday: "long" });
      const capitalizedDay = dayName.charAt(0).toUpperCase() + dayName.slice(1);
      return { dayName: capitalizedDay, dayNumber: i + 1 };
    });
  }

  // Helper para buscar compras y evitar errores de TS en el template
  function getPurchase(
    supplierId: number,
    dateStr: string,
  ): Purchase | undefined {
    if (!props.allPurchases) return undefined;
    return props.allPurchases.find(
      (p: Purchase) => p.supplier_id === supplierId && p.issue_date === dateStr,
    );
  }

  // Total de un proveedor en el mes
  function getSupplierTotal(supplierId: number): number {
    if (!props.allPurchases) return 0;
    const y = props.selectedDate.getFullYear();
    const m = String(props.selectedDate.getMonth() + 1).padStart(2, "0");
    const prefix = `${y}-${m}-`;
    return props.allPurchases
      .filter(
        (p: Purchase) =>
          p.supplier_id === supplierId && p.issue_date.startsWith(prefix),
      )
      .reduce((sum: number, p: Purchase) => sum + p.total_amount, 0);
  }

  // Total de TODOS los proveedores en un día específico
  function getDayTotal(dateStr: string): number {
    if (!props.allPurchases) return 0;
    return props.allPurchases
      .filter((p: Purchase) => p.issue_date === dateStr)
      .reduce((sum: number, p: Purchase) => sum + p.total_amount, 0);
  }

  // Total GENERAL del mes
  function getMonthTotal(): number {
    if (!props.allPurchases) return 0;
    const y = props.selectedDate.getFullYear();
    const m = String(props.selectedDate.getMonth() + 1).padStart(2, "0");
    const prefix = `${y}-${m}-`;
    return props.allPurchases
      .filter((p: Purchase) => p.issue_date.startsWith(prefix))
      .reduce((sum: number, p: Purchase) => sum + p.total_amount, 0);
  }

  function tryRemoveSupplier(supplier: Supplier) {
    const y = props.selectedDate.getFullYear();
    const m = String(props.selectedDate.getMonth() + 1).padStart(2, "0");
    const prefix = `${y}-${m}-`;

    const hasPurchases = props.allPurchases.some(
      (p: Purchase) =>
        p.supplier_id === supplier.id && p.issue_date.startsWith(prefix),
    );

    if (hasPurchases) {
      notifications.show(
        "No se puede eliminar",
        "El proveedor tiene registros de compra en este mes.",
        "warning",
      );
    } else {
      if (props.onRemoveSupplier) {
        props.onRemoveSupplier(supplier.id);
      }
    }
  }

  function formatCurrency(amount: number): string {
    return new Intl.NumberFormat("en-US", {
      style: "currency",
      currency: "USD",
    }).format(amount);
  }
</script>

<div class="w-full flex-1 flex flex-col min-h-0 pb-2">
  <div
    class="overflow-auto bg-base-100 shadow-xl rounded-box border border-base-300 w-full flex-1 min-h-0"
  >
    <!-- table-pin-rows fija los headers arriba, table-pin-cols fija la primera columna a la izquierda -->
    <table class="table table-zebra table-pin-rows table-pin-cols w-full">
      <!-- HEADER DOBLE FILA -->
      <thead>
        <!-- FILA 1: Celda combinada -->
        <tr class="bg-base-200" style="z-index: 50;">
          <th
            class="bg-base-300 border-r border-base-300 min-w-[120px] max-w-[120px] text-center"
            style="position: sticky; left: 0; width: 120px; z-index: 60;"
          >
            <div class="text-xs uppercase opacity-70">Fecha</div>
            <div class="text-x5 mt-2 text-primary">
              {currentMonthName()}
            </div>
          </th>
          <th
            class="bg-base-300 border-r border-base-300 min-w-[105px] max-w-[105px]"
            style="position: sticky; left: 120px; width: 105px; z-index: 60;"
            >Total Día</th
          >

          <!-- Celda Combinada usando 'colspan' -->
          <th
            colspan={props.activeSuppliers.length || 1}
            class="!right-auto text-center bg-base-200 text-sm uppercase tracking-wider font-bold border-b border-base-300 py-1"
          >
            Proveedores
          </th>
        </tr>

        <!-- FILA 2: Lista de proveedores -->
        <tr class="bg-base-200 shadow-sm" style="z-index: 50;">
          <th
            class="bg-base-300 border-r border-base-300"
            style="position: sticky; left: 0; width: 120px; z-index: 60;"
          ></th>
          <th
            class="bg-base-300 border-r border-base-300"
            style="position: sticky; left: 120px; width: 105px; z-index: 60;"
          ></th>
          <!-- Celdas Dinámicas (Nombres de Proveedores) -->
          {#if props.activeSuppliers.length === 0}
            <th class="text-center text-sm font-normal opacity-60 bg-base-100">
              Usa el botón "Agregar proveedor" para añadir columnas a la tabla
            </th>
          {:else}
            {#each props.activeSuppliers as proveedor (proveedor.id)}
              <th
                class="!right-auto text-center min-w-[100px] border-r border-base-300 last:border-r-0 group relative"
                style="position: relative; z-index: 50;"
              >
                <span class={getTextColor(proveedor.color)}>
                  {proveedor.name}
                </span>

                <!-- Botón de editar -->
                <button
                  class="absolute top-1/2 -translate-y-1/2 right-7 opacity-0 group-hover:opacity-100 btn btn-xs btn-ghost btn-circle text-success transition-opacity"
                  onclick={() => {
                    if (props.onEditSupplier) props.onEditSupplier(proveedor);
                  }}
                  title="Editar proveedor"
                >
                  <Pencil size={14} />
                </button>

                <!-- Botón de ocultar -->
                <button
                  class="absolute top-1/2 -translate-y-1/2 right-1 opacity-0 group-hover:opacity-100 btn btn-xs btn-ghost btn-circle text-error transition-opacity"
                  onclick={() => tryRemoveSupplier(proveedor)}
                  title="Ocultar de la tabla"
                >
                  <Trash2 size={14} />
                </button>
              </th>
            {/each}
          {/if}
        </tr>
      </thead>

      <!-- CUERPO DE LA TABLA (Días del mes) -->
      <tbody>
        {#each dias as dia, i (dia)}
          {@const y = props.selectedDate.getFullYear()}
          {@const m = String(props.selectedDate.getMonth() + 1).padStart(
            2,
            "0",
          )}
          {@const d = String(i + 1).padStart(2, "0")}
          {@const dateStr = `${y}-${m}-${d}`}

          <tr class="hover">
            <!-- Primera columna (Fecha - se queda fija al scrollear a la derecha) -->
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

            <!-- Segunda columna (Total del día - se queda fija) -->
            <td
              class="bg-base-200 border-r text-right border-base-300 font-bold whitespace-nowrap min-w-[105px] max-w-[105px]"
              style="position: sticky; left: 120px; width: 105px;"
            >
              {formatCurrency(getDayTotal(dateStr))}
            </td>

            <!-- Celdas de datos para cada proveedor -->
            {#if props.activeSuppliers.length === 0}
              <td
                class="text-center border-r border-base-300 last:border-r-0 cursor-pointer bg-base-100"
              >
              </td>
            {:else}
              {#each props.activeSuppliers as proveedor (proveedor.id)}
                {@const existingPurchase = getPurchase(proveedor.id, dateStr)}

                <td
                  class="text-center border-r border-base-300 last:border-r-0 cursor-pointer hover:bg-primary/10 transition-colors"
                  class:bg-success:={existingPurchase &&
                    existingPurchase.status === "PAID"}
                  class:bg-success-content:={existingPurchase &&
                    existingPurchase.status === "PAID"}
                  class:bg-warning:={existingPurchase &&
                    existingPurchase.status === "PENDING"}
                  class:bg-warning-content:={existingPurchase &&
                    existingPurchase.status === "PENDING"}
                  onclick={() => {
                    if (props.onCellClick) {
                      props.onCellClick(dateStr, proveedor, existingPurchase);
                    }
                  }}
                >
                  {#if existingPurchase}
                    <span class="font-medium text-sm">
                      {formatCurrency(existingPurchase.total_amount)}
                    </span>
                  {:else}
                    <span class="opacity-40 text-sm italic">-</span>
                  {/if}
                </td>
              {/each}
            {/if}
          </tr>
        {/each}
      </tbody>

      <!-- FOOTER DE LA TABLA (Totales mensuales fijos abajo) -->
      <tfoot>
        <tr
          class="bg-base-200 shadow-[0_-4px_6px_-1px_rgba(0,0,0,0.1)]"
          style="z-index: 50;"
        >
          <th
            class="bg-base-300 border-r border-t border-base-300 whitespace-nowrap font-bold uppercase min-w-[120px] max-w-[120px]"
            style="position: sticky; left: 0; width: 120px; z-index: 60;"
            >Total Mensual</th
          >
          <th
            class="bg-base-300 border-r border-t border-base-300 font-bold text-lg text-primary min-w-[105px] max-w-[105px]"
            style="position: sticky; left: 120px; width: 105px; z-index: 60;"
          >
            $ {getMonthTotal().toFixed(2)}
          </th>

          {#if props.activeSuppliers.length === 0}
            <th class="text-center bg-base-100 border-t border-base-300"></th>
          {:else}
            {#each props.activeSuppliers as proveedor (proveedor.id)}
              <th
                class="!right-auto text-center border-r border-t border-base-300 last:border-r-0 font-bold text-lg text-success"
              >
                $ {getSupplierTotal(proveedor.id).toFixed(2)}
              </th>
            {/each}
          {/if}
        </tr>
      </tfoot>
    </table>
  </div>
</div>
