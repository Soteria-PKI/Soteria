<script lang="ts" context="module">
    import { invoke } from "@tauri-apps/api/core";

    export async function create_tax_class(event: Event, class_id: number) {}

    export async function select_all_tax_class(event: Event) {
        event.preventDefault();
        let result = await invoke("select_all_tax_class");
        console.log(result);
    }

    export async function select_tax_class(event: Event, class_id: number) {
        event.preventDefault();
        let result = await invoke("select_tax_class", { id : class_id });
        console.log(result);
    }
    export async function update_tax_class(event: Event, class_id: number) {}
    export async function delete_tax_class(event: Event, class_id: number) {}
</script>
