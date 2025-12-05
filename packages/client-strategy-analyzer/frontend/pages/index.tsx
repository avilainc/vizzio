import React, { useState } from 'react';
import { CaseForm } from '../components/CaseForm';
import { CasesList } from '../components/CasesList';
import '../styles/globals.css';

export default function Dashboard() {
  const [activeTab, setActiveTab] = useState<'create' | 'list'>('list');
  const [refreshList, setRefreshList] = useState(false);

  return (
    <div className="min-h-screen bg-gradient-to-br from-purple-50 to-pink-50">
      {/* Header */}
      <header className="bg-white shadow">
        <div className="container-main">
          <h1 className="text-3xl font-bold bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text text-transparent py-6">
            📊 Client Strategy Analyzer
          </h1>
          <p className="text-gray-600 mb-4">
            Sistema inteligente de análise e propostas para clientes
          </p>
        </div>
      </header>

      {/* Navigation */}
      <nav className="bg-white border-b sticky top-0 z-10">
        <div className="container-main">
          <div className="flex gap-4">
            <button
              onClick={() => setActiveTab('list')}
              className={`px-4 py-3 font-medium border-b-2 transition-colors ${
                activeTab === 'list'
                  ? 'border-purple-600 text-purple-600'
                  : 'border-transparent text-gray-600 hover:text-gray-900'
              }`}
            >
              📋 Casos
            </button>
            <button
              onClick={() => setActiveTab('create')}
              className={`px-4 py-3 font-medium border-b-2 transition-colors ${
                activeTab === 'create'
                  ? 'border-purple-600 text-purple-600'
                  : 'border-transparent text-gray-600 hover:text-gray-900'
              }`}
            >
              ➕ Novo Caso
            </button>
          </div>
        </div>
      </nav>

      {/* Main Content */}
      <main className="container-main pb-12">
        {activeTab === 'list' && (
          <div className="card">
            <h2 className="text-2xl font-bold mb-6">Casos em Análise</h2>
            <CasesList key={refreshList ? 'refresh' : 'normal'} />
          </div>
        )}

        {activeTab === 'create' && (
          <div className="card">
            <h2 className="text-2xl font-bold mb-6">Registrar Novo Caso</h2>
            <CaseForm
              onSuccess={() => {
                setActiveTab('list');
                setRefreshList(!refreshList);
              }}
            />
          </div>
        )}
      </main>

      {/* Footer */}
      <footer className="bg-gray-900 text-white mt-12">
        <div className="container-main text-center py-6">
          <p>© 2024 Client Strategy Analyzer - Todos os direitos reservados</p>
        </div>
      </footer>
    </div>
  );
}
