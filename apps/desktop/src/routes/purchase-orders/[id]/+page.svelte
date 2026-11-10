<script lang="ts">
  import { page } from "$app/state";
  import { invoke } from "@tauri-apps/api/core";
  import type { PurchaseOrder } from "$lib/types/PurchaseOrder";
  import type { Vendor } from "$lib/types/Vendor";
  import type { Item } from "$lib/types/Item";
  import type { ItemBox } from "$lib/types/ItemBox";

  // Editable copy of the order. Ids are plain numbers at runtime (JSON),
  // even though the generated types say bigint.
  type Draft = {
    name: string;
    vendorId: number | null;
    orderDate: string;
    arrivalDate: string;
    status: string;
    itemId: number | null;
    boxId: number | null;
  };

  let po = $state<PurchaseOrder | null>(null);
  let vendor = $state<Vendor | null>(null);
  let item = $state<Item | null>(null);
  let itemBox = $state<ItemBox | null>(null);
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

  const readOne = <T,>(command: string, id: bigint | number | null) =>
    id === null
      ? Promise.resolve(null)
      : invoke<T | null>(command, { id: Number(id) });

  const toDraft = (order: PurchaseOrder): Draft => ({
    name: order.name ?? "",
    vendorId: num(order.vendorId),
    orderDate: order.orderDate ?? "",
    arrivalDate: order.arrivalDate ?? "",
    status: order.status ?? "",
    itemId: num(order.itemId),
    boxId: num(order.boxId),
  });

  // Guards against a slow earlier response overwriting a newer one.
  let loadToken = 0;

  async function load(id: number) {
    const token = ++loadToken;
    loading = true;
    error = null;
    try {
      if (!Number.isInteger(id)) throw new Error("Invalid purchase order id");

      const order = await readOne<PurchaseOrder>("read_one_purchase_order", id);
      if (!order) throw new Error(`Purchase order ${id} not found`);

      const [v, i, b] = await Promise.all([
        readOne<Vendor>("read_one_vendor", order.vendorId),
        readOne<Item>("read_one_item", order.itemId),
        readOne<ItemBox>("read_one_item_box", order.boxId),
      ]);

      if (token !== loadToken) return;
      po = order;
      vendor = v;
      item = i;
      itemBox = b;
      draft = toDraft(order);
    } catch (e) {
      if (token !== loadToken) return;
      po = vendor = item = itemBox = draft = null;
      error = e instanceof Error ? e.message : String(e);
    } finally {
      if (token === loadToken) loading = false;
    }
  }

  // Re-runs whenever the :id route param changes.
  $effect(() => {
    saveError = null;
    load(Number(page.params.id));
  });

  // TODO: replace with the po_status table once it exists.
  const statusVariant: Record<string, string> = {
    Pending: "awaiting",
    "In Transit": "progress",
    Received: "completed",
  };
  const variant = $derived(po?.status ? statusVariant[po.status] : undefined);

  // Only Pending orders are editable; In Transit, Received and the rest are locked.
  const EDITABLE_STATUS = "Pending";
  const editable = $derived(po?.status === EDITABLE_STATUS);

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

  const dirty = $derived(
    !!po &&
      !!draft &&
      JSON.stringify(draft) !== JSON.stringify(toDraft(po)),
  );
  const valid = $derived(
    !!draft && draft.name.trim() !== "" && draft.orderDate.trim() !== "",
  );

  async function save() {
    if (!po || !draft || !dirty || !valid) return;
    saving = true;
    saveError = null;
    try {
      await invoke("update_purchase_order", {
        with: {
          id: Number(po.id),
          orderId: po.orderId,
          name: draft.name.trim(),
          vendorId: draft.vendorId,
          orderDate: draft.orderDate,
          arrivalDate: draft.arrivalDate.trim() || null,
          status: draft.status.trim() || null,
          itemId: draft.itemId,
          boxId: draft.boxId,
        },
      });
      await load(Number(po.id)); // re-read so the page reflects what was saved
    } catch (e) {
      saveError = e instanceof Error ? e.message : String(e);
    } finally {
      saving = false;
    }
  }
</script>

<section class="po-detail">
  <a class="order-link" href="/purchase-orders">← Back to purchase orders</a>

  {#if loading && !po}
    <p class="cell-sub-text">Loading…</p>
  {:else if error}
    <p class="cell-sub-text">{error}</p>
  {:else if po && draft}
    <header class="po-detail__header">
      <h1>{po.orderId}</h1>
      {#if po.status?.trim()}
        <span class="status-pill {variant ? `status-pill-${variant}` : ''}">
          {po.status}
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
            <input class="form-input" bind:value={draft.name} required />
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

          <dt class="input-label">Status</dt>
          <dd>
            <input class="form-input" bind:value={draft.status} />
          </dd>

          <dt class="input-label">Item</dt>
          <dd>
            <select class="form-input" bind:value={draft.itemId}>
              <option value={null}>—</option>
              {#each items as i (i.id)}
                <option value={Number(i.id)}>{i.name}</option>
              {/each}
            </select>
          </dd>

          <dt class="input-label">Item box</dt>
          <dd>
            <select class="form-input" bind:value={draft.boxId}>
              <option value={null}>—</option>
              {#each itemBoxes as b (b.id)}
                <option value={Number(b.id)}>{b.name}</option>
              {/each}
            </select>
          </dd>
        </dl>

        {#if dirty || saveError}
          <div class="po-detail__actions">
            {#if saveError}
              <span class="cell-sub-text">{saveError}</span>
            {/if}
            {#if dirty}
              <button
                type="button"
                class="btn btn-secondary"
                disabled={saving}
                onclick={() => (draft = toDraft(po!))}
              >
                Discard
              </button>
              <button
                type="submit"
                class="btn btn-primary"
                disabled={saving || !valid}
              >
                {saving ? "Saving…" : "Save"}
              </button>
            {/if}
          </div>
        {/if}
      </form>
    {:else}
      <div class="data-table-container po-detail__card">
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

          {#if item?.name?.trim()}
            <dt class="input-label">Item</dt>
            <dd>
              {item.name}
              {#if item.sku?.trim()}
                <span class="cell-sub-text">({item.sku})</span>
              {/if}
            </dd>
          {/if}

          {#if itemBox?.name?.trim()}
            <dt class="input-label">Item box</dt>
            <dd>
              {itemBox.name}
              {#if itemBox.sku?.trim()}
                <span class="cell-sub-text">({itemBox.sku})</span>
              {/if}
            </dd>
          {/if}
        </dl>
      </div>
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
</style>
