/**
 * Sistema de Atalhos - Keyboard, Voice, Gestures
 */
import { Shortcut } from '@vizzio/core';
export declare class ShortcutManager {
    private shortcuts;
    /**
     * Registrar atalho de teclado
     * Ex: Ctrl+Alt+A
     */
    registerKeyboardShortcut(binding: string, action: string, callback: () => Promise<void>): void;
    /**
     * Registrar atalho de voz
     * Ex: "Começar automação"
     */
    registerVoiceShortcut(phrase: string, action: string, callback: () => Promise<void>): void;
    /**
     * Executar atalho
     */
    executeShortcut(binding: string): Promise<boolean>;
    /**
     * Listar todos os atalhos
     */
    listShortcuts(): Shortcut[];
    /**
     * Remover atalho
     */
    removeShortcut(binding: string): boolean;
}
//# sourceMappingURL=ShortcutManager.d.ts.map