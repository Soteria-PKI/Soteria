<script lang="ts">
  import { page } from "$app/state";
  import { goto } from "$app/navigation";
  import { poList } from "$lib/state/purchaseOrderList.svelte";

  let { children } = $props();

  // The list route shows the search toolbar; any sub-route (an individual
  // purchase order) shows a return button instead.
  const onList = $derived(page.route.id === "/purchase-orders");

  // Filter buttons are built from the statuses present in the data.
  const filters = $derived([
    "All",
    ...new Set(
      poList.purchaseOrders
        .map((po) => po.status)
        .filter((s): s is string => !!s?.trim()),
    ),
  ]);

  const countFor = (f: string) =>
    f === "All"
      ? poList.purchaseOrders.length
      : poList.purchaseOrders.filter((po) => po.status === f).length;
</script>

<section class="po-screen">
  <div class="table-toolbar po-toolbar">
    {#if onList}
      <div class="search-input-wrapper">
        <input
          class="table-search-input"
          type="search"
          placeholder="Search orders, vendors…"
          bind:value={poList.search}
        />
      </div>

      <div class="status-filter-bar">
        {#each filters as f}
          <button
            type="button"
            class="filter-btn"
            class:active={poList.activeFilter === f}
            onclick={() => (poList.activeFilter = f)}
          >
            {f}
            <span class="filter-count">{countFor(f)}</span>
          </button>
        {/each}
      </div>

      <button
        type="button"
        class="btn-primary-action"
        onclick={() => goto(`/purchase-orders/temp-${crypto.randomUUID()}`)}
      >
        New purchase order
      </button>
    {:else}
      <a class="btn-primary-action po-back" href="/purchase-orders">
        ← Return to purchase orders
      </a>
    {/if}
  </div>

  {@render children()}
</section>

<style>
  /* Full-screen scroll container */
  .po-screen {
    width: 100%;
    height: 100vh;
    height: 100dvh;
    overflow-x: hidden;
    overflow-y: auto;
    box-sizing: border-box;
    background: var(--surface-subtle, #f1f5f9);
    font-family: var(--font-sans, "DM Sans", sans-serif);
  }

  /* Toolbar stays pinned to the top of the screen */
  .po-toolbar {
    position: sticky;
    top: 0;
    z-index: 20;
    box-sizing: border-box;
    max-width: 100%;
    flex-wrap: wrap;
  }
  .po-toolbar :global(.search-input-wrapper) {
    flex: 1 1 160px;
    min-width: 0;
  }
  .po-toolbar :global(.status-filter-bar) {
    flex: 2 1 240px;
    min-width: 0;
  }
  .po-toolbar :global(.filter-btn) {
    min-width: 0;
    padding: 0 6px;
  }
  .po-back {
    display: inline-flex;
    align-items: center;
    text-decoration: none;
  }
</style>
