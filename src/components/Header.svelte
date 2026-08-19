<script lang="ts">
  import { ArrowLeft } from "@lucide/svelte";

  let { canGoBack, onBack, title } = $props<{
    canGoBack: boolean;
    onBack: () => void;
    title: string;
  }>();
</script>

<header class="w-full h-16 bg-base-100 border-b border-base-300 flex items-center px-6 shrink-0 z-10 shadow-sm">
  {#if canGoBack}
    <button class="btn btn-ghost btn-sm mr-4" onclick={onBack}>
      <ArrowLeft size={18} />
      Volver
    </button>
  {/if}
  <h2 class="text-xl font-bold text-base-content">{title}</h2>
</header>
