<script lang="ts">
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
