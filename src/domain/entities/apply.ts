export interface ApplySprayRequest {
  sprayId: string;
  vtfPath: string;
  vmtPath: string | null;
  destinationDir: string;
  createBackup: boolean;
  overwrite: boolean;
}

export interface ApplyResult {
  appliedFiles: string[];
  backupDir: string | null;
}
