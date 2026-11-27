import type { CompletePurchaseOrder } from "$lib/types/CompletePurchaseOrder";

// Shared between the purchase-orders layout (toolbar) and the list component.
export const poList = $state({
  search: "",
  activeFilter: "All",
  purchaseOrders: [] as CompletePurchaseOrder[],
});
