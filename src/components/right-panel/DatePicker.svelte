<script lang="ts">
  import { onMount } from "svelte";
  import flatpickr from "flatpickr";
  import "flatpickr/dist/flatpickr.min.css";
  import { Spanish } from "flatpickr/dist/l10n/es.js";

  let { selectedDate, onDateChange } = $props<{ selectedDate: Date, onDateChange: (d: Date) => void }>();

  let calendarContainer: HTMLDivElement;

  onMount(() => {
    const fp = flatpickr(calendarContainer, {
      inline: true,
      locale: Spanish,
      defaultDate: selectedDate,
      maxDate: "today", // Bloquea fechas futuras
      onChange: (selectedDates) => {
        if (selectedDates.length > 0) {
          onDateChange(selectedDates[0]);
        }
      },
      onMonthChange: (_, __, instance) => {
        onDateChange(new Date(instance.currentYear, instance.currentMonth, 1));
      },
      onYearChange: (_, __, instance) => {
        onDateChange(new Date(instance.currentYear, instance.currentMonth, 1));
      }
    });

    return () => {
      fp.destroy();
    };
  });
</script>

<div class="w-full flex flex-col">
  <h3 class="font-bold mb-4 text-sm uppercase opacity-70">Escoger Fecha</h3>
  <div class="flex justify-center w-full">
    <div bind:this={calendarContainer}></div>
  </div>
</div>

<style>
  /* Estilos para integrar mejor flatpickr con el tema de DaisyUI */
  :global(.flatpickr-calendar.inline) {
    box-shadow: none !important;
    border: 1px solid var(--fallback-b3,oklch(var(--b3)/1));
    border-radius: 0.5rem;
    width: 100%;
    margin-bottom: 0.5rem;
  }
</style>
