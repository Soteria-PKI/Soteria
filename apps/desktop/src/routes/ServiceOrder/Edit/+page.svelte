<script lang="ts">
  import { untrack } from "svelte";
  import { page } from "$app/state";
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/core";
  import type { ServiceOrder } from "$lib/types/ServiceOrder";
  import type { Item } from "$lib/types/Item";
  import type { Employee } from "$lib/types/Employee";
  import type { Customer } from "$lib/types/Customer";

  // /ServiceOrder/Edit?id=<id> edits an existing order;
  // /ServiceOrder/Edit (no id) creates a new one.

  // Editable copy of the order: the fields on the Figma create/edit screen.
  // Dates and notes are "" while empty so they can bind to inputs.
  type Draft = {
    itemId: number | null;
    dateCreated: string;
    dateDue: string;
    employeeId: number | null;
    customerId: number | null;
    notesInternal: string;
    notesReceipt: string;
  };

  const today = () => new Date().toISOString().slice(0, 10);

  const blankOrder = (): ServiceOrder => ({
    id: 0,
    dateCreated: today(),
    dateDue: null,
    employeeId: null,
    customerId: null,
    notesReceipt: null,
    notesInternal: null,
    itemId: null,
    labourId: null,
    assemblyId: null,
  });

  const toDraft = (o: ServiceOrder): Draft => ({
    itemId: o.itemId,
    dateCreated: o.dateCreated ?? "",
    dateDue: o.dateDue ?? "",
    employeeId: o.employeeId,
    customerId: o.customerId,
    notesInternal: o.notesInternal ?? "",
    notesReceipt: o.notesReceipt ?? "",
  });

  // Fields not on the form (labour, assembly) are kept as they were.
  const toServiceOrder = (base: ServiceOrder, d: Draft): ServiceOrder => ({
    ...base,
    itemId: d.itemId,
    dateCreated: d.dateCreated,
    dateDue: d.dateDue.trim() || null,
    employeeId: d.employeeId,
    customerId: d.customerId,
    notesInternal: d.notesInternal.trim() || null,
    notesReceipt: d.notesReceipt.trim() || null,
  });

  let order = $state<ServiceOrder | null>(null);
  let draft = $state<Draft | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let saving = $state(false);
  let saveError = $state<string | null>(null);
  let savedMessage = $state<string | null>(null);

  let items = $state<Item[]>([]);
  let employees = $state<Employee[]>([]);
  let customers = $state<Customer[]>([]);

  const idParam = $derived(page.url.searchParams.get("id"));
  const isNew = $derived(idParam === null);

  // Guards against a slow earlier response overwriting a newer one.
  let loadToken = 0;

  async function load(id: number) {
    const token = ++loadToken;
    loading = true;
    error = null;
    try {
      if (!Number.isInteger(id)) throw new Error("Invalid service order id");
      const found = await invoke<ServiceOrder | null>("read_one_service_order", {
        id,
      });
      if (!found) throw new Error(`Service order ${id} not found`);
      if (token !== loadToken) return;
      order = found;
      draft = toDraft(found);
    } catch (e) {
      if (token !== loadToken) return;
      order = draft = null;
      error = e instanceof Error ? e.message : String(e);
    } finally {
      if (token === loadToken) loading = false;
    }
  }

  function initNew() {
    ++loadToken; // drop any in-flight load
    order = blankOrder();
    draft = toDraft(order);
    error = null;
    loading = false;
  }

  // Re-runs only when the ?id changes. The body is untracked so the state it
  // writes can't re-trigger it and reset the form while the user is typing.
  $effect(() => {
    const id = idParam;
    untrack(() => {
      saveError = null;
      savedMessage = null;
      if (id === null) initNew();
      else load(Number(id));
    });
  });

  // Dropdown options.
  $effect(() => {
    Promise.all([
      invoke<Item[]>("read_item"),
      invoke<Employee[]>("read_employee"),
      invoke<Customer[]>("read_customer"),
    ])
      .then(([i, e, c]) => {
        items = i;
        employees = e;
        customers = c;
      })
      .catch((e) => (saveError = String(e)));
  });

  const dirty = $derived(
    !!order && !!draft && JSON.stringify(draft) !== JSON.stringify(toDraft(order)),
  );
  // date_created is the only NOT NULL column.
  const valid = $derived(!!draft && draft.dateCreated.trim() !== "");

  async function create(addAnother: boolean) {
    if (!order || !draft || !valid) return;
    saving = true;
    saveError = null;
    savedMessage = null;
    try {
      await invoke("create_service_order", {
        serviceorder: toServiceOrder(order, draft),
      });
      if (addAnother) {
        initNew();
        savedMessage = "Service order created. Ready for the next one.";
      } else {
        await goto("/ServiceOrder");
      }
    } catch (e) {
      saveError = e instanceof Error ? e.message : String(e);
    } finally {
      saving = false;
    }
  }

  async function save() {
    if (isNew) return create(false);
    if (!order || !draft || !dirty || !valid) return;
    saving = true;
    saveError = null;
    savedMessage = null;
    try {
      await invoke("update_service_order", {
        with: toServiceOrder(order, draft),
      });
      await load(order.id); // re-read so the page reflects what was saved
    } catch (e) {
      saveError = e instanceof Error ? e.message : String(e);
    } finally {
      saving = false;
    }
  }
