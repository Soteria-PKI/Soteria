<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";


    async function selectFromTable<T>(table_name: string): Promise<T> {
        let query = "pub(crate) fn read_" + table_name.trim().toLowerCase() + "() -> Result<Vec<ServiceOrder>, String> {}";
        let result = await invoke<T>(query);
        console.log(result);
        return result;
    }

    let items : Object[] = [];
    let categories = new Set<string>()
    let values : string[][] = [];

    onMount( async () => {
        try {
            items = await selectFromTable("service_order")
        } catch (error) {
            console.log("Failed to load data")
        }
    });

    let count = 0
    for(const obj of items) {
        for(const [k, v] of Object.entries(obj)) {
            categories.add(k);
            values[count].push();
        }
        count++
    }
    
</script>

<main>
    <ul>
        {#each categories as category}
            <li>{category}</li>
        {/each}
        {#each values as item (categories.keys)}
            {item}
        {:else}
            <p>Nothing to see here!</p>
        {/each}
    </ul>
</main>