/** A `.vtf` spray texture, optionally paired with a `.vmt` material. */
export interface Spray {
  id: string;
  name: string;
  vtfPath: string;
  vmtPath: string | null;
  sizeBytes: number;
  /** Last modification time as Unix seconds. */
  modifiedAt: number;
}
