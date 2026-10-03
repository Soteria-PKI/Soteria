<script lang="ts" context="module">
    import type {TaxClass} from "../../src-tauri/bindings/TaxClass";
    import { invoke } from "@tauri-apps/api/core";

    export async function create_tax_class(tax_class : String, amount : number) {
       return await invoke<TaxClass>("create_tax_class", {tax_class: tax_class, amount : amount} );
    }

    export async function select_all_tax_class(): Promise<TaxClass> {
        let result = await invoke<TaxClass>("select_all_tax_class");
        return result;
    }

    export async function select_tax_class(class_id: number): Promise<TaxClass> {
        return await invoke("select_tax_class", { id: class_id });
    }

    export async function update_tax_class(
        class_id: number,
        tax_class: String,
        amount: number,
    ) {
        let result = await invoke("update_tax_class", {
            id: class_id,
            tax_class: tax_class,
            amount: amount,
        });
        return result;
    }

    export async function delete_tax_class(class_id: number) {
        let result = await invoke("delete_tax_class", { id: class_id });
        return result;
    }
</script>
