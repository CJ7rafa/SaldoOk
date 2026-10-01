import { invoke } from "@tauri-apps/api/core";
import type { Supplier, Purchase, ExpenseCategory, Expense, Spender } from "../types";

class AppState {
  // 1. Estados Visuales Globales
  isPanelOpen = $state(false);
  selectedDate = $state(new Date());
  rightPanelView = $state<"calendar" | "manageSupplier" | "addSupplier" | "addPurchase" | "manageCategories" | "manageSpender" | "addSpenderCategory" | "addExpense" | "addDailyCash">("calendar");

  // 2. Datos de la Base de Datos
  allSuppliersMaster = $state<Supplier[]>([]);
  manuallyAddedSuppliers = $state<{ prefix: string; supplier: Supplier }[]>([]);
  allPurchases = $state<Purchase[]>([]);

  allCategoriesMaster = $state<ExpenseCategory[]>([]);
  manuallyAddedSpenderCategories = $state<{ prefix: string; spender: Spender; category: ExpenseCategory }[]>([]);
  allExpenses = $state<Expense[]>([]);
  allSpenders = $state<Spender[]>([]);
  
  allDailyCash = $state<import("../types").DailyCashRegister[]>([]);

  // 3. Estados de Edición/Selección
  selectedPurchaseCell = $state<{ dateStr: string; supplier: Supplier; existingPurchase?: Purchase } | null>(null);
  selectedSupplierToEdit = $state<Supplier | null>(null);

  selectedExpenseCell = $state<{ dateStr: string; spender: Spender; category: ExpenseCategory; existingExpense?: Expense } | null>(null);
  selectedCategoryToEdit = $state<ExpenseCategory | null>(null);
  selectedSpenderToEdit = $state<Spender | null>(null);

  selectedDailyCashCell = $state<{ dateStr: string; existingRegister?: import("../types").DailyCashRegister } | null>(null);

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

  get activeSpenderGroups() {
    const y = this.selectedDate.getFullYear();
    const m = String(this.selectedDate.getMonth() + 1).padStart(2, "0");
    const prefix = `${y}-${m}-`;

    const activeMap = new Map<number, Set<number>>();

    for (const e of this.allExpenses) {
      if (e.expense_date && e.expense_date.startsWith(prefix)) {
        if (!activeMap.has(e.spender_id)) {
          activeMap.set(e.spender_id, new Set());
        }
        activeMap.get(e.spender_id)!.add(e.category_id);
      }
    }

    for (const item of this.manuallyAddedSpenderCategories) {
      if (item.prefix === prefix) {
        if (!activeMap.has(item.spender.id)) {
          activeMap.set(item.spender.id, new Set());
        }
        activeMap.get(item.spender.id)!.add(item.category.id);
      }
    }

    const result: { spender: Spender; categories: ExpenseCategory[] }[] = [];
    
    for (const [spenderId, categoryIds] of activeMap.entries()) {
      const spender = this.allSpenders.find((s) => s.id === spenderId);
      if (spender) {
        const categories = this.allCategoriesMaster.filter((c) => categoryIds.has(c.id));
        categories.sort((a, b) => a.name.localeCompare(b.name));
        result.push({ spender, categories });
      }
    }

    result.sort((a, b) => a.spender.name.localeCompare(b.spender.name));
    return result;
  }

  async loadData() {
    try {
      this.allSuppliersMaster = await invoke<Supplier[]>("get_suppliers");
      this.allPurchases = await invoke<Purchase[]>("get_purchases");
      
      this.allCategoriesMaster = await invoke<ExpenseCategory[]>("get_expense_categories");
      this.allExpenses = await invoke<Expense[]>("get_expenses");
      this.allSpenders = await invoke<Spender[]>("get_spenders");
      
      this.allDailyCash = await invoke<import("../types").DailyCashRegister[]>("get_daily_cash_registers");
    } catch (e) {
      console.error("Error cargando base de datos:", e);
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

  addManualSpenderCategory(spender: Spender, category: ExpenseCategory) {
    const y = this.selectedDate.getFullYear();
    const m = String(this.selectedDate.getMonth() + 1).padStart(2, "0");
    const prefix = `${y}-${m}-`;
    
    const exists = this.manuallyAddedSpenderCategories.some(
      (item) => item.prefix === prefix && item.spender.id === spender.id && item.category.id === category.id
    );
    if (!exists) {
      this.manuallyAddedSpenderCategories.push({ prefix, spender, category });
    }
    this.rightPanelView = "calendar";
  }

  removeManualSpenderCategory(spenderId: number, categoryId: number) {
    this.manuallyAddedSpenderCategories = this.manuallyAddedSpenderCategories.filter(
      (item) => !(item.spender.id === spenderId && item.category.id === categoryId)
    );
  }
}

// Exportamos una única instancia (Singleton) para que toda la app comparta los mismos datos
export const appState = new AppState();
