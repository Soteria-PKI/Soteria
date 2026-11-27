<script lang="ts">
    import List from "../../helper/list.svelte";
    import type { ServiceOrder } from "$lib/types/ServiceOrder";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import type { Serializable } from "child_process";

    let items: ServiceOrder[] = $state([]);
    let Myfilters = {
        filteredCondition(value: ServiceOrder, index: number, array: Serializable[]): value is ServiceOrder {
            return true;
        }
    }

    onMount(async () => {
        try {
            items = await invoke<ServiceOrder[]>("read_service_order");
        } catch (error) {
            console.error("Failed to load service orders:", error);
        }
    });

    let deleteError: string | null = $state(null);

    async function deleteOrder(order: ServiceOrder) {
        if (!confirm(`Delete service order #${order.id}? This can't be undone.`)) return;
        deleteError = null;
        try {
            await invoke("delete_service_order", { id: order.id });
            items = items.filter((o) => o.id !== order.id);
        } catch (error) {
            deleteError = `Couldn't delete service order #${order.id}: ${error}`;
        }
    }
</script>

<section class="so-screen">
    <div class="table-toolbar">
        {#if deleteError}
            <span class="cell-sub-text">{deleteError}</span>
        {/if}
        <a class="btn btn-primary" href="/ServiceOrder/Edit">New service order</a>
    </div>

    <List
        items={items}
        filter={Myfilters}
        href={(order) => `/ServiceOrder/Edit?id=${order.id}`}
        onDelete={deleteOrder}
    ></List>
</section>

<style>
    /* Matches the purchase orders screen */
    .so-screen {
        width: 100%;
        min-height: 100vh;
        box-sizing: border-box;
        background: var(--surface-subtle, #f1f5f9);
        font-family: var(--font-sans, "DM Sans", sans-serif);
    }
    .table-toolbar {
        justify-content: flex-end;
    }
</style>
