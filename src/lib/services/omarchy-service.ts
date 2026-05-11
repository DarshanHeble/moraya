import { invoke } from '@tauri-apps/api/core';
import { settingsStore } from '$lib/stores/settings-store';
import { builtinThemes, type ColorTheme } from '$lib/styles/themes';

export interface OmarchyColors {
    accent: String;
    background: String;
    foreground: String;
    cursor: String;
    selection_background: String;
    selection_foreground: String;
    color0: String;
    color1: String;
    color2: String;
    color3: String;
    color4: String;
    color5: String;
    color6: String;
    color7: String;
    color8: String;
    color9: String;
    color10: String;
    color11: String;
    color12: String;
    color13: String;
    color14: String;
    color15: String;
}

export async function syncWithOmarchyTheme() {
    try {
        const colors = await invoke<OmarchyColors | null>('get_omarchy_colors');
        if (!colors) return;

        const omarchyTheme: ColorTheme = {
            id: 'omarchy-sync',
            name: 'Omarchy (System)',
            type: 'dark', // Omarchy is mostly dark by default
            colors: {
                '--bg-primary': colors.background as string,
                '--bg-secondary': colors.color0 as string,
                '--bg-sidebar': colors.color0 as string,
                '--bg-titlebar': colors.background as string,
                '--bg-hover': colors.color8 + '33', // 20% opacity
                '--bg-active': colors.color8 + '66', // 40% opacity
                '--text-primary': colors.foreground as string,
                '--text-secondary': colors.color7 as string,
                '--text-muted': colors.color8 as string,
                '--border-color': colors.color8 as string,
                '--border-light': colors.color0 as string,
                '--accent-color': colors.accent as string,
                '--accent-hover': colors.color12 as string,
                '--scrollbar-thumb': colors.color8 as string,
                // Map some highlight colors if possible
                '--hljs-keyword': colors.color1 as string,
                '--hljs-string': colors.color2 as string,
                '--hljs-number': colors.color3 as string,
                '--hljs-comment': colors.color8 as string,
                '--hljs-function': colors.color4 as string,
            }
        };

        // We can't easily push to builtinThemes as it's a constant.
        // Instead, we can apply these colors directly to document.documentElement
        const root = document.documentElement;
        for (const [prop, value] of Object.entries(omarchyTheme.colors)) {
            root.style.setProperty(prop, value);
        }
        
        // Also update settings to mark that we are in "Omarchy Sync" mode
        // This is a bit of a hack since we don't have a formal "custom theme" slot yet
    } catch (e) {
        console.error('Failed to sync with Omarchy:', e);
    }
}
