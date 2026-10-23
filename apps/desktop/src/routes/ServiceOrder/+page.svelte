<script lang="ts">
    import List from "../../helper/list.svelte";
    import type { ServiceOrder } from "$lib/types/ServiceOrder";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let items: ServiceOrder[] = $state([]);

    onMount(async () => {
        try {
            items = await invoke<ServiceOrder[]>("read_service_order");
        } catch (error) {
            console.error("Failed to load service orders:", error);
        }
    });
</script>

<List items = {items}></List>
