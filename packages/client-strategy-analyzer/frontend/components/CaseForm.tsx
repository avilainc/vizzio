import React, { useState } from 'react';
import axios from 'axios';

interface CaseFormProps {
  onSuccess?: () => void;
}

export const CaseForm: React.FC<CaseFormProps> = ({ onSuccess }) => {
  const [formData, setFormData] = useState({
    clientName: '',
    clientEmail: '',
    clientPhone: '',
    clientCompany: '',
    caseDescription: '',
    caseCategory: 'marketing',
    objectives: '',
    challenges: '',
    budget: '',
    timeline: '',
  });

  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState('');

  const handleChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>
  ) => {
    const { name, value } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: value,
    }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);

    try {
      const payload = {
        ...formData,
        objectives: formData.objectives
          .split(',')
          .map((o: string) => o.trim())
          .filter((o: string) => o),
        challenges: formData.challenges
          .split(',')
          .map((c: string) => c.trim())
          .filter((c: string) => c),
        budget: formData.budget ? parseInt(formData.budget) : undefined,
      };

      const response = await axios.post(
        `${process.env.NEXT_PUBLIC_API_URL}/api/cases`,
        payload
      );

      setMessage('Caso criado com sucesso! 🎉');
      setFormData({
        clientName: '',
        clientEmail: '',
        clientPhone: '',
        clientCompany: '',
        caseDescription: '',
        caseCategory: 'marketing',
        objectives: '',
        challenges: '',
        budget: '',
        timeline: '',
      });

      if (onSuccess) {
        setTimeout(onSuccess, 1500);
      }
    } catch (error) {
      setMessage('Erro ao criar caso. Tente novamente.');
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div className="form-group">
          <label className="label">Nome do Cliente *</label>
          <input
            type="text"
            name="clientName"
            value={formData.clientName}
            onChange={handleChange}
            className="input"
            required
            placeholder="João Silva"
          />
        </div>

        <div className="form-group">
          <label className="label">Email *</label>
          <input
            type="email"
            name="clientEmail"
            value={formData.clientEmail}
            onChange={handleChange}
            className="input"
            required
            placeholder="joao@example.com"
          />
        </div>

        <div className="form-group">
          <label className="label">Telefone</label>
          <input
            type="tel"
            name="clientPhone"
            value={formData.clientPhone}
            onChange={handleChange}
            className="input"
            placeholder="(11) 98765-4321"
          />
        </div>

        <div className="form-group">
          <label className="label">Empresa</label>
          <input
            type="text"
            name="clientCompany"
            value={formData.clientCompany}
            onChange={handleChange}
            className="input"
            placeholder="Empresa XYZ"
          />
        </div>

        <div className="form-group">
          <label className="label">Categoria *</label>
          <select
            name="caseCategory"
            value={formData.caseCategory}
            onChange={handleChange}
            className="input"
          >
            <option value="marketing">Marketing</option>
            <option value="vendas">Vendas</option>
            <option value="operacional">Operacional</option>
            <option value="rh">RH</option>
            <option value="tecnologia">Tecnologia</option>
            <option value="financeiro">Financeiro</option>
            <option value="outro">Outro</option>
          </select>
        </div>

        <div className="form-group">
          <label className="label">Orçamento (R$)</label>
          <input
            type="number"
            name="budget"
            value={formData.budget}
            onChange={handleChange}
            className="input"
            placeholder="10000"
          />
        </div>
      </div>

      <div className="form-group">
        <label className="label">Descrição do Caso *</label>
        <textarea
          name="caseDescription"
          value={formData.caseDescription}
          onChange={handleChange}
          className="input min-h-32"
          required
          placeholder="Descreva detalhadamente o caso do cliente..."
        />
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div className="form-group">
          <label className="label">Objetivos (separados por vírgula)</label>
          <textarea
            name="objectives"
            value={formData.objectives}
            onChange={handleChange}
            className="input min-h-24"
            placeholder="Aumentar vendas, Melhorar imagem, ..."
          />
        </div>

        <div className="form-group">
          <label className="label">Desafios (separados por vírgula)</label>
          <textarea
            name="challenges"
            value={formData.challenges}
            onChange={handleChange}
            className="input min-h-24"
            placeholder="Concorrência, Orçamento limitado, ..."
          />
        </div>
      </div>

      <div className="form-group">
        <label className="label">Timeline</label>
        <input
          type="text"
          name="timeline"
          value={formData.timeline}
          onChange={handleChange}
          className="input"
          placeholder="Ex: 30 dias, 3 meses, etc."
        />
      </div>

      {message && (
        <div
          className={`p-4 rounded-lg ${
            message.includes('sucesso')
              ? 'bg-green-100 text-green-800'
              : 'bg-red-100 text-red-800'
          }`}
        >
          {message}
        </div>
      )}

      <button
        type="submit"
        disabled={loading}
        className="btn btn-primary w-full"
      >
        {loading ? 'Criando caso...' : 'Criar Caso'}
      </button>
    </form>
  );
};
