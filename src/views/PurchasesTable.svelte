<script lang="ts">
  // Datos "quemados" (mock data) para depurar la interfaz
  const proveedores = ["Distribuidora XYZ", "Tech Supplies", "Alimentos S.A.", "Papelería Central", "Logística Express"];
  const dias = Array.from({ length: 31 }, (_, i) => `Agosto ${i + 1}`);
</script>

<div class="w-full h-full pb-8">
  <h1 class="text-3xl font-bold mb-6">Tabla de Compras</h1>
  
  <div class="overflow-x-auto bg-base-100 shadow-xl rounded-box border border-base-300 w-full h-full max-h-[70vh]">
    <!-- table-pin-rows fija los headers arriba, table-pin-cols fija la primera columna a la izquierda -->
    <table class="table table-zebra table-pin-rows table-pin-cols w-full">
      
      <!-- HEADER DOBLE FILA -->
      <thead>
        <!-- FILA 1: Celda combinada -->
        <tr class="bg-base-200">
          <!-- Esquina superior izquierda -->
          <th class="bg-base-300 border-r border-base-300 w-40 z-20">Fecha</th>
          
          <!-- Celda Combinada usando 'colspan' -->
          <th colspan={proveedores.length} class="text-center bg-base-200 text-lg uppercase tracking-wider font-bold border-b border-base-300">
            Proveedores
          </th>
        </tr>
        
        <!-- FILA 2: Lista de proveedores -->
        <tr class="bg-base-200 shadow-sm">
          <th class="bg-base-300 border-r border-base-300"></th> <!-- Espacio vacío bajo 'Fecha' -->
          
          {#each proveedores as proveedor}
            <th class="text-center min-w-[200px] border-r border-base-300 last:border-r-0">
              {proveedor}
            </th>
          {/each}
        </tr>
      </thead>
      
      <!-- CUERPO DE LA TABLA (Días del mes) -->
      <tbody>
        {#each dias as dia}
          <tr class="hover">
            <!-- Primera columna (Fecha - se queda fija al scrollear a la derecha) -->
            <th class="bg-base-200 border-r border-base-300 whitespace-nowrap">{dia}</th>
            
            <!-- Celdas de datos para cada proveedor -->
            {#each proveedores as _}
              <td class="text-center border-r border-base-300 last:border-r-0 cursor-pointer hover:bg-primary/10 transition-colors">
                <span class="opacity-40 text-sm italic">$ 0.00</span>
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
      
    </table>
  </div>
</div>
