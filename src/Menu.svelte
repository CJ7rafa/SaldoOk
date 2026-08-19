<script lang="ts">
  import { ChevronLeft, X } from "@lucide/svelte";
  import Header from "./components/Header.svelte";
  import PurchasesTable from "./views/PurchasesTable.svelte";
  import DatePicker from "./components/right-panel/DatePicker.svelte";

  // Estado del layout
  let isPanelOpen = $state(false);

  // Estado del enrutador inteligente (historial)
  let history = $state<string[]>(["menu"]);

  // Computed: vista actual
  let currentView = $derived(history[history.length - 1]);
  let canGoBack = $derived(history.length > 1);

  // Computed: título del Header basado en la vista actual
  let headerTitle = $derived(
    currentView === "menu" ? "Menú Principal" :
    currentView === "tablacompras" ? "Gestión de Compras" : 
    "SaldoOk"
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

<main class="flex h-screen w-full bg-base-200 overflow-hidden relative text-base-content">
  
  <!-- COLUMNA IZQUIERDA (Principal) -->
  <section class="h-full flex flex-col transition-all duration-300 bg-base-100 {isPanelOpen ? 'w-[75%]' : 'w-full'}">
    
    <!-- HEADER GLOBAL INTELIGENTE -->
    <Header 
      title={headerTitle} 
      {canGoBack} 
      onBack={goBack} 
    />

    <!-- CONTENIDO DINÁMICO (Scrollable) -->
    <div class="flex-1 overflow-auto p-8 bg-base-200">
      {#if currentView === "menu"}
        <div class="w-full max-w-7xl mx-auto">
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
              <button class="h-32 bg-base-100 hover:bg-base-300 shadow-sm border border-base-300 rounded-box flex items-center justify-center transition-colors">
                Módulo {i + 2}
              </button>
            {/each}
          </div>
        </div>
      {:else if currentView === "tablacompras"}
        <PurchasesTable />
      {/if}
    </div>

  </section>

  <!-- COLUMNA DERECHA (Panel Lateral) -->
  <aside class="h-full bg-base-100 shadow-2xl transition-all duration-300 flex flex-col overflow-hidden {isPanelOpen ? 'w-[25%] border-l border-base-300' : 'w-0 border-none'}">
    
    <div class="w-[25vw] h-full flex flex-col">
      <div class="p-4 flex justify-between items-center border-b border-base-300 sticky top-0 bg-base-100 z-10">
        <h2 class="font-bold text-lg">Panel de Acciones</h2>
        <button class="btn btn-ghost btn-sm btn-square" onclick={() => isPanelOpen = false}>
          <X size={20} />
        </button>
      </div>
      
      <div class="p-4 space-y-4 overflow-y-auto overflow-x-hidden flex-1 bg-base-100">
        <!-- DatePicker Calendar -->
        <DatePicker />

        <div class="divider"></div>

        <p class="text-sm opacity-70">Aquí cargaremos dinámicamente formularios como "Agregar Proveedor".</p>
        {#each Array(10) as _, i}
          <button class="w-full p-3 bg-base-200 hover:bg-base-300 transition-colors rounded-lg text-sm text-left truncate">
            Configuración {i + 1}
          </button>
        {/each}
      </div>
    </div>
  </aside>

  <!-- BURBUJA PARA ABRIR EL PANEL -->
  {#if !isPanelOpen}
    <button 
      class="absolute right-0 top-1/2 -translate-y-1/2 bg-primary text-primary-content hover:brightness-110 transition-all rounded-l-full w-10 h-16 shadow-lg z-50 flex items-center justify-start pl-1 cursor-pointer border-none"
      onclick={() => isPanelOpen = true}
    >
      <ChevronLeft size={24} />
    </button>
  {/if}

</main>
