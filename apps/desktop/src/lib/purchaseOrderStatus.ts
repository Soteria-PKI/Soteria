import { invoke } from "@tauri-apps/api/core";
import type { PoStatus } from "$lib/types/PoStatus";

// The valid statuses come from the po_status table. They don't change while
// the app runs, so they are read once and shared.
let statuses: Promise<PoStatus[]> | null = null;

export function loadPoStatuses(): Promise<PoStatus[]> {
  statuses ??= invoke<PoStatus[]>("read_po_status").catch((e) => {
    statuses = null; // try again on the next call
    throw e;
  });
  return statuses;
}

export const statusName = (
  list: PoStatus[],
  id: bigint | number | null | undefined,
) => (id == null ? undefined : list.find((s) => s.id == id)?.name);

// Map PO status name -> shared status classes from lib/styles. This is a
// display concern, so it stays in the frontend.
const statusVariant: Record<string, string> = {
  Pending: "awaiting",
  "In Transit": "progress",
  Received: "completed",
};

export const statusVariantOf = (status: string | null | undefined) =>
  status ? statusVariant[status] : undefined;

// Only Pending orders have editable fields and lines. The status itself can
// be changed whatever the current status is.
export const EDITABLE_STATUS = "Pending";
