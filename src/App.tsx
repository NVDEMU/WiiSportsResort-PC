import { useEffect, useMemo, useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import {
  Activity, Archive, ArrowDownToLine, ArrowUpRight, Check, CloudSun, Disc3,
  FolderOpen, Gamepad2, HardDrive, Image, Info, Joystick, Layers3, Monitor,
  PackagePlus, Play, RefreshCw, Save, Settings2, ShieldCheck, Sparkles, Trash2,
  X, WandSparkles
} from "lucide-react";
import type {
  AppConfig, BackupInfo, ControllerSettings, DolphinInfo, GameInfo, GeckoCode,
  GraphicsSettings, LaunchResult, LaunchStatus, ModEntry, Profile
} from "./types";

type Page = "home" | "mods" | "graphics" | "controls" | "saves" | "settings";
const nav: { id: Page; label: string; icon: typeof Activity }[] = [
  { id:"home",label:"Resort",icon:CloudSun },
  { id:"mods",label:"Mods & textures",icon:Layers3 },
  { id:"graphics",label:"Graphics",icon:Monitor },
  { id:"controls",label:"Controls",icon:Gamepad2 },
  { id:"saves",label:"Saves & backups",icon:Save },
  { id:"settings",label:"Settings",icon:Settings2 }
];

function newProfile(name="Default"): Profile {
  return {
    id:crypto.randomUUID(),name,
    graphics:{internalResolution:3,msaa:1,anisotropy:4,vertexRounding:false,showFps:false,borderlessFullscreen:false},
    controller:{mode:"real",motionPlus:true,nunchuk:false,deviceLabel:"",sensitivity:1},
    mods:[],geckoCodes:[]
  };
}
function initialConfig(): AppConfig {
  const profile=newProfile();
  return {schemaVersion:1,dolphinPath:null,gamePath:null,gameId:null,activeProfileId:profile.id,profiles:[profile]};
}
function bytes(value:number):string {
  if(value<1024*1024)return Math.round(value/1024)+" KB";
  if(value<1024*1024*1024)return (value/1024/1024).toFixed(1)+" MB";
  return (value/1024/1024/1024).toFixed(2)+" GB";
}

export default function App() {
  const [page,setPage]=useState<Page>("home");
  const [config,setConfig]=useState<AppConfig>(initialConfig);
  const [dolphin,setDolphin]=useState<DolphinInfo|null>(null);
  const [game,setGame]=useState<GameInfo|null>(null);
  const [running,setRunning]=useState<LaunchStatus>({running:false});
  const [busy,setBusy]=useState(false);
  const [notice,setNotice]=useState<{kind:"ok"|"error"|"info";message:string}|null>(null);
  const [backups,setBackups]=useState<BackupInfo[]>([]);
  const [codeName,setCodeName]=useState("");
  const [codeText,setCodeText]=useState("");
  const active=useMemo(()=>config.profiles.find(p=>p.id===config.activeProfileId)??config.profiles[0],[config]);

  const tell=(message:string,kind:"ok"|"error"|"info"="ok")=>setNotice({kind,message});
  const persist=async(next:AppConfig)=>{
    setConfig(next);
    await invoke("save_config",{config:next});
  };
  const reload=async()=>{
    const next=await invoke<AppConfig>("load_config");
    setConfig(next);
  };
  const saveProfile=async(profile:Profile)=>{
    await persist({...config,profiles:config.profiles.map(p=>p.id===profile.id?profile:p)});
  };

  useEffect(()=>{
    invoke<AppConfig>("load_config").then(async value=>{
      setConfig(value);
      if(value.dolphinPath){
        try{setDolphin(await invoke<DolphinInfo>("inspect_dolphin",{path:value.dolphinPath}));}catch{setDolphin(null);}
      }
      if(value.gamePath){
        try{setGame(await invoke<GameInfo>("inspect_game",{path:value.gamePath}));}catch{setGame(null);}
      }
    }).catch(error=>tell(String(error),"error"));
  },[]);
  useEffect(()=>{
    const timer=window.setInterval(()=>invoke<LaunchStatus>("poll_launch_status").then(setRunning).catch(()=>undefined),1500);
    return ()=>window.clearInterval(timer);
  },[]);

  const chooseDolphin=async()=>{
    const path=await open({multiple:false,title:"Select Dolphin executable"});
    if(!path||typeof path!=="string")return;
    setBusy(true);
    try{
      const info=await invoke<DolphinInfo>("inspect_dolphin",{path});
      setDolphin(info);await persist({...config,dolphinPath:info.executable});tell("Dolphin detected: "+info.version);
    }catch(error){tell(String(error),"error");}finally{setBusy(false);}
  };
  const chooseGame=async()=>{
    const path=await open({multiple:false,title:"Select your Wii Sports Resort disc image",filters:[{name:"Wii disc image",extensions:["iso","rvz","wbfs","wia","gcz","gcm","ciso"]}]});
    if(!path||typeof path!=="string")return;
    setBusy(true);
    try{
      const info=await invoke<GameInfo>("inspect_game",{path});
      setGame(info);await persist({...config,gamePath:info.path,gameId:info.gameId??null});
      tell(info.note,info.isWiiSportsResort?"ok":"info");
    }catch(error){tell(String(error),"error");}finally{setBusy(false);}
  };
  const play=async()=>{
    if(!config.dolphinPath||!config.gamePath||!active){setPage("settings");tell("Select Dolphin and a game image first.","error");return;}
    setBusy(true);
    try{
      const result=await invoke<LaunchResult>("launch_game",{dolphinPath:config.dolphinPath,gamePath:config.gamePath,profileId:active.id});
      setRunning({running:true,pid:result.pid});tell("Dolphin started with the active profile.");
    }catch(error){tell(String(error),"error");}finally{setBusy(false);}
  };
  const openDolphin=async()=>{
    if(!active||!config.dolphinPath){tell("Select Dolphin and a profile first.","error");return;}
    try{
      await invoke("launch_dolphin_ui",{dolphinPath:config.dolphinPath,profileId:active.id});
      tell("Dolphin opened. Use its game list to access Riivolution patches or controller settings.");
    }catch(error){tell(String(error),"error");}
  };
  const addProfile=async()=>{
    const profile=newProfile("Profile "+(config.profiles.length+1));
    await persist({...config,profiles:[...config.profiles,profile],activeProfileId:profile.id});
    tell("New profile created.");
  };
  const addMod=async(kind:"riivolution"|"resource_pack")=>{
    if(!active)return;
    const path=await open({multiple:false,title:kind==="riivolution"?"Import Riivolution ZIP":"Import Dolphin Resource Pack",filters:[{name:"ZIP archive",extensions:["zip"]}]});
    if(!path||typeof path!=="string")return;
    setBusy(true);
    try{
      const item=await invoke<ModEntry>("import_mod_archive",{profileId:active.id,archivePath:path,kind});
      await reload();tell("Imported "+item.name+".");
    }catch(error){tell(String(error),"error");}finally{setBusy(false);}
  };
  const toggleMod=async(item:ModEntry)=>{
    if(!active)return;
    try{await invoke("set_mod_enabled",{profileId:active.id,modId:item.id,enabled:!item.enabled});await reload();tell((item.enabled?"Disabled ":"Enabled ")+item.name);}
    catch(error){tell(String(error),"error");}
  };
  const deleteMod=async(item:ModEntry)=>{
    if(!active||!window.confirm("Remove the managed mod folder for "+item.name+"?"))return;
    try{await invoke("remove_mod",{profileId:active.id,modId:item.id});await reload();tell("Removed "+item.name);}
    catch(error){tell(String(error),"error");}
  };
  const saveCode=async()=>{
    if(!active||!config.gameId){tell("A six-character game ID is required. Check Settings, especially for compressed images.","error");return;}
    if(!codeName.trim()||!codeText.trim()){tell("Enter the code name and code lines.","error");return;}
    const codes:GeckoCode[]=[...active.geckoCodes,{name:codeName.trim(),code:codeText.trim(),enabled:true}];
    try{
      await invoke("save_gecko_codes",{profileId:active.id,gameId:config.gameId,codes});
      await saveProfile({...active,geckoCodes:codes});setCodeName("");setCodeText("");tell("Gecko code written to Dolphin GameSettings.");
    }catch(error){tell(String(error),"error");}
  };
  const backup=async()=>{
    if(!active)return;setBusy(true);
    try{const entry=await invoke<BackupInfo>("backup_profile_data",{profileId:active.id});await refreshBackups();tell("Backup created: "+entry.id);}
    catch(error){tell(String(error),"error");}finally{setBusy(false);}
  };
  const refreshBackups=async()=>{
    if(!active)return;
    try{setBackups(await invoke<BackupInfo[]>("list_backups",{profileId:active.id}));}
    catch(error){tell(String(error),"error");}
  };
  const restore=async(id:string)=>{
    if(!active||!window.confirm("Restore this backup? The current saves will be backed up first."))return;
    setBusy(true);
    try{await invoke("restore_backup",{profileId:active.id,backupId:id});tell("Backup restored. Previous data was backed up first.");}
    catch(error){tell(String(error),"error");}finally{setBusy(false);}
  };
  const openProfileFolder=async()=>{
    if(!active)return;
    try{const path=await invoke<string>("get_profile_data_path",{profileId:active.id});await invoke("open_folder",{path});}
    catch(error){tell(String(error),"error");}
  };
  const updateGraphics=(value:GraphicsSettings)=>active&&saveProfile({...active,graphics:value});
  const updateController=(value:ControllerSettings)=>active&&saveProfile({...active,controller:value});
  const updateGameId=(value:string)=>persist({...config,gameId:value.toUpperCase().slice(0,6)});

  return <div className="shell">
    <aside className="sidebar">
      <div className="brand"><div className="brand-mark">W</div><div><b>RESORT</b><small>ENHANCED</small></div></div>
      <div className="nav-label">YOUR ISLAND</div>
      <nav>{nav.map(item=>{const Icon=item.icon;return <button key={item.id} className={"nav-item "+(page===item.id?"selected":"")} onClick={()=>setPage(item.id)}><Icon size={17}/>{item.label}</button>;})}</nav>
      <div className="sidebar-bottom"><span className={"running "+(running.running?"is-live":"")}><i/>{running.running?"GAME RUNNING":"READY TO PLAY"}</span><small>Powered by Dolphin</small><small className="version">PRE-RELEASE · 0.1.0</small></div>
    </aside>

    <main className="main">
      <header className="topbar">
        <div className="crumb">WII SPORTS RESORT <span>/</span> {nav.find(item=>item.id===page)?.label.toUpperCase()}</div>
        <div className="top-right"><label>PROFILE</label><select value={active?.id??""} onChange={e=>persist({...config,activeProfileId:e.target.value})}>{config.profiles.map(profile=><option key={profile.id} value={profile.id}>{profile.name}</option>)}</select><button className="icon-button" onClick={()=>invoke<LaunchStatus>("poll_launch_status").then(setRunning)} title="Refresh process status"><RefreshCw size={15}/></button></div>
      </header>
      {notice&&<div className={"notice "+notice.kind}><span>{notice.kind==="error"?"!":"✓"}</span><div>{notice.message}</div><button onClick={()=>setNotice(null)}><X size={14}/></button></div>}
      <div className="content">
        {page==="home"&&<section>
          <div className="hero">
            <div className="hero-copy"><span className="eyebrow"><Sparkles size={13}/> YOUR RESORT, REIMAGINED</span><h1>WELCOME BACK<br/><em>TO WUHU ISLAND.</em></h1><p>A dedicated Wii Sports Resort experience with enhanced rendering, controller profiles, mod support and more.</p><div className="hero-actions"><button className="primary" disabled={busy||running.running} onClick={play}><Play size={16} fill="currentColor"/>{running.running?"GAME RUNNING":"PLAY RESORT"}</button><button className="ghost" onClick={()=>setPage("mods")}><WandSparkles size={15}/> EXPLORE MODS</button></div></div>
            <div className="island-art"><div className="sun"/><div className="mountain back"/><div className="mountain front"/><div className="sea"/><div className="palm p1"/><div className="palm p2"/><span>ISLAND LIFE, UPGRADED</span></div>
          </div>
          <div className="section-title"><div><small>QUICK ACCESS</small><h2>Your setup</h2></div><button className="text-link" onClick={()=>setPage("settings")}>Manage setup <ArrowUpRight size={14}/></button></div>
          <div className="quick-grid">
            <div className="quick-card"><Disc3/><div><small>GAME IMAGE</small><b>{game?"Image selected":"Not configured"}</b><span>{game?(game.gameId??game.extension.toUpperCase()+" · "+bytes(game.sizeBytes)):"Choose your own disc image"}</span></div><button onClick={chooseGame}><FolderOpen size={15}/></button></div>
            <div className="quick-card"><Activity/><div><small>EMULATOR</small><b>{dolphin?"Dolphin ready":"Not configured"}</b><span>{dolphin?dolphin.version:"Choose a Dolphin installation"}</span></div><button onClick={chooseDolphin}><FolderOpen size={15}/></button></div>
            <div className="quick-card"><Layers3/><div><small>ACTIVE MODS</small><b>{active?.mods.filter(m=>m.enabled).length??0} enabled</b><span>{active?.mods.length??0} installed in this profile</span></div><button onClick={()=>setPage("mods")}><ArrowUpRight size={15}/></button></div>
            <div className="quick-card"><Gamepad2/><div><small>CONTROLS</small><b>{active?.controller.mode==="real"?"Real Wii Remote":"Emulated Remote"}</b><span>{active?.controller.motionPlus?"MotionPlus preferred":"MotionPlus not selected"}</span></div><button onClick={()=>setPage("controls")}><ArrowUpRight size={15}/></button></div>
          </div>
          <div className="sports-strip"><div><small>12 WAYS TO PLAY</small><b>Find your favourite sport.</b></div><span>ARCHERY</span><span>BOWLING</span><span>CYCLING</span><span>GOLF</span><span>SWORDPLAY</span><span>+ 7 MORE</span></div>
        </section>}

        {page==="mods"&&active&&<section>
          <div className="page-title"><div><small>MAKE IT YOURS</small><h1>Mods & textures</h1><p>Import Riivolution patches, Dolphin resource packs, and Gecko codes.</p></div></div>
          <div className="action-grid">
            <article className="action-card"><Archive/><h3>Riivolution patches</h3><p>Import ZIPs containing Riivolution XML and replacement files. The manager installs them into Dolphin's Load/Riivolution folder.</p><button className="secondary" disabled={busy} onClick={()=>addMod("riivolution")}><PackagePlus size={15}/> IMPORT RIIVOLUTION ZIP</button></article>
            <article className="action-card"><Image/><h3>HD texture packs</h3><p>Import Dolphin Resource Pack ZIPs with a manifest and textures. Enabled packs are installed under the profile's ResourcePacks directory.</p><button className="secondary" disabled={busy} onClick={()=>addMod("resource_pack")}><PackagePlus size={15}/> IMPORT RESOURCE PACK</button></article>
          </div>
          <div className="section-title"><div><small>PROFILE CONTENT</small><h2>Installed modifications</h2></div><span className="count">{active.mods.length} INSTALLED</span></div>
          <div className="list-card">
            {active.mods.length===0&&<div className="empty"><Layers3/><b>No mods installed yet</b><span>Import a Riivolution archive or Dolphin resource pack.</span></div>}
            {active.mods.map(item=><div className="mod-row" key={item.id}><div className="mod-icon">{item.kind==="riivolution"?<Archive/>:<Image/>}</div><div className="mod-meta"><b>{item.name}</b><small>{item.kind==="riivolution"?"RIIVOLUTION PATCH":"DOLPHIN RESOURCE PACK"}{item.version?" · "+item.version:""}{item.gamePrefix?" · "+item.gamePrefix+" family":""}</small></div><span className={item.enabled?"enabled":"disabled"}>{item.enabled?"ENABLED":"DISABLED"}</span><button className="icon-button" onClick={()=>toggleMod(item)} title={item.enabled?"Disable":"Enable"}>{item.enabled?<X size={14}/>:<Check size={14}/>}</button><button className="icon-button danger" onClick={()=>deleteMod(item)} title="Remove"><Trash2 size={14}/></button></div>)}
          </div>
          <div className="info"><Info/><div><b>Riivolution launch note</b><p>Dolphin's desktop workflow still requires selecting “Start with Riivolution Patches” from its game list, then choosing the XML/preset. The normal Play button launches the unpatched image. The button opens Dolphin with this profile's user folder.</p><button className="text-link" onClick={openDolphin}>OPEN DOLPHIN FOR RIIVOLUTION <ArrowUpRight size={14}/></button></div></div>
          <div className="section-title"><div><small>GECKO CODES</small><h2>Code manager</h2></div></div>
          <div className="form-card"><label>Code name<input value={codeName} onChange={e=>setCodeName(e.target.value)} placeholder="60 FPS patch"/></label><label>Code lines<textarea rows={4} value={codeText} onChange={e=>setCodeText(e.target.value)} placeholder={"04000000 00000000\n..."} /></label><button className="secondary" onClick={saveCode} disabled={!config.gameId}><Save size={15}/> SAVE TO DOLPHIN GAME SETTINGS</button><small>Code tokens must be eight-character hexadecimal values. Only enable codes from sources you trust.</small></div>
          {active.geckoCodes.map((code,index)=><div className="code-row" key={index}><b>{code.name}</b><small>{code.enabled?"Enabled":"Disabled"}</small></div>)}
        </section>}

        {page==="graphics"&&active&&<section>
          <div className="page-title"><div><small>VISUAL EXPERIENCE</small><h1>Graphics</h1><p>Per-profile Dolphin settings. Changes apply the next time you launch the game.</p></div></div>
          <div className="graphics-banner"><div><small>INTERNAL RENDERING SCALE</small><h2>{active.graphics.internalResolution}× <em>native</em></h2><p>Higher scale improves clarity but increases GPU and VRAM requirements.</p></div><div className="resolution-label"><Monitor size={34}/><b>{active.graphics.internalResolution>=12?"8K TARGET":active.graphics.internalResolution>=6?"4K+ TARGET":active.graphics.internalResolution>=3?"1440p+ TARGET":"NATIVE / HD"}</b></div></div>
          <div className="settings-grid">
            <div className="setting wide"><div className="setting-head"><div><b>Internal resolution</b><small>Scales the emulated Wii framebuffer, not your monitor.</small></div><span>{active.graphics.internalResolution}×</span></div><input type="range" min="1" max="12" value={active.graphics.internalResolution} onChange={e=>updateGraphics({...active.graphics,internalResolution:Number(e.target.value)})}/><div className="range-labels"><small>Native</small><small>12× maximum</small></div><div className="presets">{[1,2,3,4,6,8,12].map(value=><button className={active.graphics.internalResolution===value?"chosen":""} key={value} onClick={()=>updateGraphics({...active.graphics,internalResolution:value})}>{value}×</button>)}</div></div>
            <div className="setting"><b>Anti-aliasing (MSAA)</b><small>Higher values cost more GPU time.</small><select value={active.graphics.msaa} onChange={e=>updateGraphics({...active.graphics,msaa:Number(e.target.value)})}><option value="1">Off</option><option value="2">2×</option><option value="4">4×</option><option value="8">8×</option><option value="16">16×</option></select></div>
            <div className="setting"><b>Anisotropic filtering preference</b><small>Profile setting; mapping is pending version verification.</small><select value={active.graphics.anisotropy} onChange={e=>updateGraphics({...active.graphics,anisotropy:Number(e.target.value)})}>{[1,2,4,8,16].map(v=><option key={v} value={v}>{v}×</option>)}</select></div>
            <Toggle title="Vertex rounding" hint="May help some alignment issues but can cause menu animation jitter." value={active.graphics.vertexRounding} onChange={value=>updateGraphics({...active.graphics,vertexRounding:value})}/>
            <Toggle title="Show FPS" hint="Show frame-rate metrics in Dolphin." value={active.graphics.showFps} onChange={value=>updateGraphics({...active.graphics,showFps:value})}/>
            <Toggle title="Borderless fullscreen" hint="Use borderless mode when supported by Dolphin." value={active.graphics.borderlessFullscreen} onChange={value=>updateGraphics({...active.graphics,borderlessFullscreen:value})}/>
          </div>
          <div className="info"><Info/><div><b>About 8K</b><p>12× is Dolphin's maximum internal-resolution setting in the current engine. It does not force the separate Dolphin game window to 7680×4320. Final output depends on the display, window, backend, and GPU.</p></div></div>
        </section>}

        {page==="controls"&&active&&<section>
          <div className="page-title"><div><small>MOTION & INPUT</small><h1>Controls</h1><p>Choose the Wii Remote source and save a preferred setup per profile.</p></div></div>
          <div className="controller-options">
            <button className={active.controller.mode==="real"?"controller chosen":"controller"} onClick={()=>updateController({...active.controller,mode:"real"})}><span>⌁</span><b>Real Wii Remote</b><small>Bluetooth Remote with MotionPlus</small><em>Best motion accuracy</em></button>
            <button className={active.controller.mode==="emulated"?"controller chosen":"controller"} onClick={()=>updateController({...active.controller,mode:"emulated"})}><span>⌘</span><b>Virtual Wii Remote</b><small>Gamepad, keyboard, or motion-capable device</small><em>Flexible PC input</em></button>
          </div>
          <div className="settings-grid">
            <Toggle title="MotionPlus preferred" hint="Wii Sports Resort requires MotionPlus functionality." value={active.controller.motionPlus} onChange={value=>updateController({...active.controller,motionPlus:value})}/>
            <Toggle title="Nunchuk preferred" hint="Enable the Nunchuk extension for sports that require it." value={active.controller.nunchuk} onChange={value=>updateController({...active.controller,nunchuk:value})}/>
            <div className="setting wide"><b>Preferred controller/device</b><small>Profile note (e.g. Wii Remote 1, DualSense via DSU).</small><input value={active.controller.deviceLabel} onChange={e=>updateController({...active.controller,deviceLabel:e.target.value})} placeholder="Enter device name"/></div>
            <div className="setting wide"><div className="setting-head"><b>Motion sensitivity preference</b><span>{active.controller.sensitivity.toFixed(1)}×</span></div><input type="range" min=".5" max="2" step=".1" value={active.controller.sensitivity} onChange={e=>updateController({...active.controller,sensitivity:Number(e.target.value)})}/><small>Stored for the future mapping layer; not applied as a universal Dolphin motion multiplier.</small></div>
          </div>
          <div className="info"><Joystick/><div><b>Detailed motion mapping in Dolphin</b><p>The launcher selects Real or Emulated Wii Remote for slot 1. Gyro, IR pointer, swing, rumble, Nunchuk and MotionPlus extension mappings remain configured through Dolphin's controller interface. Pair physical remotes inside Dolphin rather than in OS Bluetooth settings.</p><button className="text-link" onClick={openDolphin}>OPEN DOLPHIN FOR CONTROLLER SETUP <ArrowUpRight size={14}/></button></div></div>
        </section>}

        {page==="saves"&&active&&<section>
          <div className="page-title"><div><small>PROTECT YOUR PROGRESS</small><h1>Saves & backups</h1><p>Each profile has a persistent Dolphin user folder outside the application install.</p></div><button className="secondary" onClick={backup} disabled={busy}><Save size={15}/> BACK UP NOW</button></div>
          <div className="save-card"><div><HardDrive/></div><article><small>ACTIVE PROFILE DATA</small><b>{active.name}</b><span>NAND saves · save states · screenshots</span></article><button className="text-link" onClick={openProfileFolder}>OPEN DATA FOLDER <ArrowUpRight size={14}/></button></div>
          <div className="section-title"><div><small>RESTORE POINTS</small><h2>Available backups</h2></div><button className="text-link" onClick={refreshBackups}>Refresh <RefreshCw size={14}/></button></div>
          <div className="list-card">{backups.length===0?<div className="empty"><Save/><b>No backups listed</b><span>Create a backup or refresh this list.</span></div>:backups.map(item=><div className="backup-row" key={item.id}><Archive/><div><b>{item.id}</b><small>{item.path}</small></div><button className="secondary compact" onClick={()=>restore(item.id)} disabled={busy}><ArrowDownToLine size={14}/> RESTORE</button></div>)}</div>
          <div className="info"><ShieldCheck/><div><b>Safe restore behavior</b><p>Before restoring, the app backs up the current profile. Restore replaces only managed Wii/title, StateSaves and Screenshots folders; mods and emulator settings are left alone.</p></div></div>
        </section>}

        {page==="settings"&&<section>
          <div className="page-title"><div><small>APPLICATION SETUP</small><h1>Settings</h1><p>Connect Dolphin and your legally obtained Wii Sports Resort image.</p></div></div>
          <div className="setup-card">
            <SetupRow number="01" title="Dolphin emulator" complete={!!dolphin} detail={dolphin?dolphin.version+" · "+dolphin.executable:"Select Dolphin.exe, a Linux Dolphin binary, or the executable inside Dolphin.app."} action={chooseDolphin}/>
            <SetupRow number="02" title="Wii Sports Resort image" complete={!!game} detail={game?game.path+" · "+bytes(game.sizeBytes):"Choose ISO, RVZ, WBFS, WIA, GCZ, GCM or CISO."} action={chooseGame}/>
            <div className="setup-row"><i>03</i><div><b>Active profile</b><small>{active?.name??"Default"} · {config.profiles.length} profile(s)</small></div><button className="secondary" onClick={addProfile}><PackagePlus size={14}/> NEW PROFILE</button></div>
          </div>
          {game&&<div className="form-card"><label>Game ID (needed for Gecko codes)<input value={config.gameId??""} onChange={e=>updateGameId(e.target.value)} placeholder="RZTE01" maxLength={6}/></label><small>Raw ISO/GCM IDs are detected automatically. For RVZ/WBFS/WIA/GCZ, confirm the six-character ID in Dolphin's game details.</small></div>}
          {active&&<div className="form-card"><div className="section-title"><div><small>PROFILE MANAGEMENT</small><h2>Current profile</h2></div></div><label>Profile name<input value={active.name} onChange={e=>saveProfile({...active,name:e.target.value})}/></label><div className="button-row"><button className="secondary" onClick={addProfile}><PackagePlus size={14}/> ADD PROFILE</button><button className="secondary danger-button" disabled={config.profiles.length<2} onClick={async()=>{if(!window.confirm("Delete this profile? Its Dolphin user data and saves will be retained."))return;const rest=config.profiles.filter(p=>p.id!==active.id);await persist({...config,profiles:rest,activeProfileId:rest[0].id});}}> <Trash2 size={14}/> DELETE PROFILE</button></div></div>}
          <div className="info"><Info/><div><b>Runtime ownership</b><p>The game image, NAND, firmware and proprietary Nintendo artwork are not bundled. The user selects a Dolphin installation. Any future bundled Dolphin runtime must satisfy GPL-2.0-or-later obligations.</p><button className="text-link" onClick={openProfileFolder}>OPEN PROFILE DATA <ArrowUpRight size={14}/></button></div></div>
        </section>}
      </div>
    </main>
    {busy&&<div className="busy"><div className="spinner"/><span>WORKING…</span></div>}
  </div>;
}

function Toggle({title,hint,value,onChange}:{title:string;hint:string;value:boolean;onChange:(value:boolean)=>void}){
  return <div className="setting toggle-setting"><div><b>{title}</b><small>{hint}</small></div><label className="switch"><input type="checkbox" checked={value} onChange={e=>onChange(e.target.checked)}/><span/></label></div>;
}
function SetupRow({number,title,complete,detail,action}:{number:string;title:string;complete:boolean;detail:string;action:()=>void}){
  return <div className="setup-row"><i className={complete?"done":""}>{complete?<Check size={15}/>:number}</i><div><b>{title}</b><small>{detail}</small></div><button className="secondary" onClick={action}><FolderOpen size={14}/> BROWSE</button></div>;
}
