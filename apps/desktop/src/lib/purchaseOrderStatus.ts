// Map PO status -> shared status classes from lib/styles.
// TODO: replace with the po_status table once it exists.
const statusVariant: Record<string, string> = {
  Pending: "awaiting",
  "In Transit": "progress",
  Received: "completed",
};

export const statusVariantOf = (status: string | null | undefined) =>
  status ? statusVariant[status] : undefined;

// Only Pending orders are editable; In Transit, Received and the rest are locked.
export const EDITABLE_STATUS = "Pending";
