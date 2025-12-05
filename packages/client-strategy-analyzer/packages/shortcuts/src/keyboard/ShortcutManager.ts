/**
 * Sistema de Atalhos - Keyboard, Voice, Gestures
 */

import { Shortcut } from '@vizzio/core';

export class ShortcutManager {
  private shortcuts: Map<string, Shortcut> = new Map();

  /**
   * Registrar atalho de teclado
   * Ex: Ctrl+Alt+A
   */
  registerKeyboardShortcut(
    binding: string,
    action: string,
    callback: () => Promise<void>
  ): void {
    const shortcut: Shortcut = {
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
  registerVoiceShortcut(
    phrase: string,
    action: string,
    callback: () => Promise<void>
  ): void {
    const shortcut: Shortcut = {
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
  async executeShortcut(binding: string): Promise<boolean> {
    const shortcut = this.shortcuts.get(binding);
    if (!shortcut) return false;

    try {
      const callback = shortcut.metadata?.callback;
      if (callback && typeof callback === 'function') {
        await callback();
      }
      return true;
    } catch (error) {
      console.error(`Erro ao executar atalho: ${binding}`, error);
      return false;
    }
  }

  /**
   * Listar todos os atalhos
   */
  listShortcuts(): Shortcut[] {
    return Array.from(this.shortcuts.values());
  }

  /**
   * Remover atalho
   */
  removeShortcut(binding: string): boolean {
    return this.shortcuts.delete(binding);
  }
}
