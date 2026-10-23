<script lang="ts" generics="T extends { id: bigint }">
  let { items }: { items: T[] } = $props();

    let properties: string[] = $derived(
        items.length > 0 ? Object.keys(items[0]) : []
    );
</script>

<main>
    <p>{properties.join(", ")}</p>
    {#each items as item (item.id)}
        <div>
            {#each properties as property}
                    {String(item[property as keyof typeof item] ?? "N/A")},
            {/each}
        </div>
    {:else}
        <p>Nothing to see here!</p>
    {/each}
</main>