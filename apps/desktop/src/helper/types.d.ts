
interface ServiceOrder {
        id: number,
        date_created: Date,
        date_due?: Date,
        employee_id?: number,
        customer_id?: number,
        notes_receipt?: string,
        notes_internal?: string,
        item_id?: number,
        labour_id?: number,
        assembly_id?: number
    };
    //add additionnal types for more lists