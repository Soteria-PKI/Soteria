<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { ServiceOrder } from "$lib/types/ServiceOrder";

    async function addServiceOrder() {
        let test = {
            
            dateCreated: "die",
            dateDue:null,
            employeeId:null,
            customerId:null,
            notesInternal:"hi",
            notesReceipt: "hi2",
            itemId:null,
            labourId:null,
            assemblyId:null
        }
        
        try {
            
            await invoke<ServiceOrder>("create_service_order",test);
        } catch (error) {
            console.error("Failed to load service orders:", error);
        }  
    }
    
    
</script>

<main>
    <button on:click={addServiceOrder}>Hi</button>
</main>