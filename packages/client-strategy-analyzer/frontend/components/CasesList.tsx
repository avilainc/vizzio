import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { FiRefreshCw, FiMail, FiEye } from 'react-icons/fi';

interface Case {
  _id: string;
  clientName: string;
  clientEmail: string;
  clientCompany?: string;
  caseCategory: string;
  status: string;
  createdAt: string;
}

export const CasesList: React.FC = () => {
  const [cases, setCases] = useState<Case[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    loadCases();
  }, []);

  const loadCases = async () => {
    setLoading(true);
    try {
      const response = await axios.get(
        `${process.env.NEXT_PUBLIC_API_URL}/api/cases`
      );
      setCases(response.data.data || []);
    } catch (error) {
      console.error('Erro ao carregar casos:', error);
    } finally {
      setLoading(false);
    }
  };

  const analyzeCase = async (caseId: string) => {
    try {
      await axios.post(
        `${process.env.NEXT_PUBLIC_API_URL}/api/cases/${caseId}/analyze`
      );
      loadCases();
      alert('Caso analisado com sucesso!');
    } catch (error) {
      alert('Erro ao analisar caso');
    }
  };

  const sendProposal = async (caseId: string) => {
    try {
      await axios.post(
        `${process.env.NEXT_PUBLIC_API_URL}/api/cases/${caseId}/send-proposal`
      );
      loadCases();
      alert('Proposta enviada com sucesso!');
    } catch (error) {
      alert('Erro ao enviar proposta');
    }
  };

  const getStatusColor = (status: string) => {
    const colors: Record<string, string> = {
      draft: 'bg-gray-100 text-gray-800',
      analyzing: 'bg-blue-100 text-blue-800',
      analyzed: 'bg-yellow-100 text-yellow-800',
      proposal_sent: 'bg-green-100 text-green-800',
      completed: 'bg-purple-100 text-purple-800',
    };
    return colors[status] || 'bg-gray-100 text-gray-800';
  };

  return (
    <div className="overflow-x-auto">
      <table className="w-full">
        <thead>
          <tr className="border-b border-gray-200">
            <th className="px-4 py-2 text-left font-semibold">Cliente</th>
            <th className="px-4 py-2 text-left font-semibold">Empresa</th>
            <th className="px-4 py-2 text-left font-semibold">Categoria</th>
            <th className="px-4 py-2 text-left font-semibold">Status</th>
            <th className="px-4 py-2 text-left font-semibold">Ações</th>
          </tr>
        </thead>
        <tbody>
          {cases.map(caseItem => (
            <tr key={caseItem._id} className="border-b border-gray-100 hover:bg-gray-50">
              <td className="px-4 py-3">{caseItem.clientName}</td>
              <td className="px-4 py-3">{caseItem.clientCompany || '-'}</td>
              <td className="px-4 py-3 capitalize">{caseItem.caseCategory}</td>
              <td className="px-4 py-3">
                <span
                  className={`px-3 py-1 rounded-full text-sm font-medium ${getStatusColor(
                    caseItem.status
                  )}`}
                >
                  {caseItem.status.replace('_', ' ')}
                </span>
              </td>
              <td className="px-4 py-3 flex gap-2">
                {caseItem.status === 'draft' && (
                  <button
                    onClick={() => analyzeCase(caseItem._id)}
                    className="p-2 bg-blue-100 text-blue-600 rounded hover:bg-blue-200"
                    title="Analisar"
                  >
                    <FiRefreshCw size={18} />
                  </button>
                )}

                {caseItem.status === 'analyzed' && (
                  <button
                    onClick={() => sendProposal(caseItem._id)}
                    className="p-2 bg-green-100 text-green-600 rounded hover:bg-green-200"
                    title="Enviar Proposta"
                  >
                    <FiMail size={18} />
                  </button>
                )}

                <button
                  className="p-2 bg-gray-100 text-gray-600 rounded hover:bg-gray-200"
                  title="Visualizar"
                >
                  <FiEye size={18} />
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>

      {cases.length === 0 && !loading && (
        <div className="text-center py-8 text-gray-500">
          Nenhum caso encontrado
        </div>
      )}

      {loading && (
        <div className="text-center py-8 text-gray-500">
          Carregando casos...
        </div>
      )}
    </div>
  );
};
