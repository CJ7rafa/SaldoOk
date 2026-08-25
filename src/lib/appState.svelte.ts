import { invoke } from "@tauri-apps/api/core";
import type { Supplier, Purchase } from "../types";

class AppState {
  // 1. Estados Visuales Globales
  isPanelOpen = $state(false);
  selectedDate = $state(new Date());
  rightPanelView = $state<"calendar" | "manageSupplier" | "addSupplier" | "addPurchase">("calendar");

  // 2. Datos de la Base de Datos
  allSuppliersMaster = $state<Supplier[]>([]);
  manuallyAddedSuppliers = $state<{ prefix: string; supplier: Supplier }[]>([]);
  allPurchases = $state<Purchase[]>([]);

  // 3. Estados de Edición/Selección
  selectedPurchaseCell = $state<{ dateStr: string; supplier: Supplier; existingPurchase?: Purchase } | null>(null);
  selectedSupplierToEdit = $state<Supplier | null>(null);

  // 4. Valores Calculados (Derivados automáticamente)
  // Al usar "get", Svelte 5 automáticamente lo hace reactivo cuando cambian las variables internas
  get activeSuppliers() {
    const y = this.selectedDate.getFullYear();
    const m = String(this.selectedDate.getMonth() + 1).padStart(2, "0");
    const prefix = `${y}-${m}-`;

    const activeIds = new Set<number>();

    // Filtrar IDs de proveedores con compras en este mes
    for (const p of this.allPurchases) {
      if (p.issue_date.startsWith(prefix)) {
        activeIds.add(p.supplier_id);
      }
    }

    // Añadir IDs proveedores insertados manualmente (solo para este mes)
    for (const item of this.manuallyAddedSuppliers) {
      if (item.prefix === prefix) {
        activeIds.add(item.supplier.id);
      }
    }

    // Obtener los proveedores reales desde nuestra lista maestra
    const result = this.allSuppliersMaster.filter((s) => activeIds.has(s.id));
    result.sort((a, b) => a.name.localeCompare(b.name));
    return result;
  }

  // 5. Métodos Globales
  async loadData() {
    try {
      this.allSuppliersMaster = await invoke<Supplier[]>("get_suppliers");
      this.allPurchases = await invoke<Purchase[]>("get_purchases");
    } catch (e) {
      console.error("Error cargando compras:", e);
    }
  }

  addManualSupplier(supplier: Supplier) {
    const y = this.selectedDate.getFullYear();
    const m = String(this.selectedDate.getMonth() + 1).padStart(2, "0");
    const prefix = `${y}-${m}-`;
    this.manuallyAddedSuppliers.push({ prefix, supplier });
    this.rightPanelView = "calendar";
  }

  removeManualSupplier(supplierId: number) {
    this.manuallyAddedSuppliers = this.manuallyAddedSuppliers.filter(
      (item) => item.supplier.id !== supplierId
    );
  }
}

// Exportamos una única instancia (Singleton) para que toda la app comparta los mismos datos
export const appState = new AppState();
