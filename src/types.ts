export type GraphicsSettings = {
  internalResolution: number; msaa: number; anisotropy: number;
  vertexRounding: boolean; showFps: boolean; borderlessFullscreen: boolean;
};
export type ControllerSettings = {
  mode: "real" | "emulated"; motionPlus: boolean; nunchuk: boolean;
  deviceLabel: string; sensitivity: number;
};
export type GeckoCode = { name: string; code: string; enabled: boolean };
export type ModEntry = {
  id: string; name: string; kind: "riivolution" | "resource_pack"; enabled: boolean;
  gamePrefix?: string | null; xmlPath?: string | null; version?: string | null;
};
export type Profile = {
  id: string; name: string; graphics: GraphicsSettings;
  controller: ControllerSettings; mods: ModEntry[]; geckoCodes: GeckoCode[];
};
export type AppConfig = {
  schemaVersion: number; dolphinPath?: string | null; gamePath?: string | null;
  gameId?: string | null; activeProfileId: string; profiles: Profile[];
};
export type DolphinInfo = { executable: string; version: string };
export type GameInfo = {
  path: string; extension: string; sizeBytes: number; gameId?: string | null;
  isWiiSportsResort?: boolean | null; note: string;
};
export type LaunchResult = { pid: number; userDataPath: string; arguments: string[] };
export type LaunchStatus = { running: boolean; pid?: number | null; exitCode?: number | null };
export type BackupInfo = { id: string; path: string };
