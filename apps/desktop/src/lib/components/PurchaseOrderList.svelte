<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { poList } from "$lib/state/purchaseOrderList.svelte";
  import {
    loadPoStatuses,
    statusName,
    statusVariantOf,
  } from "$lib/purchaseOrderStatus";
  import type { CompletePurchaseOrder } from "$lib/types/CompletePurchaseOrder";
  import type { Vendor } from "$lib/types/Vendor";

  let vendors = $state<Vendor[]>([]);
  const purchaseOrders = $derived(poList.purchaseOrders);
  let loading = $state(true);
  let error = $state<string | null>(null);

  onMount(async () => {
    try {
      [vendors, poList.purchaseOrders, poList.statuses] = await Promise.all([
        invoke<Vendor[]>("read_vendor"),
        invoke<CompletePurchaseOrder[]>("read_complete_purchase_order"),
        loadPoStatuses(),
      ]);
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  });

  const vendorName = (id: bigint | null) =>
    vendors.find((v) => v.id == id)?.name ?? "—";

  // Names of everything on the order: items first, then item boxes.
  const lineNames = (po: CompletePurchaseOrder) => [
    ...po.items.map((l) => l.item.name),
    ...po.itemBoxes.map((l) => l.itemBox.name),
  ];

  const visible = $derived(
    purchaseOrders.filter((complete) => {
      const po = complete.purchaseOrder;
      if (
        poList.activeFilter !== "All" &&
        Number(po.statusId) !== poList.activeFilter
      )
        return false;
      const q = poList.search.trim().toLowerCase();
      return (
        !q ||
        po.orderId.toLowerCase().includes(q) ||
        po.name.toLowerCase().includes(q) ||
        vendorName(po.vendorId).toLowerCase().includes(q) ||
        lineNames(complete).some((n) => n.toLowerCase().includes(q))
      );
    }),
  );
</script>

<div class="po-results data-table-container">
  <div class="table-scroll-wrap">
    <table class="data-table">
      <thead>
        <tr>
          <th>Order #</th>
          <th>Name</th>
          <th>Vendor</th>
          <th>Contents</th>
          <th>Ordered</th>
          <th>Arrival</th>
          <th>Status</th>
        </tr>
      </thead>

      <tbody>
        {#each visible as complete (complete.purchaseOrder.id)}
          {@const po = complete.purchaseOrder}
          {@const names = lineNames(complete)}
          {@const status = statusName(poList.statuses, po.statusId)}
          {@const variant = statusVariantOf(status)}
          <tr class={variant ? `row-status-${variant}` : ""}>
            <td>
              <a class="order-link" href="/purchase-orders/{po.id}"
                >{po.orderId}</a
              >
            </td>
            <td><span class="cell-primary-text">{po.name}</span></td>
            <td>{vendorName(po.vendorId)}</td>
            <td title={names.join("\n")}>
              {#if names.length === 0}
                <span class="cell-sub-text">—</span>
              {:else}
                {names[0]}
                {#if names.length > 1}
                  <span class="cell-sub-text">+{names.length - 1} more</span>
                {/if}
              {/if}
            </td>
            <td>{po.orderDate}</td>
            <td>
              {#if po.arrivalDate}
                {po.arrivalDate}
              {:else}
                <span class="cell-sub-text">—</span>
              {/if}
            </td>
            <td>
              {#if status}
                <span
                  class="status-pill {variant ? `status-pill-${variant}` : ''}"
                >
                  {status}
                </span>
              {:else}
                <span class="cell-sub-text">—</span>
              {/if}
            </td>
          </tr>
        {:else}
          <tr>
            <td colspan="7" class="cell-sub-text">
              {loading ? "Loading…" : (error ?? "No purchase orders found.")}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>

<style>
  /* Results list centered horizontally */
  .po-results {
    width: 100%;
    max-width: 1100px;
    margin: 24px auto;
    box-sizing: border-box;
  }

  /* Keep everything inside the horizontal space */
  .po-results {
    width: calc(100% - 32px);
  }
  .po-results :global(.table-scroll-wrap) {
    overflow-x: hidden;
  }
  .po-results :global(.data-table) {
    table-layout: fixed;
  }
  .po-results :global(.data-table th),
  .po-results :global(.data-table td) {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
