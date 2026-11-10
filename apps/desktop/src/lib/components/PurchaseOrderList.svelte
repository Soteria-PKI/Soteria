<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import type { PurchaseOrder } from "$lib/types/PurchaseOrder";
    import type { Vendor } from "$lib/types/Vendor";

    let vendors = $state<Vendor[]>([]);
    let purchaseOrders = $state<PurchaseOrder[]>([]);
    let loading = $state(true);
    let error = $state<string | null>(null);

    onMount(async () => {
        try {
            [vendors, purchaseOrders] = await Promise.all([
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

    // Filter buttons are built from the statuses present in the data.
    const filters = $derived([
        "All",
        ...new Set(
            purchaseOrders
                .map((po) => po.status)
                .filter((s): s is string => !!s?.trim()),
        ),
    ]);
    let activeFilter = $state("All");
    let search = $state("");

    const countFor = (f: string) =>
        f === "All"
            ? purchaseOrders.length
            : purchaseOrders.filter((po) => po.status === f).length;

    const visible = $derived(
        purchaseOrders.filter((po) => {
            if (activeFilter !== "All" && po.status !== activeFilter)
                return false;
            const q = search.trim().toLowerCase();
            return (
                !q ||
                po.orderId.toLowerCase().includes(q) ||
                po.name.toLowerCase().includes(q) ||
                vendorName(po.vendorId).toLowerCase().includes(q)
            );
        }),
    );
</script>

<section class="po-screen">
    <div class="table-toolbar po-toolbar">
        <div class="search-input-wrapper">
            <input
                class="table-search-input"
                type="search"
                placeholder="Search orders, vendors…"
                bind:value={search}
            />
        </div>

        <div class="status-filter-bar">
            {#each filters as f}
                <button
                    type="button"
                    class="filter-btn"
                    class:active={activeFilter === f}
                    onclick={() => (activeFilter = f)}
                >
                    {f}
                    <span class="filter-count">{countFor(f)}</span>
                </button>
            {/each}
        </div>

        <button type="button" class="btn-primary-action">
            New purchase order
        </button>
    </div>

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
                            {loading
                                ? "Loading…"
                                : (error ?? "No purchase orders found.")}
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
    </div>
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
    }

    /* Toolbar stays pinned to the top of the screen */
    .po-toolbar {
        position: sticky;
        top: 0;
        z-index: 20;
    }

    /* Results list centered horizontally */
    .po-results {
        width: 100%;
        max-width: 1100px;
        margin: 24px auto;
        box-sizing: border-box;
    }

    /* Keep everything inside the horizontal space */
    .po-toolbar {
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
