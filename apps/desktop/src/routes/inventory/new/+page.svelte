<div class="page">
  <header class="topbar">
    <div class="crumbs">
      <span class="transparent">thingy</span>
      <span class="pale">thing</span>
      <span class="transparent">thinggg</span>
      <span class="pale">thing</span>
      <strong>thing thing</strong>
    </div>
    <div class="actions">
      <button class="btn grey" type="button">thing</button>
      <button class="btn primary" type="button">Save Changes</button>
    </div>
  </header>

  <main class="content">
    <input
      class="name-input"
      placeholder="Item name (required)"
      aria-label="Item name"
    />

    <div class="columns">
      <!-- Left column -->
      <div class="col">
        <button class="add-images" type="button">thing for images</button>

        <section class="panel">
          <h2>thing</h2>
          <div class="row">
            <span class="k">thing</span>
            <span class="v static">thing</span>
          </div>
          <div class="row">
            <label class="k" for="sku">thing</label>
            <input id="sku" class="v" placeholder="Auto-generated if empty" />
          </div>
        </section>

        <section class="panel">
          <h2>thing</h2>
          <div class="row">
            <label class="k" for="category">thing</label>
            <select id="category" class="v">
              <option value="">thing</option>
              <option value="1">thing</option>
            </select>
          </div>
          <div class="row">
            <label class="k" for="tag-input">thing</label>
            <div class="v tags">
              <span class="chip"
                >thing <button type="button" aria-label="Remove brake">×</button
                ></span
              >
              <span class="chip"
                >thing <button type="button" aria-label="Remove 2026">×</button
                ></span
              >
              <input id="tag-input" class="tag-input" placeholder="Tag(s)" />
            </div>
          </div>
        </section>
      </div>

      <!-- Right column -->
      <div class="col">
        <section class="panel">
          <h2>thing</h2>
          <div class="price-head">
            <span>thing</span>
            <span>thing</span>
            <span>thing</span>
            <span>thing</span>
          </div>
          <div class="price-row">
            <strong>thing</strong>
            <span class="money">
              <span class="cur">$</span>
              <input
                type="number"
                min="0"
                step="0.01"
                placeholder="0.00"
                aria-label="Default price"
              />
            </span>
            <span class="calc">—</span>
            <span class="calc">—</span>
          </div>

          <div class="row">
            <label class="k" for="discount">thing</label>
            <select id="discount" class="v">
              <option value="">No thing</option>
            </select>
          </div>
          <div class="row">
            <label class="k" for="tax">thing</label>
            <select id="tax" class="v">
              <option value="1">thing</option>
            </select>
          </div>
        </section>

        <section class="panel">
          <h2>thing</h2>
          <div class="row">
            <label class="k" for="cost">thing</label>
            <span class="v money">
              <span class="cur">$</span>
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
            <label class="k" for="vendor">thing</label>
            <select id="vendor" class="v">
              <option value="">truc</option>
              <option value="1">thing</option>
            </select>
          </div>
          <div class="row">
            <label class="k" for="qty">thing</label>
            <input
              id="qty"
              class="v"
              type="number"
              min="0"
              step="1"
              placeholder="0"
            />
          </div>
        </section>
      </div>
    </div>
  </main>
</div>

