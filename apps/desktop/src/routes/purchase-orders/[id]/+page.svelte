<script lang="ts">
  import { untrack } from "svelte";
  import { page } from "$app/state";
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/core";
  import {
    EDITABLE_STATUS,
    loadPoStatuses,
    statusName,
    statusVariantOf,
  } from "$lib/purchaseOrderStatus";
  import type { CompletePurchaseOrder } from "$lib/types/CompletePurchaseOrder";
  import type { PoStatus } from "$lib/types/PoStatus";
  import type { PurchaseOrder } from "$lib/types/PurchaseOrder";
  import type { Vendor } from "$lib/types/Vendor";
  import type { Item } from "$lib/types/Item";
  import type { ItemBox } from "$lib/types/ItemBox";

  // One item or item box on the order. `name` and `sku` are only for display.
  type LineKind = "item" | "itemBox";
  type Line = {
    kind: LineKind;
    id: number;
    name: string;
    sku: string | null;
    quantity: number | null; // null while the quantity input is empty
  };

  // Editable copy of the order. Ids are plain numbers at runtime (JSON),
  // even though the generated types say bigint.
  type Draft = {
    name: string;
    vendorId: number | null;
    orderDate: string;
    arrivalDate: string;
    statusId: number;
    lines: Line[];
  };

  let complete = $state<CompletePurchaseOrder | null>(null);
  const po = $derived(complete?.purchaseOrder ?? null);
  let vendor = $state<Vendor | null>(null);
  let statuses = $state<PoStatus[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);

  let draft = $state<Draft | null>(null);
  let vendors = $state<Vendor[]>([]);
  let items = $state<Item[]>([]);
  let itemBoxes = $state<ItemBox[]>([]);
  let saving = $state(false);
  let saveError = $state<string | null>(null);

  const num = (v: bigint | number | null | undefined) =>
    v === null || v === undefined ? null : Number(v);

  const lineKey = (kind: LineKind, id: number) => `${kind}:${id}`;

  // Items first, then item boxes, each in the order the backend sends
  // (sorted by name).
  const toLines = (c: CompletePurchaseOrder): Line[] => [
    ...c.items.map(({ item, quantity }) => ({
      kind: "item" as const,
      id: Number(item.id),
      name: item.name,
      sku: item.sku,
      quantity: Number(quantity),
    })),
    ...c.itemBoxes.map(({ itemBox, quantity }) => ({
      kind: "itemBox" as const,
      id: Number(itemBox.id),
      name: itemBox.name,
      sku: itemBox.sku,
      quantity: Number(quantity),
    })),
  ];

  const toDraft = (c: CompletePurchaseOrder): Draft => ({
    name: c.purchaseOrder.name ?? "",
    vendorId: num(c.purchaseOrder.vendorId),
    orderDate: c.purchaseOrder.orderDate ?? "",
    arrivalDate: c.purchaseOrder.arrivalDate ?? "",
    statusId: Number(c.purchaseOrder.statusId),
    lines: toLines(c),
  });

  // Guards against a slow earlier response overwriting a newer one.
  let loadToken = 0;

  async function load(id: number) {
    const token = ++loadToken;
    loading = true;
    error = null;
    try {
      if (!Number.isInteger(id)) throw new Error("Invalid purchase order id");

      const [order, s] = await Promise.all([
        invoke<CompletePurchaseOrder | null>(
          "read_one_complete_purchase_order",
          { id },
        ),
        loadPoStatuses(),
      ]);
      if (!order) throw new Error(`Purchase order ${id} not found`);

      const vendorId = order.purchaseOrder.vendorId;
      const v =
        vendorId === null
          ? null
          : await invoke<Vendor | null>("read_one_vendor", {
              id: Number(vendorId),
            });

      if (token !== loadToken) return;
      statuses = s;
      complete = order;
      vendor = v;
      draft = toDraft(order);
    } catch (e) {
      if (token !== loadToken) return;
      complete = vendor = draft = null;
      error = e instanceof Error ? e.message : String(e);
    } finally {
      if (token === loadToken) loading = false;
    }
  }

  // "/purchase-orders/temp-<uuid>" is an unsaved draft: nothing exists in the
  // database until the user clicks "Save New Purchase Order".
  const TEMP_PREFIX = "temp-";
  const isNew = $derived(page.params.id?.startsWith(TEMP_PREFIX) ?? false);

  // A new order starts with the editable ("Pending") status, so the statuses
  // must be read first to know its id.
  async function initNew() {
    const token = ++loadToken; // drop any in-flight load
    loading = true;
    error = null;
    try {
      const s = await loadPoStatuses();
      const pending = s.find((st) => st.name === EDITABLE_STATUS);
      if (!pending)
        throw new Error(`The status "${EDITABLE_STATUS}" does not exist`);

      if (token !== loadToken) return;
      const blank: CompletePurchaseOrder = {
        purchaseOrder: {
          id: 0n,
          orderId: "New purchase order",
          name: "",
          vendorId: null,
          orderDate: "",
          arrivalDate: null,
          statusId: pending.id,
        },
        items: [],
        itemBoxes: [],
      };
      statuses = s;
      complete = blank;
      vendor = null;
      draft = toDraft(blank);
    } catch (e) {
      if (token !== loadToken) return;
      complete = vendor = draft = null;
      error = e instanceof Error ? e.message : String(e);
    } finally {
      if (token === loadToken) loading = false;
    }
  }

  // Re-runs only when the :id route param changes. The body is untracked so
  // the state it reads and writes (complete, draft, …) can't re-trigger it
  // and reset the form while the user is typing.
  $effect(() => {
    const id = page.params.id;
    untrack(() => {
      saveError = null;
      if (id?.startsWith(TEMP_PREFIX)) initNew();
      else load(Number(id));
    });
  });

  // Name of the saved status (not the one picked in the form).
  const status = $derived(statusName(statuses, po?.statusId));
  const variant = $derived(statusVariantOf(status));
  const editable = $derived(status === EDITABLE_STATUS);

  // The dropdown options are only needed when the form is editable.
  let optionsLoaded = false;
  $effect(() => {
    if (!editable || optionsLoaded) return;
    optionsLoaded = true;
    Promise.all([
      invoke<Vendor[]>("read_vendor"),
      invoke<Item[]>("read_item"),
      invoke<ItemBox[]>("read_item_box"),
    ])
      .then(([v, i, b]) => {
        vendors = v;
        items = i;
        itemBoxes = b;
      })
      .catch((e) => {
        optionsLoaded = false;
        saveError = String(e);
      });
  });

  const quantityValid = (q: number | null) =>
    q !== null && Number.isInteger(q) && q >= 1;

  const dirty = $derived(
    !!complete &&
      !!draft &&
      JSON.stringify(draft) !== JSON.stringify(toDraft(complete)),
  );
  const valid = $derived(
    !!draft &&
      draft.name.trim() !== "" &&
      draft.orderDate.trim() !== "" &&
      draft.lines.every((l) => quantityValid(l.quantity)),
  );

  // ---- Adding and removing lines ----------------------------------------

  // Value of the "add" dropdown: "item:<id>" or "itemBox:<id>", or "".
  let newLine = $state("");
  let newLineQuantity = $state<number | null>(1);

  // Items and boxes already on the order can't be added a second time.
  const onOrder = $derived(
    new Set(draft?.lines.map((l) => lineKey(l.kind, l.id)) ?? []),
  );

  function addLine() {
    if (!draft || !newLine || !quantityValid(newLineQuantity)) return;
    const [kind, rawId] = newLine.split(":") as [LineKind, string];
    const id = Number(rawId);
    const source =
      kind === "item"
        ? items.find((i) => Number(i.id) === id)
        : itemBoxes.find((b) => Number(b.id) === id);
    if (!source || onOrder.has(newLine)) return;

    draft.lines.push({
      kind,
      id,
      name: source.name,
      sku: source.sku,
      quantity: newLineQuantity,
    });
    newLine = "";
    newLineQuantity = 1;
  }

  function removeLine(index: number) {
    draft?.lines.splice(index, 1);
  }

  // ---- Saving -------------------------------------------------------------

  // Body of create_complete_purchase_order / update_complete_purchase_order.
  const toInput = (d: Draft, id: number, orderId: string) => {
    const linesOf = (kind: LineKind) =>
      d.lines
        .filter((l) => l.kind === kind)
        .map((l) => ({ id: l.id, quantity: l.quantity }));
    return {
      purchaseOrder: {
        id,
        orderId,
        name: d.name.trim(),
        vendorId: d.vendorId,
        orderDate: d.orderDate,
        arrivalDate: d.arrivalDate.trim() || null,
        statusId: d.statusId,
      },
      items: linesOf("item"),
      itemBoxes: linesOf("itemBox"),
    };
  };

  // Next "PO-<year>-NNN" after the highest existing one for that year.
  async function nextOrderId(): Promise<string> {
    const year = new Date().getFullYear();
    const prefix = `PO-${year}-`;
    const existing = await invoke<PurchaseOrder[]>("read_purchase_order");
    const max = existing.reduce((m, o) => {
      if (!o.orderId.startsWith(prefix)) return m;
      const n = parseInt(o.orderId.slice(prefix.length), 10);
      return Number.isNaN(n) ? m : Math.max(m, n);
    }, 0);
    return `${prefix}${String(max + 1).padStart(3, "0")}`;
  }

  async function create() {
    if (!draft || !valid) return;
    saving = true;
    saveError = null;
    try {
      const orderId = await nextOrderId();
      const id = await invoke<number>("create_complete_purchase_order", {
        order: toInput(draft, 0, orderId),
      });
      await goto(`/purchase-orders/${id}`, { replaceState: true });
    } catch (e) {
      saveError = e instanceof Error ? e.message : String(e);
    } finally {
      saving = false;
    }
  }

  async function save() {
    if (isNew) return create();
    if (!po || !draft || !dirty || !valid) return;
    saving = true;
    saveError = null;
    try {
      await invoke("update_complete_purchase_order", {
        order: toInput(draft, Number(po.id), po.orderId),
      });
      await load(Number(po.id)); // re-read so the page reflects what was saved
    } catch (e) {
      saveError = e instanceof Error ? e.message : String(e);
    } finally {
      saving = false;
    }
  }
