<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { poList } from "$lib/state/purchaseOrderList.svelte";
  import type { PurchaseOrder } from "$lib/types/PurchaseOrder";
  import type { Vendor } from "$lib/types/Vendor";

  let vendors = $state<Vendor[]>([]);
  const purchaseOrders = $derived(poList.purchaseOrders);
  let loading = $state(true);
  let error = $state<string | null>(null);

  onMount(async () => {
    try {
      [vendors, poList.purchaseOrders] = await Promise.all([
        invoke<Vendor[]>("read_vendor"),
        invoke<PurchaseOrder[]>("read_purchase_order"),
      ]);
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  });

  const vendorName = (id: bigint | null) =>
    vendors.find((v) => v.id == id)?.name ?? "—";

  // Map PO status -> shared status classes from lib/styles.
  // TODO: replace with the po_status table once it exists.
  const statusVariant: Record<string, string> = {
    Pending: "awaiting",
    "In Transit": "progress",
    Received: "completed",
  };
  const variantOf = (status: string | null) =>
    status ? statusVariant[status] : undefined;

  const visible = $derived(
    purchaseOrders.filter((po) => {
      if (poList.activeFilter !== "All" && po.status !== poList.activeFilter)
        return false;
      const q = poList.search.trim().toLowerCase();
      return (
        !q ||
        po.orderId.toLowerCase().includes(q) ||
        po.name.toLowerCase().includes(q) ||
        vendorName(po.vendorId).toLowerCase().includes(q)
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
          <th>Ordered</th>
          <th>Arrival</th>
          <th>Status</th>
        </tr>
      </thead>

      <tbody>
        {#each visible as po (po.id)}
          {@const variant = variantOf(po.status)}
          <tr class={variant ? `row-status-${variant}` : ""}>
            <td>
              <a class="order-link" href="/purchase-orders/{po.id}"
                >{po.orderId}</a
              >
            </td>
            <td><span class="cell-primary-text">{po.name}</span></td>
            <td>{vendorName(po.vendorId)}</td>
            <td>{po.orderDate}</td>
            <td>
              {#if po.arrivalDate}
                {po.arrivalDate}
              {:else}
                <span class="cell-sub-text">—</span>
              {/if}
            </td>
            <td>
              {#if po.status?.trim()}
                <span
                  class="status-pill {variant
                    ? `status-pill-${variant}`
                    : ''}"
                >
                  {po.status}
                </span>
              {:else}
                <span class="cell-sub-text">—</span>
              {/if}
            </td>
          </tr>
        {:else}
          <tr>
            <td colspan="6" class="cell-sub-text">
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
