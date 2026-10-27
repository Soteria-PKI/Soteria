<script lang="ts">
    import "$lib/styles/tokens.css";
    import "$lib/styles/badges_pills.css";
    import "$lib/styles/buttons_inputs.css";
    import "$lib/styles/data_table.css";
    import "$lib/styles/sidebar_nav.css";

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
    :global(body) {
        margin: 0;
        padding: 0;
    }
</style>
