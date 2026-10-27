<script lang="ts">
    import "$lib/styles/tokens.css";
    import "$lib/styles/badges_pills.css";
    import "$lib/styles/buttons_inputs.css";
    import "$lib/styles/data_table.css";
    import "$lib/styles/sidebar_nav.css";
    import "$lib/styles/tokens.css";

    import SideMenu from "$lib/components/SideMenu.svelte";

    let { children } = $props();
</script>

<main class="container">
    <SideMenu />
    <div class="page">
        {@render children()}
    </div>
</main>

<style>
    .container {
        display: flex;
        flex-direction: row;
        background-color: green;
        padding: 0;
        margin: 0;

    }
    .page {
        display: flex;
        flex-grow: 13;
        background-color: white;
    }
</style>
