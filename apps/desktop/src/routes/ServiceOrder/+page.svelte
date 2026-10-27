<script lang="ts">
    import List from "../../helper/list.svelte";
    import type { ServiceOrder } from "$lib/types/ServiceOrder";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import type { Serializable } from "child_process";

    let items: ServiceOrder[] = $state([]);
    let Myfilters = {
        filteredCondition(value: ServiceOrder, index: number, array: Serializable[]): value is ServiceOrder {
            return value.id == BigInt(1);
        }
    }

    onMount(async () => {
        try {
            items = await invoke<ServiceOrder[]>("read_service_order");
        } catch (error) {
            console.error("Failed to load service orders:", error);
        }
    });
</script>

<List items = {items} filter = {Myfilters}></List>
