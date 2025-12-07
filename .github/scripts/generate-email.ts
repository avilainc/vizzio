/**
 * VIZZIO - Email Notification Generator
 * Gera relatórios em HTML/CSS para notificação de builds
 *
 * Uso: npx ts-node generate-email.ts
 */

import * as fs from 'fs';
import * as path from 'path';

interface BuildData {
  workflowName: string;
  status: 'success' | 'failure' | 'cancelled';
  branch: string;
  commit: string;
  author: string;
  timestamp: string;
  duration: number;
  details: {
    testsRun?: number;
    testsPassed?: number;
    testsFailed?: number;
    coverage?: number;
    issues?: Array<{
      type: 'error' | 'warning';
      message: string;
      file?: string;
      line?: number;
    }>;
  };
}

/**
 * Gera HTML do email com CSS inline
 */
function generateEmailHTML(data: BuildData): string {
  const statusColor = {
    success: '#10b981',
    failure: '#ef4444',
    cancelled: '#f59e0b',
  }[data.status];

  const statusText = {
    success: '✅ Sucesso',
    failure: '❌ Falha',
    cancelled: '⚠️ Cancelado',
  }[data.status];

  const issuesHTML = (data.details.issues || [])
    .map(
      (issue) =>
        `
    <tr style="border-bottom: 1px solid #e5e7eb;">
      <td style="padding: 12px; color: ${issue.type === 'error' ? '#ef4444' : '#f59e0b'};">
        ${issue.type === 'error' ? '🔴' : '🟡'} ${issue.type.toUpperCase()}
      </td>
      <td style="padding: 12px; color: #374151;">${issue.message}</td>
      <td style="padding: 12px; color: #9ca3af; font-size: 12px;">
        ${issue.file ? `${issue.file}:${issue.line}` : 'N/A'}
      </td>
    </tr>
    `
    )
    .join('');

  const testStats =
    data.details.testsPassed !== undefined
      ? `
  <div style="background: #f3f4f6; padding: 12px; border-radius: 6px; margin-bottom: 16px;">
    <div style="display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px;">
      <div style="text-align: center; padding: 8px; background: white; border-radius: 4px;">
        <div style="font-size: 20px; font-weight: bold; color: #374151;">${data.details.testsRun}</div>
        <div style="font-size: 12px; color: #6b7280;">Testes</div>
      </div>
      <div style="text-align: center; padding: 8px; background: #dcfce7; border-radius: 4px;">
        <div style="font-size: 20px; font-weight: bold; color: #10b981;">${data.details.testsPassed}</div>
        <div style="font-size: 12px; color: #059669;">Passaram</div>
      </div>
      <div style="text-align: center; padding: 8px; background: #fee2e2; border-radius: 4px;">
        <div style="font-size: 20px; font-weight: bold; color: #ef4444;">${data.details.testsFailed}</div>
        <div style="font-size: 12px; color: #dc2626;">Falharam</div>
      </div>
      <div style="text-align: center; padding: 8px; background: white; border-radius: 4px;">
        <div style="font-size: 20px; font-weight: bold; color: #3b82f6;">${data.details.coverage}%</div>
        <div style="font-size: 12px; color: #1e40af;">Coverage</div>
      </div>
    </div>
  </div>
  `
      : '';

  return `
<!DOCTYPE html>
<html lang="pt-BR">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>VIZZIO Build Report - ${data.workflowName}</title>
  <style>
    * {
      margin: 0;
      padding: 0;
      box-sizing: border-box;
    }
    body {
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
      padding: 20px;
      min-height: 100vh;
    }
    .container {
      max-width: 800px;
      margin: 0 auto;
      background: white;
      border-radius: 12px;
      overflow: hidden;
      box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
    }
    .header {
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
      padding: 32px 24px;
      color: white;
    }
    .header h1 {
      font-size: 28px;
      margin-bottom: 8px;
    }
    .header p {
      font-size: 14px;
      opacity: 0.9;
    }
    .status-badge {
      display: inline-block;
      background: ${statusColor};
      color: white;
      padding: 8px 16px;
      border-radius: 6px;
      font-weight: 600;
      margin-top: 12px;
      font-size: 14px;
    }
    .content {
      padding: 32px 24px;
    }
    .section {
      margin-bottom: 24px;
    }
    .section h2 {
      font-size: 16px;
      color: #1f2937;
      margin-bottom: 12px;
      font-weight: 600;
      border-bottom: 2px solid #667eea;
      padding-bottom: 8px;
    }
    .info-grid {
      display: grid;
      grid-template-columns: repeat(2, 1fr);
      gap: 16px;
      margin-bottom: 16px;
    }
    .info-item {
      background: #f9fafb;
      padding: 12px;
      border-radius: 6px;
      border-left: 4px solid #667eea;
    }
    .info-label {
      font-size: 12px;
      color: #6b7280;
      text-transform: uppercase;
      letter-spacing: 0.5px;
      margin-bottom: 4px;
    }
    .info-value {
      font-size: 14px;
      color: #1f2937;
      font-weight: 500;
      font-family: 'Monaco', 'Menlo', monospace;
      word-break: break-all;
    }
    .issues-table {
      width: 100%;
      border-collapse: collapse;
      margin-top: 12px;
    }
    .issues-table th {
      background: #f3f4f6;
      padding: 12px;
      text-align: left;
      font-size: 12px;
      font-weight: 600;
      color: #374151;
      text-transform: uppercase;
      letter-spacing: 0.5px;
    }
    .footer {
      background: #f9fafb;
      padding: 24px;
      text-align: center;
      border-top: 1px solid #e5e7eb;
      font-size: 12px;
      color: #6b7280;
    }
    .cta-button {
      display: inline-block;
      background: #667eea;
      color: white;
      padding: 12px 24px;
      border-radius: 6px;
      text-decoration: none;
      font-weight: 600;
      margin-top: 16px;
      text-align: center;
      font-size: 14px;
    }
    .progress-bar {
      width: 100%;
      height: 8px;
      background: #e5e7eb;
      border-radius: 4px;
      overflow: hidden;
      margin-top: 8px;
    }
    .progress-fill {
      height: 100%;
      background: linear-gradient(90deg, #667eea, #764ba2);
      width: ${data.details.coverage || 0}%;
      transition: width 0.3s ease;
    }
  </style>
</head>
<body>
  <div class="container">
    <div class="header">
      <h1>🚀 VIZZIO Build Report</h1>
      <p>${new Date(data.timestamp).toLocaleString('pt-BR')}</p>
      <div class="status-badge">${statusText}</div>
    </div>

    <div class="content">
      <!-- Informações Básicas -->
      <div class="section">
        <h2>📋 Detalhes da Build</h2>
        <div class="info-grid">
          <div class="info-item">
            <div class="info-label">Workflow</div>
            <div class="info-value">${data.workflowName}</div>
          </div>
          <div class="info-item">
            <div class="info-label">Branch</div>
            <div class="info-value">${data.branch}</div>
          </div>
          <div class="info-item">
            <div class="info-label">Commit</div>
            <div class="info-value">${data.commit.substring(0, 8)}</div>
          </div>
          <div class="info-item">
            <div class="info-label">Autor</div>
            <div class="info-value">${data.author}</div>
          </div>
          <div class="info-item">
            <div class="info-label">Duração</div>
            <div class="info-value">${data.duration}s</div>
          </div>
          <div class="info-item">
            <div class="info-label">Status</div>
            <div class="info-value" style="color: ${statusColor};">${statusText}</div>
          </div>
        </div>
      </div>

      ${testStats ? `<div class="section"><h2>🧪 Resultados de Testes</h2>${testStats}</div>` : ''}

      <!-- Issues/Erros -->
      ${
        (data.details.issues || []).length > 0
          ? `
      <div class="section">
        <h2>⚠️ Issues Encontradas (${data.details.issues!.length})</h2>
        <table class="issues-table">
          <thead>
            <tr>
              <th>Tipo</th>
              <th>Mensagem</th>
              <th>Localização</th>
            </tr>
          </thead>
          <tbody>
            ${issuesHTML}
          </tbody>
        </table>
      </div>
      `
          : ''
      }

      <!-- Call to Action -->
      <div style="text-align: center; padding-top: 24px; border-top: 1px solid #e5e7eb;">
        <p style="color: #6b7280; margin-bottom: 16px; font-size: 14px;">
          Clique abaixo para ver o log completo no GitHub Actions
        </p>
        <a href="${process.env.GITHUB_SERVER_URL}/${process.env.GITHUB_REPOSITORY}/actions/runs/${process.env.GITHUB_RUN_ID}" class="cta-button">
          🔍 Ver Detalhes Completos
        </a>
      </div>
    </div>

    <div class="footer">
      <p><strong>VIZZIO Automation Platform</strong> | Build #${process.env.GITHUB_RUN_NUMBER || 'N/A'}</p>
      <p style="margin-top: 8px; opacity: 0.7;">Este é um email automático. Não responda diretamente.</p>
    </div>
  </div>
</body>
</html>
  `.trim();
}