</script>

{#snippet linesTable(lines: Line[], canEdit: boolean)}
  <div class="po-lines">
    <h2 class="input-label po-lines__title">Contents</h2>
    <table class="data-table">
      <thead>
        <tr>
          <th class="po-lines__type">Type</th>
          <th>Name</th>
          <th class="po-lines__sku">SKU</th>
          <th class="po-lines__qty">Qty</th>
          {#if canEdit}<th class="po-lines__action"></th>{/if}
        </tr>
      </thead>
      <tbody>
        {#each lines as line, index (lineKey(line.kind, line.id))}
          <tr>
            <td class="cell-sub-text">
              {line.kind === "item" ? "Item" : "Box"}
            </td>
            <td><span class="cell-primary-text">{line.name}</span></td>
            <td class="cell-sub-text">{line.sku?.trim() || "—"}</td>
            <td class="po-lines__qty">
              {#if canEdit && draft}
                <input
                  class="form-input po-lines__qty-input"
                  class:field-missing={!quantityValid(line.quantity)}
                  type="number"
                  min="1"
                  step="1"
                  aria-label="Quantity of {line.name}"
                  bind:value={draft.lines[index].quantity}
                />
              {:else}
                {line.quantity}
              {/if}
            </td>
            {#if canEdit}
              <td class="po-lines__action">
                <button
                  type="button"
                  class="btn btn-secondary btn-sm"
                  onclick={() => removeLine(index)}
                >
                  Remove
                </button>
              </td>
            {/if}
          </tr>
        {:else}
          <tr>
            <td colspan={canEdit ? 5 : 4} class="cell-sub-text">
              No items on this order.
            </td>
          </tr>
        {/each}
      </tbody>
    </table>

    {#if canEdit}
      <div class="po-lines__add">
        <select
          class="form-input"
          aria-label="Item or box to add"
          bind:value={newLine}
        >
          <option value="">Add an item or box…</option>
          <optgroup label="Items">
            {#each items as i (i.id)}
              {@const key = lineKey("item", Number(i.id))}
              <option value={key} disabled={onOrder.has(key)}>
                {i.name}{i.sku ? ` (${i.sku})` : ""}
              </option>
            {/each}
          </optgroup>
          <optgroup label="Boxes">
            {#each itemBoxes as b (b.id)}
              {@const key = lineKey("itemBox", Number(b.id))}
              <option value={key} disabled={onOrder.has(key)}>
                {b.name}{b.sku ? ` (${b.sku})` : ""}
              </option>
            {/each}
          </optgroup>
        </select>
        <input
          class="form-input po-lines__qty-input"
          type="number"
          min="1"
          step="1"
          aria-label="Quantity to add"
          bind:value={newLineQuantity}
          onkeydown={(e) => {
            // Enter adds the line instead of submitting the whole form.
            if (e.key === "Enter") {
              e.preventDefault();
              addLine();
            }
          }}
        />
        <button
          type="button"
          class="btn btn-secondary"
          disabled={!newLine || !quantityValid(newLineQuantity)}
          onclick={addLine}
        >
          Add
        </button>
      </div>
    {/if}
  </div>
{/snippet}

{#snippet statusField()}
  {#if draft}
    <dt class="input-label">Status</dt>
    <dd>
      <select class="form-input" bind:value={draft.statusId}>
        {#each statuses as s (s.id)}
          <option value={Number(s.id)}>{s.name}</option>
        {/each}
      </select>
    </dd>
  {/if}
{/snippet}

{#snippet formActions()}
  {#if isNew ? valid || saveError : dirty || saveError}
    <div class="po-detail__actions">
      {#if saveError}
        <span class="cell-sub-text">{saveError}</span>
      {/if}
      {#if isNew}
        {#if valid}
          <button type="submit" class="btn btn-primary" disabled={saving}>
            {saving ? "Saving…" : "Save New Purchase Order"}
          </button>
        {/if}
      {:else if dirty}
        <button
          type="button"
          class="btn btn-secondary"
          disabled={saving}
          onclick={() => (draft = toDraft(complete!))}
        >
          Discard
        </button>
        <button type="submit" class="btn btn-primary" disabled={saving || !valid}>
          {saving ? "Saving…" : "Save"}
        </button>
      {/if}
    </div>
  {/if}
{/snippet}

<section class="po-detail">
  {#if loading && !complete}
    <p class="cell-sub-text">Loading…</p>
  {:else if error}
    <p class="cell-sub-text">{error}</p>
  {:else if complete && po && draft}
    <header class="po-detail__header">
      <h1>{po.orderId}</h1>
      {#if status}
        <span class="status-pill {variant ? `status-pill-${variant}` : ''}">
          {status}
        </span>
      {/if}
    </header>

    {#if editable}
      <form
        class="data-table-container po-detail__card"
        onsubmit={(e) => {
          e.preventDefault();
          save();
        }}
      >
        <dl>
          <dt class="input-label">Name</dt>
          <dd>
            <input
              class="form-input"
              class:field-missing={isNew && draft.name.trim() === ""}
              bind:value={draft.name}
              required
            />
          </dd>

          <dt class="input-label">Vendor</dt>
          <dd>
            <select class="form-input" bind:value={draft.vendorId}>
              <option value={null}>—</option>
              {#each vendors as v (v.id)}
                <option value={Number(v.id)}>{v.name}</option>
              {/each}
            </select>
          </dd>

          <dt class="input-label">Ordered</dt>
          <dd>
            <input
              class="form-input"
              class:field-missing={isNew && draft.orderDate.trim() === ""}
              type="date"
              bind:value={draft.orderDate}
              required
            />
          </dd>

          <dt class="input-label">Arrival</dt>
          <dd>
            <input
              class="form-input"
              type="date"
              bind:value={draft.arrivalDate}
            />
          </dd>

          {@render statusField()}
        </dl>

        {@render linesTable(draft.lines, true)}

        {@render formActions()}
      </form>
    {:else}
      <!-- Only the status can change once an order is no longer Pending. -->
      <form
        class="data-table-container po-detail__card"
        onsubmit={(e) => {
          e.preventDefault();
          save();
        }}
      >
        <dl>
          {#if po.name?.trim()}
            <dt class="input-label">Name</dt>
            <dd>{po.name}</dd>
          {/if}

          {#if vendor?.name?.trim()}
            <dt class="input-label">Vendor</dt>
            <dd>{vendor.name}</dd>
          {/if}

          {#if po.orderDate?.trim()}
            <dt class="input-label">Ordered</dt>
            <dd>{po.orderDate}</dd>
          {/if}

          {#if po.arrivalDate?.trim()}
            <dt class="input-label">Arrival</dt>
            <dd>{po.arrivalDate}</dd>
          {/if}

          {@render statusField()}
        </dl>

        {@render linesTable(toLines(complete), false)}

        {@render formActions()}
      </form>
    {/if}
  {/if}
</section>

<style>
  .po-detail {
    box-sizing: border-box;
    width: 100%;
    max-width: 720px;
    margin: 0 auto;
    padding: 24px 16px;
  }
  .po-detail__header {
    display: flex;
    align-items: center;
    gap: 12px;
  }
  .po-detail__card {
    padding: 8px 16px;
  }
  .po-detail__actions {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 8px;
    padding: 12px 0 8px;
  }
  dl {
    display: grid;
    grid-template-columns: max-content 1fr;
    align-items: center;
    gap: 12px 24px;
    margin: 0;
  }
  dd {
    margin: 0;
    min-width: 0;
    overflow-wrap: anywhere;
  }
  dd :global(.form-input) {
    width: 100%;
  }

  /* Item and box lines */
  .po-lines {
    margin-top: 20px;
  }
  .po-lines__title {
    margin: 0 0 8px;
  }
  .po-lines :global(.data-table) {
    width: 100%;
  }
  .po-lines__type {
    width: 56px;
  }
  .po-lines__sku {
    width: 120px;
  }
  .po-lines__qty {
    width: 88px;
    text-align: right;
  }
  .po-lines__action {
    width: 84px;
    text-align: right;
  }
  .po-lines__qty-input {
    width: 72px;
    box-sizing: border-box;
    text-align: right;
  }
  .po-lines__add {
    display: flex;
    gap: 8px;
    align-items: center;
    padding: 12px 0 4px;
  }
  .po-lines__add select {
    flex: 1 1 auto;
    min-width: 0;
  }

  /* Mandatory field still empty on a new purchase order */
  .field-missing,
  .field-missing:focus {
    border-color: #dc2626;
    background-color: #fef2f2;
    box-shadow: 0 0 0 2px #fee2e2;
  }
</style>
