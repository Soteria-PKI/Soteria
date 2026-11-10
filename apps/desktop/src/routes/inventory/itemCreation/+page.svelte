<script>
  let itemType = "single";
</script>

<svelte:head>
  <title>Inventory - New Item</title>
</svelte:head>

<div class="page">
  <main class="content">
    <form class="item-form">
      <div class="top-fields">
        <input
          class="description-input"
          placeholder="Description / Item name"
          aria-label="Item name"
          required
        />

        <label class="type-field">
          <span>Type</span>
          <select bind:value={itemType}>
            <option value="single">Single</option>
          </select>
        </label>
      </div>

      <div class="form-grid">
        <!-- Left column -->
        <div class="left-column">
          <section class="panel">
            <h2>Add Inventory</h2>

            <div class="row">
              <label for="quantity">Add Qty.</label>
              <input id="quantity" type="number" min="0" step="1" />
            </div>
          </section>

          <section class="panel">
            <h2>IDs</h2>

            <div class="system-id">
              <span>System ID</span>
              <span class="static">Generated when the item is saved</span>
            </div>

            <div class="row">
              <label for="sku">SKU</label>
              <input id="sku" placeholder="SKU (optional)" />
            </div>
          </section>
        </div>

        <!-- Middle column -->
        <div class="middle-column">
          <section class="panel">
            <h2>Pricing</h2>

            <div class="pricing-head">
              <span>NAME</span>
              <span>PRICE</span>
            </div>

            <div class="price-row">
              <strong>Default</strong>

              <span class="money">
                <span>$</span>
                <input
                  type="number"
                  min="0"
                  step="0.01"
                  placeholder="0.00"
                  aria-label="Selling price"
                />
              </span>
            </div>

            <div class="row compact">
              <label for="discount">Discount</label>
              <input
                id="discount"
                type="number"
                min="1"
                placeholder="Discount ID (optional)"
              />
            </div>

            <div class="row compact">
              <label for="tax-class">Tax Class</label>
              <input
                id="tax-class"
                type="number"
                min="1"
                placeholder="Tax class ID (optional)"
              />
            </div>
          </section>

          <section class="panel">
            <h2>Purchase Information</h2>

            <div class="row">
              <label for="cost">Cost</label>

              <span class="money padded">
                <span>$</span>
                <input
                  id="cost"
                  type="number"
                  min="0"
                  step="0.01"
                  placeholder="0.00"
                />
              </span>
            </div>

            <div class="row">
              <label for="vendor">Vendor ID</label>
              <input
                id="vendor"
                type="number"
                min="1"
                placeholder="Vendor ID (optional)"
              />
            </div>
          </section>
        </div>
      </div>

      <div class="form-actions">
        <button class="cancel-button" type="button">Cancel</button>
        <button class="save-button" type="button">Save Item</button>
      </div>
    </form>
  </main>
</div>

<style>
  :global(body) {
    margin: 0;
    font-family: "Helvetica Neue", Arial, system-ui, sans-serif;
    font-size: 15px;
    color: #222;
    background: #fff;
  }

  /* Space for sidebar and top bar */
  .page {
    margin-left: 300px;
    margin-top: 100px;
  }

  /* Main content */
  .content {
    width: 100%;
    max-width: 1600px;
    padding: 0 22px 32px;
    box-sizing: border-box;
  }

  .save-button {
    border: none;
    padding: 0 20px;
    color: #fff;
    background: #367fca;
    font: inherit;
    cursor: pointer;
  }

  /* Top fields */
  .top-fields {
    display: grid;
    grid-template-columns: 2fr auto;
    align-items: center;
    gap: 22px;
    margin-bottom: 14px;
  }

  .description-input {
    width: 100%;
    height: 42px;
    padding: 0 10px;
    border: 1px solid #aaa;
    font: inherit;
    font-size: 17px;
    box-sizing: border-box;
  }

  .type-field {
    display: flex;
    align-items: center;
    gap: 9px;
  }

  .type-field select {
    min-width: 145px;
    height: 32px;
    padding: 2px 7px;
    border: 1px solid #aaa;
    background: #fff;
    font: inherit;
  }

  /* Columns */
  .form-grid {
    display: grid;
    grid-template-columns: 2fr 1fr 0.85fr;
    align-items: start;
    gap: 24px;
  }

  .left-column,
  .middle-column,

  /* Panels */
  .panel {
    border: 1px solid #d2d2d2;
    background: #fff;
  }

  .panel h2 {
    margin: 0;
    padding: 10px;
    background: #ddd;
    border-bottom: 2px solid #888;
    font-size: 17px;
    font-weight: 600;
  }

  /* Rows */
  .row {
    display: grid;
    grid-template-columns: 22% 1fr;
    min-height: 43px;
    border-bottom: 1px solid #ddd;
  }

  .row:last-child {
    border-bottom: none;
  }

  .row label {
    display: flex;
    align-items: center;
    padding: 8px;
    background: #f7f7f7;
    border-right: 1px solid #ddd;
  }

  .row > input {
    width: 100%;
    min-width: 0;
    padding: 8px;
    border: none;
    font: inherit;
    box-sizing: border-box;
  }

  input:focus,
  select:focus {
    outline: 2px solid #78afe8;
    outline-offset: -2px;
  }

  /* System ID */
  .system-id {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    min-height: 42px;
    padding: 8px;
    border-bottom: 1px solid #ddd;
    background: #fafafa;
  }

  .static {
    color: #888;
    font-style: italic;
    text-align: right;
  }

  /* Pricing */
  .pricing-head,
  .price-row {
    display: grid;
    grid-template-columns: 1fr 1.2fr;
    align-items: center;
    gap: 10px;
    padding: 8px 10px;
    border-bottom: 1px solid #ddd;
  }

  .pricing-head {
    color: #999;
    font-size: 13px;
  }

  .price-row strong {
    color: #555;
    font-size: 17px;
  }

  .money {
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
  }

  .money span {
    color: #888;
  }

  .money input {
    width: 100%;
    min-width: 0;
    padding: 7px 5px;
    border: 1px solid #ddd;
    font: inherit;
    text-align: right;
    box-sizing: border-box;
  }

  .money.padded {
    padding: 0 8px;
  }

  .compact {
    grid-template-columns: 36% 1fr;
  }

  /* Buttons */
  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 18px;
  }

  .form-actions button {
    min-height: 38px;
    padding: 0 16px;
    border-radius: 3px;
    font: inherit;
    cursor: pointer;
  }

  .cancel-button {
    border: 1px solid #bbb;
    background: #fff;
    color: #555;
  }

  .save-button {
    border: 1px solid #367fca;
    font-size: 15px;
  }

  @media (max-width: 1150px) {
    .form-grid {
      grid-template-columns: 1fr 1fr;
    }
  }
</style>