</script>

<section class="so-screen">
  <div class="table-toolbar">
    <a class="btn btn-primary" href="/ServiceOrder">← Return to service orders</a>
  </div>

  <div class="so-detail">
    {#if loading && !order}
      <p class="cell-sub-text">Loading…</p>
    {:else if error}
      <p class="cell-sub-text">{error}</p>
    {:else if order && draft}
      <header class="so-detail__header">
        <h1>{isNew ? "New service order" : `Service order #${order.id}`}</h1>
      </header>

      <form
        onsubmit={(e) => {
          e.preventDefault();
          save();
        }}
      >
        <div class="data-table-container so-detail__card">
          <dl>
            <dt class="input-label">Item</dt>
            <dd>
              <select class="form-input" bind:value={draft.itemId}>
                <option value={null}>—</option>
                {#each items as i (i.id)}
                  <option value={i.id}>{i.name}{i.sku ? ` (${i.sku})` : ""}</option>
                {/each}
              </select>
            </dd>

            <dt class="input-label">Creation date</dt>
            <dd>
              <input
                class="form-input"
                class:field-missing={draft.dateCreated.trim() === ""}
                type="date"
                bind:value={draft.dateCreated}
                required
              />
            </dd>

            <dt class="input-label">Due date</dt>
            <dd>
              <input class="form-input" type="date" bind:value={draft.dateDue} />
            </dd>

            <dt class="input-label">Employee</dt>
            <dd>
              <select class="form-input" bind:value={draft.employeeId}>
                <option value={null}>—</option>
                {#each employees as e (e.id)}
                  <option value={e.id}>{e.name} ({e.employeeNumber})</option>
                {/each}
              </select>
            </dd>

            <dt class="input-label">Customer</dt>
            <dd>
              <select class="form-input" bind:value={draft.customerId}>
                <option value={null}>—</option>
                {#each customers as c (c.id)}
                  <option value={c.id}>{c.name}</option>
                {/each}
              </select>
            </dd>
          </dl>
        </div>

        <div class="data-table-container so-detail__card">
          <dl>
            <dt class="input-label">Internal notes</dt>
            <dd>
              <input
                class="form-input"
                placeholder="Only internal"
                bind:value={draft.notesInternal}
              />
            </dd>

            <dt class="input-label">Receipt notes</dt>
            <dd>
              <input
                class="form-input"
                placeholder="Appears on receipts"
                bind:value={draft.notesReceipt}
              />
            </dd>
          </dl>
        </div>

        {#if saveError || savedMessage || (isNew ? valid : dirty)}
          <div class="so-detail__actions">
            {#if saveError}
              <span class="cell-sub-text">{saveError}</span>
            {:else if savedMessage}
              <span class="cell-sub-text">{savedMessage}</span>
            {/if}
            {#if isNew}
              {#if valid}
                <button
                  type="button"
                  class="btn btn-secondary"
                  disabled={saving}
                  onclick={() => create(true)}
                >
                  Save & add another
                </button>
                <button type="submit" class="btn btn-primary" disabled={saving}>
                  {saving ? "Saving…" : "Create service order"}
                </button>
              {/if}
            {:else if dirty}
              <button
                type="button"
                class="btn btn-secondary"
                disabled={saving}
                onclick={() => (draft = toDraft(order!))}
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
    {/if}
  </div>
</section>

<style>
  /* Same screen and card layout as the purchase order detail page */
  .so-screen {
    width: 100%;
    min-height: 100vh;
    box-sizing: border-box;
    background: var(--surface-subtle, #f1f5f9);
    font-family: var(--font-sans, "DM Sans", sans-serif);
  }
  .so-detail {
    box-sizing: border-box;
    width: 100%;
    max-width: 720px;
    margin: 0 auto;
    padding: 24px 16px;
  }
  .so-detail__header {
    display: flex;
    align-items: center;
    gap: 12px;
  }
  .so-detail__card {
    padding: 8px 16px;
    margin-bottom: 16px;
  }
  .so-detail__actions {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 8px;
    padding: 0 0 8px;
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

  /* Mandatory field still empty */
  .field-missing,
  .field-missing:focus {
    border-color: #dc2626;
    background-color: #fef2f2;
    box-shadow: 0 0 0 2px #fee2e2;
  }
</style>
