import type { PurchaseOrder } from "$lib/types/PurchaseOrder";

// Shared between the purchase-orders layout (toolbar) and the list component.
export const poList = $state({
  search: "",
  activeFilter: "All",
  purchaseOrders: [] as PurchaseOrder[],
});
