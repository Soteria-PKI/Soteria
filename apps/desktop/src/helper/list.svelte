<script
  lang="ts"
  generics="I extends { id: number }, F extends { filteredCondition: (value: I, index: number, array: I[]) => value is I}"
>
  //Items is given an array of any type that contains an id
  // Filter is a nullable condition boolean : <List items = {}></List>
  // href (optional) builds the link each row navigates to when clicked
  let {
    items,
    filter,
    href,
  }: { items: I[]; filter: F[] | F; href?: (item: I) => string } = $props();

  let properties: string[] = $derived(
    items.length > 0 ? Object.keys(items[0]) : [],
  );
  let filteredItems: I[] = $derived.by(() => {
    const filters = Array.isArray(filter) ? filter : [filter];
    return filters.reduce<I[]>(
      (acc, f) => acc.filter(f.filteredCondition),
      items,
    );
  });
</script>

<main>
  <p class="">{properties.join(", ")}</p>

  <div class="">
    {#each filteredItems as item (item.id)}
      {#if href}
        <a class="info" href={href(item)}>
          {#each properties as property}
            {String(item[property as keyof typeof item] ?? "N/A")},
          {/each}
        </a>
      {:else}
        <div class="info">
          {#each properties as property}
            {String(item[property as keyof typeof item] ?? "N/A")},
          {/each}
        </div>
      {/if}
    {:else}
      <p>Nothing to see here!</p>
    {/each}
  </div>
</main>

<style>
  /* <a> is inline by default; make linked rows stack like the <div> rows */
  a.info {
    display: block;
    color: inherit;
    text-decoration: none;
  }
</style>
