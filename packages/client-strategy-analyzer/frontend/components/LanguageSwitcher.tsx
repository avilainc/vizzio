import React from 'react';
import { useI18n } from '../hooks/useI18n';

export const LanguageSwitcher: React.FC = () => {
  const { language, setLanguage } = useI18n();

  return (
    <div className="flex gap-2 items-center">
      <button
        onClick={() => setLanguage('pt-BR')}
        className={`px-3 py-1 rounded text-sm font-medium transition-colors ${
          language === 'pt-BR'
            ? 'bg-purple-600 text-white'
            : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
        }`}
      >
        🇧🇷 Português
      </button>
      <button
        onClick={() => setLanguage('en-US')}
        className={`px-3 py-1 rounded text-sm font-medium transition-colors ${
          language === 'en-US'
            ? 'bg-purple-600 text-white'
            : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
        }`}
      >
        🇺🇸 English
      </button>
    </div>
  );
};
