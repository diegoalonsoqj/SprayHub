/** A supported Source game annotated with installation state. */
export interface GameInfo {
  id: string;
  name: string;
  appId: number;
  installed: boolean;
  installDir: string | null;
  spraysDir: string | null;
}
