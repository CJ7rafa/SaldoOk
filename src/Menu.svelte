<script lang="ts">
  import { ChevronLeft, X } from "@lucide/svelte";
  import Header from "./components/Header.svelte";
  import PurchasesTable from "./views/PurchasesTable.svelte";
  import ExpensesTable from "./views/ExpenesTable.svelte";
  import DatePicker from "./components/right-panel/DatePicker.svelte";
  import AddSupplier from "./components/right-panel/purchase/AddSupplier.svelte";
  import ManageSupplier from "./components/right-panel/purchase/ManageSupplier.svelte";
  import AddPurchase from "./components/right-panel/purchase/AddPurchase.svelte";
  import ManageCategories from "./components/right-panel/expense/ManageCategories.svelte";
  import ManageSpender from "./components/right-panel/expense/ManageSpender.svelte";
  import AddSpenderCategories from "./components/right-panel/expense/AddSpender-Categories.svelte";
  import AddExpense from "./components/right-panel/expense/AddExpense.svelte";
  import ToastContainer from "./components/ToastContainer.svelte";
  import ConfirmationModal from "./components/ConfirmationModal.svelte";

  import { onMount } from "svelte";

  // Aquí está la magia: Importamos todo nuestro estado global desde un solo archivo
  import { appState } from "./lib/appState.svelte";

  onMount(() => {
    appState.loadData();
  });

  // Estado del enrutador inteligente (historial) - Este lo dejamos aquí porque es exclusivo visual de este componente
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
        : currentView === "tablagastos"
          ? "Gestión de Gastos"
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
    class="h-full flex flex-col transition-all duration-300 bg-base-100 {appState.isPanelOpen
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
        appState.isPanelOpen = true;
        appState.rightPanelView = "manageSupplier";
      }}
      onAddSupplier={() => {
        appState.isPanelOpen = true;
        appState.rightPanelView = "addSupplier";
      }}
      onManageCategories={() => {
        appState.isPanelOpen = true;
        appState.rightPanelView = "manageCategories";
      }}
      onManageSpenders={() => {
        appState.isPanelOpen = true;
        appState.rightPanelView = "manageSpender";
      }}
      onAddSpenderCategory={() => {
        appState.isPanelOpen = true;
        appState.rightPanelView = "addSpenderCategory";
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
            <button
              onclick={() => navigateTo("tablacompras")}
              class="h-32 bg-primary/10 hover:bg-primary/20 border border-primary/20 rounded-box flex items-center justify-center transition-colors cursor-pointer"
            >
              <span class="font-bold text-primary">Tabla de Compras</span>
            </button>

            <button
              onclick={() => navigateTo("tablagastos")}
              class="h-32 bg-primary/10 hover:bg-primary/20 border border-primary/20 rounded-box flex items-center justify-center transition-colors cursor-pointer"
            >
              <span class="font-bold text-primary">Tabla de Gastos</span>
            </button>
          </div>
        </div>
      {:else if currentView === "tablacompras"}
        <PurchasesTable
          selectedDate={appState.selectedDate}
          activeSuppliers={appState.activeSuppliers}
          allPurchases={appState.allPurchases}
          onCellClick={(dateStr, supplier, existingPurchase) => {
            appState.selectedPurchaseCell = {
              dateStr,
              supplier,
              existingPurchase,
            };
            appState.isPanelOpen = true;
            appState.rightPanelView = "addPurchase";
          }}
          onRemoveSupplier={(supplierId) =>
            appState.removeManualSupplier(supplierId)}
          onEditSupplier={(supplier) => {
            appState.selectedSupplierToEdit = supplier;
            appState.isPanelOpen = true;
            appState.rightPanelView = "manageSupplier";
          }}
        />
      {:else if currentView === "tablagastos"}
        <ExpensesTable
          selectedDate={appState.selectedDate}
          activeSpenderGroups={appState.activeSpenderGroups}
          allExpenses={appState.allExpenses}
          onCellClick={(dateStr, spender, category, existingExpense) => {
            appState.selectedExpenseCell = {
              dateStr,
              spender,
              category,
              existingExpense,
            };
            appState.isPanelOpen = true;
            appState.rightPanelView = "addExpense";
          }}
          onRemoveSpenderCategory={(spenderId, categoryId) =>
            appState.removeManualSpenderCategory(spenderId, categoryId)}
          onEditCategory={(category) => {
            appState.selectedCategoryToEdit = category;
            appState.isPanelOpen = true;
            appState.rightPanelView = "manageCategories";
          }}
        />
      {/if}
    </div>
  </section>

  <!-- COLUMNA DERECHA (Panel Lateral) -->
  <aside
    class="h-full bg-base-100 shadow-2xl transition-all duration-300 flex flex-col overflow-hidden {appState.isPanelOpen
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
          onclick={() => {
            appState.isPanelOpen = false;
            appState.rightPanelView = "calendar";
          }}
        >
          <X size={20} />
        </button>
      </div>

      <div
        class="p-4 space-y-4 overflow-y-auto overflow-x-hidden flex-1 bg-base-100"
      >
        {#if appState.rightPanelView === "calendar"}
          <DatePicker
            selectedDate={appState.selectedDate}
            onDateChange={(d) => (appState.selectedDate = d)}
          />
        {:else if appState.rightPanelView === "manageSupplier"}
          {#key appState.selectedSupplierToEdit?.id}
            <ManageSupplier
              initialEditSupplier={appState.selectedSupplierToEdit}
              onClose={() => {
                appState.rightPanelView = "calendar";
                appState.selectedSupplierToEdit = null;
              }}
              onSuccess={() => {
                appState.loadData();
                appState.rightPanelView = "calendar";
                appState.selectedSupplierToEdit = null;
              }}
            />
          {/key}
        {:else if appState.rightPanelView === "addSupplier"}
          <AddSupplier
            activeSuppliers={appState.activeSuppliers}
            onClose={() => (appState.rightPanelView = "calendar")}
            onAdd={(supplier) => appState.addManualSupplier(supplier)}
          />
        {:else if appState.rightPanelView === "addPurchase" && appState.selectedPurchaseCell}
          {#key appState.selectedPurchaseCell}
            <AddPurchase
              dateStr={appState.selectedPurchaseCell.dateStr}
              supplier={appState.selectedPurchaseCell.supplier}
              existingPurchase={appState.selectedPurchaseCell.existingPurchase}
              onClose={() => {
                appState.rightPanelView = "calendar";
                appState.selectedPurchaseCell = null;
              }}
              onSuccess={() => {
                appState.loadData();
              }}
            />
          {/key}
        {:else if appState.rightPanelView === "manageCategories"}
          <ManageCategories
            onClose={() => (appState.rightPanelView = "calendar")}
          />
        {:else if appState.rightPanelView === "manageSpender"}
          <ManageSpender
            onClose={() => (appState.rightPanelView = "calendar")}
          />
        {:else if appState.rightPanelView === "addSpenderCategory"}
          <AddSpenderCategories
            onClose={() => (appState.rightPanelView = "calendar")}
          />
        {:else if appState.rightPanelView === "addExpense" && appState.selectedExpenseCell}
          <AddExpense
            dateStr={appState.selectedExpenseCell.dateStr}
            spender={appState.selectedExpenseCell.spender}
            category={appState.selectedExpenseCell.category}
            existingExpense={appState.selectedExpenseCell.existingExpense}
            onClose={() => (appState.rightPanelView = "calendar")}
            onSuccess={() => appState.loadData()}
          />
        {/if}
      </div>
    </div>
  </aside>

  <!-- BURBUJA PARA ABRIR EL PANEL -->
  {#if !appState.isPanelOpen}
    <button
      class="absolute right-0 top-1/2 -translate-y-1/2 bg-primary text-primary-content hover:brightness-110 transition-all rounded-l-full w-10 h-16 shadow-lg z-50 flex items-center justify-start pl-1 cursor-pointer border-none"
      onclick={() => (appState.isPanelOpen = true)}
    >
      <ChevronLeft size={24} />
    </button>
  {/if}

  <!-- NOTIFICACIONES GLOBALES -->
  <ToastContainer />
  <ConfirmationModal />
</main>
