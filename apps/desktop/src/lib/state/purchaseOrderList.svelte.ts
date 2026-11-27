import type { CompletePurchaseOrder } from "$lib/types/CompletePurchaseOrder";
import type { PoStatus } from "$lib/types/PoStatus";

// Shared between the purchase-orders layout (toolbar) and the list component.
export const poList = $state({
  search: "",
  // "All", or the id of the status to show.
  activeFilter: "All" as "All" | number,
  purchaseOrders: [] as CompletePurchaseOrder[],
  statuses: [] as PoStatus[],
});