/**
 * Salva o HTML gerado
 */
function saveEmail(html: string, filename: string = 'build-report.html'): void {
  const outputPath = path.join(__dirname, filename);
  fs.writeFileSync(outputPath, html, 'utf-8');
  console.log(`✅ Email HTML salvo: ${outputPath}`);
}

/**
 * Função principal
 */
function main(): void {
  // Dados de exemplo (serão substituídos por variáveis do GitHub Actions)
  const buildData: BuildData = {
    workflowName: process.env.WORKFLOW_NAME || 'CI/CD Pipeline',
    status: (process.env.JOB_STATUS || 'success') as 'success' | 'failure' | 'cancelled',
    branch: process.env.GITHUB_REF_NAME || 'main',
    commit: process.env.GITHUB_SHA || '1234567890abcdef',
    author: process.env.GITHUB_ACTOR || 'Unknown',
    timestamp: new Date().toISOString(),
    duration: parseInt(process.env.JOB_DURATION || '120'),
    details: {
      testsRun: parseInt(process.env.TESTS_RUN || '42'),
      testsPassed: parseInt(process.env.TESTS_PASSED || '40'),
      testsFailed: parseInt(process.env.TESTS_FAILED || '2'),
      coverage: parseInt(process.env.TEST_COVERAGE || '87'),
      issues:
        process.env.ISSUES_JSON && process.env.ISSUES_JSON !== ''
          ? JSON.parse(process.env.ISSUES_JSON)
          : [
              {
                type: 'error',
                message: 'Falha no teste de integração',
                file: 'src/api/handler.ts',
                line: 45,
              },
              {
                type: 'warning',
                message: 'Função deprecated detectada',
                file: 'src/utils/legacy.ts',
                line: 12,
              },
            ],
    },
  };

  const html = generateEmailHTML(buildData);
  saveEmail(html);

  // Também salva como variável de ambiente para o workflow usar
  fs.writeFileSync(path.join(__dirname, 'email-body.txt'), html, 'utf-8');
}

main();
