"use strict";
/**
 * Sistema de Atalhos - Keyboard, Voice, Gestures
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.ShortcutManager = void 0;
class ShortcutManager {
    constructor() {
        this.shortcuts = new Map();
    }
    /**
     * Registrar atalho de teclado
     * Ex: Ctrl+Alt+A
     */
    registerKeyboardShortcut(binding, action, callback) {
        const shortcut = {
            id: `kb-${Date.now()}`,
            name: action,
            type: 'keyboard',
            binding,
            action,
            metadata: { callback },
        };
        this.shortcuts.set(binding, shortcut);
    }
    /**
     * Registrar atalho de voz
     * Ex: "Começar automação"
     */
    registerVoiceShortcut(phrase, action, callback) {
        const shortcut = {
            id: `voice-${Date.now()}`,
            name: action,
            type: 'voice',
            binding: phrase,
            action,
            metadata: { callback },
        };
        this.shortcuts.set(phrase, shortcut);
    }
    /**
     * Executar atalho
     */
    async executeShortcut(binding) {
        const shortcut = this.shortcuts.get(binding);
        if (!shortcut)
            return false;
        try {
            const callback = shortcut.metadata?.callback;
            if (callback && typeof callback === 'function') {
                await callback();
            }
            return true;
        }
        catch (error) {
            console.error(`Erro ao executar atalho: ${binding}`, error);
            return false;
        }
    }
    /**
     * Listar todos os atalhos
     */
    listShortcuts() {
        return Array.from(this.shortcuts.values());
    }
    /**
     * Remover atalho
     */
    removeShortcut(binding) {
        return this.shortcuts.delete(binding);
    }
}
exports.ShortcutManager = ShortcutManager;
//# sourceMappingURL=ShortcutManager.js.map