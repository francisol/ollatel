import { load } from '@tauri-apps/plugin-store';


export async function initDone() {
    const store = await load('store.bin');

    await store.set('init',  true );
}
export async function isInited() {
    const store = await load('store.bin');

 return   await ( store).get<boolean>('init');
}

export interface Settings{
    autoStart: boolean,
    autoRunOllama: boolean,
}

export async function loadSettings() {
    const store = await load('store.bin');

   return await store.get<Settings>('setting' );
}
export async function saveSettings(setting:Settings) {
    const store = await load('store.bin');
    await ( store).set('setting',setting);
}

export async function saveEnvVariables (env:any) {
    const store = await load('env.bin');
    await ( store).set('env',env);
}