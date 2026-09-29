<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");


  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }

  async function test_select(event: Event){
    event.preventDefault();
    await invoke("test_select");
  }

  async function select_tax_class(event: Event){
    event.preventDefault();
    await invoke("command_select_tax_class");
  }

  async function select_items(event: Event){
    event.preventDefault();
    await invoke("command_select_all_items");
  }
  async function select_item1(event: Event){
    event.preventDefault();
    await invoke("command_select_item",{id:1});
  }

  async function insert_item(event: Event){
    event.preventDefault();
    await invoke("command_insert_item",{sku:"testSku",name:"testName",cost:9,price:10,quantity:2,discount_id:null,taxClassId:1,vendor_id:null});
  }
  async function update_item(event: Event){
    event.preventDefault();
    await invoke("command_update_item",{id:1,sku:"testSku",name:"testName",cost:9,price:10,quantity:2,discount_id:null,taxClassId:1,vendor_id:null});
  }
  async function delete_item(event: Event){
    event.preventDefault();
    await invoke("command_delete_item",{id:6});
  }
</script>

<main class="container">
  <h1>Welcome to Tauri + Svelte</h1>

  <div class="row">
    <a href="https://vite.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte-kit" alt="SvelteKit Logo" />
    </a>
  </div>
  <p>Click on the Tauri, Vite, and SvelteKit logos to learn more.</p>

  <form class="row" onsubmit={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>

  <p>{greetMsg}</p>

  <form class="row" onsubmit={test_select}>
    <button type="submit">test query</button>
  </form>

  <form class="row" onsubmit={select_tax_class}>
    <button type="submit">test query</button>
  </form>

  <form class="row" onsubmit={select_items}>
    <button type="submit">Select items</button>
  </form>
  <form class="row" onsubmit={select_item1}>
    <button type="submit">Select item (1)</button>
  </form>
  <form class="row" onsubmit={insert_item}>
    <button type="submit">Insert item</button>
  </form>
  <form class="row" onsubmit={update_item}>
    <button type="submit">Update item</button>
  </form>
  <form class="row" onsubmit={delete_item}>
    <button type="submit">Delete item</button>
  </form>
</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte-kit:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }

  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: 0.75s;
  }

  .logo.tauri:hover {
    filter: drop-shadow(0 0 2em #24c8db);
  }

  .row {
    display: flex;
    justify-content: center;
  }

  a {
    font-weight: 500;
    color: #646cff;
    text-decoration: inherit;
  }

  a:hover {
    color: #535bf2;
  }

  h1 {
    text-align: center;
  }

  input,
  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }
  button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  input,
  button {
    outline: none;
  }

  #greet-input {
    margin-right: 5px;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    a:hover {
      color: #24c8db;
    }

    input,
    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
    button:active {
      background-color: #0f0f0f69;
    }
  }
</style>
