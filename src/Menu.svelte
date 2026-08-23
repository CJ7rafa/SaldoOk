<script lang="ts">
  import { ChevronLeft, X } from "@lucide/svelte";
  import Header from "./components/Header.svelte";
  import PurchasesTable from "./views/PurchasesTable.svelte";
  import DatePicker from "./components/right-panel/DatePicker.svelte";
  import AddSupplier from "./components/right-panel/AddSupplier.svelte";
  import ManageSupplier from "./components/right-panel/ManageSupplier.svelte";
  import AddPurchase from "./components/right-panel/AddPurchase.svelte";
  import ToastContainer from "./components/ToastContainer.svelte";

  import type { Supplier, Purchase } from "./types.ts";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  // Estado del layout
  let isPanelOpen = $state(false);
  let selectedDate = $state(new Date());
  let rightPanelView = $state<
    "calendar" | "manageSupplier" | "addSupplier" | "addPurchase"
  >("calendar");

  // Estado de la tabla de compras
  let allSuppliersMaster = $state<Supplier[]>([]);
  let manuallyAddedSuppliers = $state<Supplier[]>([]);
  let allPurchases = $state<Purchase[]>([]);
  let selectedPurchaseCell = $state<{
    dateStr: string;
    supplier: Supplier;
    existingPurchase?: Purchase;
  } | null>(null);
  let selectedSupplierToEdit = $state<Supplier | null>(null);

  // Computar proveedores activos automáticamente usando la lista maestra
  let activeSuppliers = $derived(() => {
    const y = selectedDate.getFullYear();
    const m = String(selectedDate.getMonth() + 1).padStart(2, "0");
    const prefix = `${y}-${m}-`;

    const activeIds = new Set<number>();

    // 1. Filtrar IDs de proveedores con compras en este mes
    for (const p of allPurchases) {
      if (p.issue_date.startsWith(prefix)) {
        activeIds.add(p.supplier_id);
      }
    }

    // 2. Añadir IDs proveedores insertados manualmente
    for (const s of manuallyAddedSuppliers) {
      activeIds.add(s.id);
    }

    // 3. Obtener los proveedores reales desde nuestra lista maestra (que ya tiene color, etc)
    const result = allSuppliersMaster.filter((s) => activeIds.has(s.id));
    result.sort((a, b) => a.name.localeCompare(b.name));
    return result;
  });

  async function loadData() {
    try {
      allSuppliersMaster = await invoke<Supplier[]>("get_suppliers");
      allPurchases = await invoke<Purchase[]>("get_purchases");
    } catch (e) {
      console.error("Error cargando compras:", e);
    }
  }

  onMount(() => {
    loadData();
  });

  // Estado del enrutador inteligente (historial)
  let history = $state<string[]>(["menu"]);

  // Computed: vista actual
  let currentView = $derived(history[history.length - 1]);
  let canGoBack = $derived(history.length > 1);

  // Computed: título del Header basado en la vista actual
  let headerTitle = $derived(
    currentView === "menu"
      ? "Menú Principal"
      : currentView === "tablacompras"
        ? "Gestión de Compras"
        : "SaldoOk",
  );

  function navigateTo(view: string) {
    history.push(view);
  }

  function goBack() {
    if (history.length > 1) {
      history.pop();
    }
  }
</script>

<main
  class="flex h-screen w-full bg-base-200 overflow-hidden relative text-base-content"
