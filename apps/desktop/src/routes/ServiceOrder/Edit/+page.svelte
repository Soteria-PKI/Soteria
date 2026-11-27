<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { ServiceOrder } from "$lib/types/ServiceOrder";

    async function addServiceOrder() {
        let test: ServiceOrder = {
            id: 0,
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

            await invoke("create_service_order",{serviceorder: {
                id:test.id,
                dateCreated: test.dateCreated,
                dateDue: test.dateDue,
                employeeId: test.employeeId,
                customerId: test.customerId,
                notesInternal: test.notesInternal,
                notesReceipt: test.notesReceipt,
                item_id: test.itemId,
                labour_id: test.labourId,
                assembly_id: test.assemblyId
            }});
        } catch (error) {
            console.error("Failed to load service orders:", error);
        }  
    }
    
    
</script>

<main>
    <button on:click={addServiceOrder}>Hi</button>
</main>