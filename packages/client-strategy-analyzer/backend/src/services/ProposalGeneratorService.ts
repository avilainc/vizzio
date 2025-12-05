export interface ProposalData {
  clientName: string;
  clientCompany?: string;
  strategy: string;
  recommendations: string[];
  timeline: string;
  estimatedBudget: number;
  risks?: string[];
  caseDescription: string;
}

export class ProposalGeneratorService {
  generateHTML(data: ProposalData): string {
    return `
<!DOCTYPE html>
<html lang="pt-BR">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Proposta de Estratégia</title>
  <style>
    * {
      margin: 0;
      padding: 0;
      box-sizing: border-box;
    }

    body {
      font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
      line-height: 1.6;
      color: #333;
      background: #f5f5f5;
    }

    .container {
      max-width: 900px;
      margin: 0 auto;
      background: white;
      box-shadow: 0 2px 10px rgba(0,0,0,0.1);
    }

    .header {
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
      color: white;
      padding: 40px;
      text-align: center;
    }

    .header h1 {
      font-size: 28px;
      margin-bottom: 10px;
    }

    .header p {
      font-size: 16px;
      opacity: 0.9;
    }

    .content {
      padding: 40px;
    }

    .section {
      margin-bottom: 35px;
    }

    .section h2 {
      color: #667eea;
      font-size: 22px;
      margin-bottom: 15px;
      padding-bottom: 10px;
      border-bottom: 2px solid #667eea;
    }

    .client-info {
      background: #f9f9f9;
      padding: 20px;
      border-radius: 8px;
      margin-bottom: 25px;
    }

    .client-info p {
      margin-bottom: 8px;
    }

    .client-info strong {
      color: #667eea;
    }

    .strategy-text {
      background: #f0f4ff;
      padding: 20px;
      border-left: 4px solid #667eea;
      border-radius: 4px;
      line-height: 1.8;
    }

    .recommendations {
      list-style: none;
    }

    .recommendations li {
      padding: 12px 0;
      padding-left: 30px;
      position: relative;
    }

    .recommendations li:before {
      content: "✓";
      position: absolute;
      left: 0;
      color: #667eea;
      font-weight: bold;
      font-size: 18px;
    }

    .timeline {
      background: #fff3cd;
      padding: 20px;
      border-radius: 8px;
      border-left: 4px solid #ffc107;
    }

    .budget-box {
      background: #d4edda;
      padding: 20px;
      border-radius: 8px;
      border-left: 4px solid #28a745;
      text-align: center;
    }

    .budget-box .label {
      color: #155724;
      font-size: 14px;
      margin-bottom: 5px;
    }

    .budget-box .amount {
      font-size: 32px;
      font-weight: bold;
      color: #155724;
    }

    .risks {
      background: #f8d7da;
      padding: 20px;
      border-radius: 8px;
      border-left: 4px solid #dc3545;
    }

    .risks h3 {
      color: #721c24;
      margin-bottom: 12px;
    }

    .risks ul {
      list-style: none;
      padding-left: 20px;
    }

    .risks li {
      margin-bottom: 8px;
      color: #721c24;
    }

    .risks li:before {
      content: "⚠ ";
      margin-right: 8px;
    }

    .footer {
      background: #f9f9f9;
      padding: 30px 40px;
      text-align: center;
      border-top: 1px solid #ddd;
    }

    .cta-button {
      display: inline-block;
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
      color: white;
      padding: 12px 30px;
      border-radius: 5px;
      text-decoration: none;
      margin-top: 15px;
      font-weight: bold;
    }

    .divider {
      height: 1px;
      background: #ddd;
      margin: 30px 0;
    }

    @media (max-width: 600px) {
      .container {
        margin: 0;
      }

      .header, .content, .footer {
        padding: 20px;
      }

      .header h1 {
        font-size: 22px;
      }

      .section h2 {
        font-size: 18px;
      }
    }
  </style>
</head>
<body>
  <div class="container">
    <div class="header">
      <h1>Proposta de Estratégia Personalizada</h1>
      <p>Análise Profissional para seu Negócio</p>
    </div>

    <div class="content">
      <!-- Informações do Cliente -->
      <div class="section">
        <div class="client-info">
          <p><strong>Cliente:</strong> ${data.clientName}</p>
          ${data.clientCompany ? `<p><strong>Empresa:</strong> ${data.clientCompany}</p>` : ''}
          <p><strong>Data da Proposta:</strong> ${new Date().toLocaleDateString('pt-BR')}</p>
        </div>
      </div>

      <!-- Resumo do Caso -->
      <div class="section">
        <h2>📋 Resumo do Caso</h2>
        <p>${data.caseDescription}</p>
      </div>

      <div class="divider"></div>

      <!-- Estratégia -->
      <div class="section">
        <h2>🎯 Estratégia Recomendada</h2>
        <div class="strategy-text">
          ${data.strategy}
        </div>
      </div>

      <!-- Recomendações -->
      <div class="section">
        <h2>💡 Recomendações Principais</h2>
        <ul class="recommendations">
          ${data.recommendations.map(rec => `<li>${rec}</li>`).join('')}
        </ul>
      </div>

      <div class="divider"></div>

      <!-- Timeline -->
      <div class="section">
        <h2>📅 Timeline de Implementação</h2>
        <div class="timeline">
          ${data.timeline}
        </div>
      </div>

      <!-- Orçamento -->
      <div class="section">
        <div class="budget-box">
          <div class="label">Orçamento Estimado</div>
          <div class="amount">R$ ${data.estimatedBudget.toLocaleString('pt-BR')}</div>
        </div>
      </div>

      <!-- Riscos -->
      ${data.risks && data.risks.length > 0 ? `
      <div class="section">
        <div class="risks">
          <h3>⚠️ Riscos Identificados</h3>
          <ul>
            ${data.risks.map(risk => `<li>${risk}</li>`).join('')}
          </ul>
        </div>
      </div>
      ` : ''}

      <div class="divider"></div>

      <!-- Próximos Passos -->
      <div class="section">
        <h2>🚀 Próximos Passos</h2>
        <p>Para implementarmos esta estratégia, agendamos uma reunião para discutir os detalhes e definir o cronograma de início.</p>
        <center>
          <a href="mailto:?subject=Agendamento%20de%20Reunião%20-%20Estratégia%20de%20Negócio" class="cta-button">
            Agendar Reunião
          </a>
        </center>
      </div>
    </div>

    <div class="footer">
      <p>Esta proposta foi gerada automaticamente com análise especializada.</p>
      <p style="color: #999; font-size: 12px; margin-top: 10px;">
        © ${new Date().getFullYear()} - Client Strategy Analyzer
      </p>
    </div>
  </div>
</body>
</html>
    `;
  }
}
