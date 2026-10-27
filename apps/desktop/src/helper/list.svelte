<script
  lang="ts"
  generics="I extends { id: bigint }, F extends { filteredCondition: (value: I, index: number, array: I[]) => value is I}"
>
  //Items is given an array of any type that contains an id
  // Filter is a nullable condition boolean : <List items = {}></List>
  let { items, filter }: { items: I[]; filter: F[] | F } = $props();

  let properties: string[] = $derived(
    items.length > 0 ? Object.keys(items[0]) : [],
  );
  let filteredItems = $state(items);

  if (Array.isArray(filter)) {
    for (const f of filter) {
      filteredItems =
        filteredItems.filter(
          f.filteredCondition,
        );
    }
  } else {
    filteredItems =
      filteredItems.filter(
        filter.filteredCondition,
      );
  }
</script>

<main>
  <p class="">{properties.join(", ")}</p>

  <div class="">
    {#each filteredItems as item (item.id)}
      <div class="info">
        {#each properties as property}
          {String(item[property as keyof typeof item] ?? "N/A")},
        {/each}
      </div>
    {:else}
      <p>Nothing to see here!</p>
    {/each}
  </div>
</main>