<style>
  :global(body) {
    margin: 0;
    font-family: "Helvetica Neue", Arial, system-ui, sans-serif;
    font-size: 13px;
    color: #222;
    background: #fff;
  }

  /* Space for sidebar. */
  .page {
    margin-left: 300px;
    margin-top: 50px;
  }

  /*  Top bar (temporary)  */
  .topbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 1.25rem;
    height: 56px;
    background: #f1f1f1;
    border-bottom: 1px solid #ddd;
    position: sticky;
    top: 0;
    z-index: 5;
  }

  .crumbs {
    font-size: 1.1rem;
  }

  .transparent {
    color: #777777e3;
  }

  .pale {
    color: #999999d3;
    margin: 0 0.4rem;
  }

  .actions {
    display: flex;
    gap: 0.5rem;
  }

  .btn {
    font: inherit;
    padding: 0.45rem 0.9rem;
    border: none;
    border-radius: 3px;
    cursor: pointer;
    color: #fff;
  }

  .btn.grey {
    background: #7a7a7a;
  }

  .btn.primary {
    background: #2d7ff9;
  }

  /*  Content  */
  .content {
    padding: 1.25rem;
    max-width: 1100px;
  }

  .name-input {
    width: 100%;
    box-sizing: border-box;
    font: inherit;
    font-size: 1.05rem;
    padding: 0.6rem 0.7rem;
    border: 1px solid #bbb;
    border-radius: 3px;
    margin-bottom: 1rem;
  }

  .columns {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
    align-items: start;
  }

  .col {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    min-width: 0;
  }

  .add-images {
    font: inherit;
    padding: 0.6rem;
    background: #ececec;
    border: 1px solid #ccc;
    border-radius: 3px;
    color: #555;
    cursor: not-allowed;
  }

  /*  Panels (grey header + rows)  */
  .panel {
    border: 1px solid #ccc;
    background: #fff;
  }

  .panel h2 {
    margin: 0;
    padding: 0.5rem 0.7rem;
    background: #ddd;
    font-size: 0.9rem;
    font-weight: 600;
    border-bottom: 1px solid #ccc;
  }

  .row {
    display: grid;
    grid-template-columns: 130px 1fr;
    align-items: stretch;
    border-bottom: 1px solid #e3e3e3;
  }

  .row:last-child {
    border-bottom: none;
  }

  .k {
    padding: 0.55rem 0.7rem;
    background: #f7f7f7;
    border-right: 1px solid #e3e3e3;
    display: flex;
    align-items: center;
  }

  .v {
    border: none;
    padding: 0.55rem 0.7rem;
    font: inherit;
    background: #fff;
    min-width: 0;
    box-sizing: border-box;
  }

  .v:focus {
    outline: 2px solid #2d7ff9;
    outline-offset: -2px;
  }

  .v.static {
    display: flex;
    align-items: center;
    color: #888;
    font-style: italic;
  }

  select.v {
    width: 100%;
  }

  /*  Tags  */
  .tags {
    display: flex;
    flex-wrap: wrap;
    gap: 0.3rem;
    align-items: center;
  }

  .chip {
    display: inline-flex;
    align-items: center;
    gap: 0.3rem;
    padding: 0.1rem 0.5rem;
    background: #e6eefb;
    border: 1px solid #c3d5f5;
    border-radius: 999px;
    font-size: 0.8rem;
  }

  .chip button {
    border: none;
    background: none;
    cursor: pointer;
    color: #557;
    padding: 0;
    font-size: 1rem;
    line-height: 1;
  }

  .tag-input {
    flex: 1;
    min-width: 6rem;
    border: none;
    font: inherit;
    outline: none;
    padding: 0.15rem 0;
  }

  /*  Pricing table  */
  .price-head,
  .price-row {
    display: grid;
    grid-template-columns: 1fr 1.2fr 0.8fr 0.8fr;
    align-items: center;
    padding: 0.4rem 0.7rem;
    gap: 0.5rem;
  }

  .price-head {
    font-size: 0.7rem;
    text-transform: uppercase;
    color: #777;
    border-bottom: 1px solid #e3e3e3;
  }

  .price-row {
    border-bottom: 1px solid #e3e3e3;
  }

  .calc {
    color: #777;
    text-align: right;
  }

  .money {
    display: flex;
    align-items: center;
    gap: 0.3rem;
  }

  .money .cur {
    color: #777;
  }

  .money input {
    width: 100%;
    font: inherit;
    text-align: right;
    border: 1px solid #ddd;
    border-radius: 3px;
    padding: 0.3rem 0.4rem;
    box-sizing: border-box;
  }

  .row .money {
    padding: 0.3rem 0.7rem;
  }

  .row .money input {
    max-width: 8rem;
  }
</style>