>
  <!-- COLUMNA IZQUIERDA (Principal) -->
  <section
    class="h-full flex flex-col transition-all duration-300 bg-base-100 {isPanelOpen
      ? 'w-[75%]'
      : 'w-full'}"
  >
    <!-- HEADER GLOBAL INTELIGENTE -->
    <Header
      title={headerTitle}
      {canGoBack}
      onBack={goBack}
      {currentView}
      onRegisterSupplier={() => {
        isPanelOpen = true;
        rightPanelView = "manageSupplier";
      }}
      onAddSupplier={() => {
        isPanelOpen = true;
        rightPanelView = "addSupplier";
      }}
    />

    <!-- CONTENIDO DINÁMICO (Scrollable) -->
    <div
      class="flex-1 bg-base-200 flex flex-col min-h-0 {currentView === 'menu'
        ? 'p-8 overflow-auto'
        : 'p-0 overflow-hidden'}"
    >
      {#if currentView === "menu"}
        <div class="w-full mx-auto">
          <p class="text-base-content/70 text-lg mb-8">
            Selecciona un módulo para comenzar.
          </p>

          <div class="grid grid-cols-5 gap-4 w-full">
            <!-- BOTÓN 1: Tabla de Compras -->
            <button
              onclick={() => navigateTo("tablacompras")}
              class="h-32 bg-primary/10 hover:bg-primary/20 border border-primary/20 rounded-box flex items-center justify-center transition-colors cursor-pointer"
            >
              <span class="font-bold text-primary">Tabla de Compras</span>
            </button>

            <!-- BOTONES de Relleno -->
            {#each Array(4) as _, i}
              <button
                class="h-32 bg-base-100 hover:bg-base-300 shadow-sm border border-base-300 rounded-box flex items-center justify-center transition-colors"
              >
                Módulo {i + 2}
              </button>
            {/each}
          </div>
        </div>
      {:else if currentView === "tablacompras"}
        <PurchasesTable
          {selectedDate}
          activeSuppliers={activeSuppliers()}
          {allPurchases}
          onCellClick={(dateStr, supplier, existingPurchase) => {
            selectedPurchaseCell = { dateStr, supplier, existingPurchase };
            isPanelOpen = true;
            rightPanelView = "addPurchase";
          }}
          onRemoveSupplier={(supplierId) => {
            manuallyAddedSuppliers = manuallyAddedSuppliers.filter(
              (s) => s.id !== supplierId,
            );
          }}
          onEditSupplier={(supplier) => {
            selectedSupplierToEdit = supplier;
            isPanelOpen = true;
            rightPanelView = "manageSupplier";
          }}
        />
      {/if}
    </div>
  </section>

  <!-- COLUMNA DERECHA (Panel Lateral) -->
  <aside
    class="h-full bg-base-100 shadow-2xl transition-all duration-300 flex flex-col overflow-hidden {isPanelOpen
      ? 'w-[25%] border-l border-base-300'
      : 'w-0 border-none'}"
  >
    <div class="w-[25vw] h-full flex flex-col">
      <div
        class="p-4 flex justify-between items-center border-b border-base-300 sticky top-0 bg-base-100 z-10"
      >
        <h2 class="font-bold text-lg">Panel de Acciones</h2>
        <button
          class="btn btn-ghost btn-sm btn-square"
          onclick={() => (isPanelOpen = false)}
        >
          <X size={20} />
        </button>
      </div>

      <div
        class="p-4 space-y-4 overflow-y-auto overflow-x-hidden flex-1 bg-base-100"
      >
        {#if rightPanelView === "calendar"}
          <!-- DatePicker Calendar -->
          <DatePicker {selectedDate} onDateChange={(d) => (selectedDate = d)} />

          <div class="divider"></div>

          <p class="text-sm opacity-70">
            Aquí cargaremos dinámicamente formularios como "Agregar Proveedor".
          </p>
          {#each Array(10) as _, i}
            <button
              class="w-full p-3 bg-base-200 hover:bg-base-300 transition-colors rounded-lg text-sm text-left truncate"
            >
              Configuración {i + 1}
            </button>
          {/each}
        {:else if rightPanelView === "manageSupplier"}
          {#key selectedSupplierToEdit?.id}
            <ManageSupplier
              initialEditSupplier={selectedSupplierToEdit}
              onClose={() => {
                rightPanelView = "calendar";
                selectedSupplierToEdit = null;
              }}
              onSuccess={() => {
                // Si se editó con éxito, recargamos los datos para actualizar nombres y colores
                loadData();
                rightPanelView = "calendar";
                selectedSupplierToEdit = null;
              }}
            />
          {/key}
        {:else if rightPanelView === "addSupplier"}
          <AddSupplier
            activeSuppliers={activeSuppliers()}
            onClose={() => (rightPanelView = "calendar")}
            onAdd={(supplier) => {
              manuallyAddedSuppliers.push(supplier);
              rightPanelView = "calendar";
            }}
          />
        {:else if rightPanelView === "addPurchase" && selectedPurchaseCell}
          {#key selectedPurchaseCell}
            <AddPurchase
              dateStr={selectedPurchaseCell.dateStr}
              supplier={selectedPurchaseCell.supplier}
              existingPurchase={selectedPurchaseCell.existingPurchase}
              onClose={() => {
                rightPanelView = "calendar";
                selectedPurchaseCell = null;
              }}
              onSuccess={() => {
                loadData();
              }}
            />
          {/key}
        {/if}
      </div>
    </div>
  </aside>

  <!-- BURBUJA PARA ABRIR EL PANEL -->
  {#if !isPanelOpen}
    <button
      class="absolute right-0 top-1/2 -translate-y-1/2 bg-primary text-primary-content hover:brightness-110 transition-all rounded-l-full w-10 h-16 shadow-lg z-50 flex items-center justify-start pl-1 cursor-pointer border-none"
      onclick={() => (isPanelOpen = true)}
    >
      <ChevronLeft size={24} />
    </button>
  {/if}

  <!-- NOTIFICACIONES GLOBALES -->
  <ToastContainer />
</main>
