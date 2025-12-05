import axios from 'axios';

interface AnalysisRequest {
  caseDescription: string;
  clientCompany?: string;
  objectives?: string[];
  challenges?: string[];
}

interface AnalysisResponse {
  strategy: string;
  recommendations: string[];
  timeline: string;
  estimatedBudget: number;
  risks: string[];
}

export class CopilotAnalysisService {
  private apiKey: string;
  private model: string = 'gpt-4';

  constructor(apiKey: string) {
    this.apiKey = apiKey;
  }

  async analyzeCase(request: AnalysisRequest): Promise<AnalysisResponse> {
    try {
      const prompt = this.buildPrompt(request);

      const response = await axios.post(
        'https://api.openai.com/v1/chat/completions',
        {
          model: this.model,
          messages: [
            {
              role: 'system',
              content: `Você é um consultor experiente em estratégia de negócios.
              Analise o caso do cliente e forneça uma estratégia detalhada,
              recomendações práticas, timeline e orçamento estimado em formato JSON.`,
            },
            {
              role: 'user',
              content: prompt,
            },
          ],
          temperature: 0.7,
          max_tokens: 2000,
        },
        {
          headers: {
            'Authorization': `Bearer ${this.apiKey}`,
            'Content-Type': 'application/json',
          },
        }
      );

      const content = response.data.choices[0].message.content;
      const analysis = this.parseAnalysis(content);

      return analysis;
    } catch (error) {
      console.error('Erro ao analisar caso com Copilot:', error);
      throw error;
    }
  }

  private buildPrompt(request: AnalysisRequest): string {
    return `
Analise o seguinte caso de cliente e forneça uma resposta estruturada em JSON:

**Descrição do Caso:**
${request.caseDescription}

**Empresa:** ${request.clientCompany || 'Não informado'}
**Objetivos:** ${request.objectives?.join(', ') || 'Não informados'}
**Desafios:** ${request.challenges?.join(', ') || 'Não informados'}

Por favor, forneça uma análise estruturada no seguinte formato JSON:
{
  "strategy": "Estratégia detalhada em 2-3 parágrafos",
  "recommendations": ["Recomendação 1", "Recomendação 2", "Recomendação 3"],
  "timeline": "Timeline em fases (Ex: Fase 1: 2 semanas, Fase 2: 4 semanas...)",
  "estimatedBudget": 15000,
  "risks": ["Risco 1", "Risco 2"]
}
    `;
  }

  private parseAnalysis(content: string): AnalysisResponse {
    try {
      // Tentar extrair JSON da resposta
      const jsonMatch = content.match(/\{[\s\S]*\}/);
      if (jsonMatch) {
        return JSON.parse(jsonMatch[0]);
      }

      // Fallback para análise manual
      return {
        strategy: content,
        recommendations: [
          'Implementar estratégia baseada em dados',
          'Realizar testes A/B',
          'Monitorar métricas de sucesso',
        ],
        timeline: '30 dias',
        estimatedBudget: 10000,
        risks: ['Resistência à mudança', 'Limitações técnicas'],
      };
    } catch (error) {
      throw new Error(`Erro ao parsear análise: ${error}`);
    }
  }
}
