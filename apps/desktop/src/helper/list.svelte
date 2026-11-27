<script
  lang="ts"
  generics="I extends { id: number }, F extends { filteredCondition: (value: I, index: number, array: I[]) => value is I}"
>
  //Items is given an array of any type that contains an id
  // Filter is a nullable condition boolean : <List items = {}></List>
  // href (optional) builds the link each row navigates to when clicked
  // onDelete (optional) adds a Delete button at the end of each row
  let {
    items,
    filter,
    href,
    onDelete,
  }: {
    items: I[];
    filter: F[] | F;
    href?: (item: I) => string;
    onDelete?: (item: I) => void;
  } = $props();

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

  const columnCount = $derived(
    Math.max(properties.length, 1) + (onDelete ? 1 : 0),
  );

  // "dateCreated" -> "date Created" (the table header CSS uppercases it)
  const label = (property: string) =>
    property.replace(/([a-z0-9])([A-Z])/g, "$1 $2");

  const valueOf = (item: I, property: string) =>
    item[property as keyof I] as unknown;
</script>

<div class="list-results data-table-container">
  <div class="table-scroll-wrap">
    <table class="data-table">
      {#if properties.length > 0}
        <thead>
          <tr>
            {#each properties as property}
              <th>{label(property)}</th>
            {/each}
            {#if onDelete}<th class="list-action"></th>{/if}
          </tr>
        </thead>
      {/if}

      <tbody>
        {#each filteredItems as item (item.id)}
          <tr>
            {#each properties as property, i}
              {@const value = valueOf(item, property)}
              <td>
                {#if value === null || value === undefined || value === ""}
                  <span class="cell-sub-text">—</span>
                {:else if i === 0 && href}
                  <a class="order-link" href={href(item)}>{String(value)}</a>
                {:else}
                  {String(value)}
                {/if}
              </td>
            {/each}
            {#if onDelete}
              <td class="list-action">
                <button
                  type="button"
                  class="btn btn-secondary btn-sm"
                  onclick={() => onDelete(item)}
                >
                  Delete
                </button>
              </td>
            {/if}
          </tr>
        {:else}
          <tr>
            <td colspan={columnCount} class="cell-sub-text">
              Nothing to see here!
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>

<style>
  /* Same layout as the purchase order list: centered, no horizontal scroll */
  .list-results {
    width: calc(100% - 32px);
    max-width: 1100px;
    margin: 24px auto;
    box-sizing: border-box;
  }
  .list-results :global(.table-scroll-wrap) {
    overflow-x: hidden;
  }
  .list-results :global(.data-table) {
    table-layout: fixed;
  }
  .list-results :global(.data-table th),
  .list-results :global(.data-table td) {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  /* Delete button column, same width as the PO page's Remove column */
  .list-action {
    width: 84px;
    text-align: right;
  }
</style>
