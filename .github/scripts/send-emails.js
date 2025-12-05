const nodemailer = require('nodemailer');
const fs = require('fs');
const path = require('path');

// Configuração dos parceiros
const PARTNERS = [
  {
    email: process.env.PARTNER_1_EMAIL || 'nicolas@avila.inc',
    name: process.env.PARTNER_1_NAME || 'Nicolas'
  },
  {
    email: process.env.PARTNER_2_EMAIL || 'marcelosavazzi1@gmail.com',
    name: process.env.PARTNER_2_NAME || 'Marcelo Savazzi'
  },
  {
    email: process.env.PARTNER_3_EMAIL || 'rafaelochiussi@hotmail.com',
    name: process.env.PARTNER_3_NAME || 'Rafael Ochiussi'
  }
];

// Extrar informações do evento GitHub
const eventName = process.env.GITHUB_EVENT_NAME;
const repository = process.env.GITHUB_REPOSITORY;
const actor = process.env.GITHUB_ACTOR;
const ref = process.env.GITHUB_REF_NAME;
const sha = process.env.GITHUB_SHA;
const serverUrl = process.env.GITHUB_SERVER_URL;

// Ler o payload do evento
let eventPayload = {};
try {
  const eventPath = process.env.GITHUB_EVENT_PATH;
  if (eventPath && fs.existsSync(eventPath)) {
    eventPayload = JSON.parse(fs.readFileSync(eventPath, 'utf8'));
  }
} catch (e) {
  console.log('Não foi possível ler o payload do evento');
}

// Função para gerar o HTML do email
function generateEmailHTML(partner, eventInfo) {
  return `<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Notificação Vizzio</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            padding: 20px;
            line-height: 1.6;
        }
        .container {
            max-width: 600px;
            margin: 0 auto;
            background: white;
            border-radius: 12px;
            overflow: hidden;
            box-shadow: 0 10px 40px rgba(0,0,0,0.2);
        }
        .header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 30px 20px;
            text-align: center;
        }
        .header h1 {
            font-size: 28px;
            margin-bottom: 10px;
        }
        .header p {
            font-size: 14px;
            opacity: 0.9;
        }
        .content {
            padding: 30px 20px;
        }
        .greeting {
            font-size: 16px;
            margin-bottom: 20px;
            color: #333;
        }
        .badge {
            display: inline-block;
            padding: 8px 16px;
            border-radius: 20px;
            font-size: 12px;
            font-weight: bold;
            margin-bottom: 20px;
        }
        .badge.push {
            background: #e3f2fd;
            color: #1976d2;
        }
        .badge.pull_request {
            background: #f3e5f5;
            color: #7b1fa2;
        }
        .badge.issue {
            background: #fff3e0;
            color: #f57c00;
        }
        .info-block {
            background: #f5f5f5;
            border-left: 4px solid #667eea;
            padding: 15px;
            margin: 15px 0;
            border-radius: 4px;
        }
        .info-block label {
            font-weight: bold;
            color: #333;
            display: block;
            margin-bottom: 5px;
            font-size: 14px;
        }
        .info-block .value {
            color: #666;
            word-break: break-all;
            font-family: 'Courier New', monospace;
            font-size: 13px;
        }
        .btn {
            display: inline-block;
            padding: 12px 24px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white !important;
            text-decoration: none;
            border-radius: 6px;
            margin-top: 20px;
            font-weight: bold;
            transition: transform 0.2s;
        }
        .btn:hover {
            transform: translateY(-2px);
        }
        .footer {
            background: #f9f9f9;
            border-top: 1px solid #eee;
            padding: 20px;
            text-align: center;
            font-size: 12px;
            color: #999;
        }
        .footer a {
            color: #667eea;
            text-decoration: none;
        }
        .divider {
            border-top: 1px solid #eee;
            margin: 20px 0;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>📊 Vizzio Platform</h1>
            <p>Notificação de Atualização do Repositório</p>
        </div>

        <div class="content">
            <p class="greeting">Olá <strong>${partner.name}</strong>,</p>

            <p>Uma nova atualização foi realizada no repositório Vizzio Platform!</p>

            <span class="badge ${eventInfo.type}">
                ${eventInfo.badge}
            </span>

            ${eventInfo.details}

            <div class="divider"></div>

            <table style="width: 100%; font-size: 13px; margin: 15px 0;">
                <tr>
                    <td style="padding: 8px 0;"><strong>📦 Repositório:</strong></td>
                    <td style="padding: 8px 0; text-align: right;">${repository}</td>
                </tr>
                <tr>
                    <td style="padding: 8px 0;"><strong>⏰ Data:</strong></td>
                    <td style="padding: 8px 0; text-align: right;">${new Date().toLocaleString('pt-BR')}</td>
                </tr>
            </table>

            <a href="${eventInfo.link}" class="btn">Ver Detalhes no GitHub →</a>
        </div>

        <div class="footer">
            <p>© 2025 Vizzio Platform - Todos os direitos reservados</p>
            <p>Você está recebendo este email como sócio do projeto.</p>
        </div>
    </div>
</body>
</html>`;
}

// Função para preparar informações do evento
function prepareEventInfo() {
  let info = {
    type: eventName,
    badge: '📌 Evento',
    details: '',
    link: `${serverUrl}/${repository}`
  };

  if (eventName === 'push') {
    const commit = eventPayload.head_commit;
    const message = commit ? commit.message : 'Commit realizado';
    const author = commit ? commit.author.name : actor;

    info.badge = '📤 PUSH';
    info.details = `
      <div class="info-block">
        <label>👤 Autor:</label>
        <span class="value">${author}</span>
      </div>
      <div class="info-block">
        <label>🌿 Branch:</label>
        <span class="value">${ref}</span>
      </div>
      <div class="info-block">
        <label>📝 Mensagem:</label>
        <span class="value">${message}</span>
      </div>
      <div class="info-block">
        <label>🔗 Commit:</label>
        <span class="value">${sha.substring(0, 7)}</span>
      </div>
    `;
    info.link = `${serverUrl}/${repository}/commit/${sha}`;
  }

  if (eventName === 'pull_request') {
    const pr = eventPayload.pull_request;
    const action = eventPayload.action;
    const actionLabel = action === 'opened' ? '🆕 ABERTO' : action === 'closed' ? '✅ FECHADO' : '🔄 ATUALIZADO';

    info.badge = '🔀 PULL REQUEST';
    info.details = `
      <div class="info-block">
        <label>PR #${pr.number} - ${pr.title}</label>
        <span class="value">Status: ${actionLabel}</span>
      </div>
      <div class="info-block">
        <label>👤 Autor:</label>
        <span class="value">${pr.user.login}</span>
      </div>
      <div class="info-block">
        <label>📌 Para:</label>
        <span class="value">${pr.base.ref} ← ${pr.head.ref}</span>
      </div>
    `;
    info.link = pr.html_url;
  }

  if (eventName === 'issues') {
    const issue = eventPayload.issue;
    const action = eventPayload.action;
    const actionLabel = action === 'opened' ? '🆕 ABERTA' : '✅ FECHADA';

    info.badge = '⚠️ ISSUE';
    info.details = `
      <div class="info-block">
        <label>Issue #${issue.number}</label>
        <span class="value">${issue.title}</span>
      </div>
      <div class="info-block">
        <label>Status:</label>
        <span class="value">${actionLabel}</span>
      </div>
      <div class="info-block">
        <label>👤 Reportado por:</label>
        <span class="value">${issue.user.login}</span>
      </div>
    `;
    info.link = issue.html_url;
  }

  return info;
}

// Função principal
async function sendEmails() {
  try {
    // Configurar transportador SMTP
    const transporter = nodemailer.createTransport({
      host: process.env.SMTP_HOST || 'smtp.gmail.com',
      port: parseInt(process.env.SMTP_PORT || '587'),
      secure: process.env.SMTP_PORT === '465',
      auth: {
        user: process.env.SMTP_USER,
        pass: process.env.SMTP_PASSWORD
      }
    });

    // Preparar informações do evento
    const eventInfo = prepareEventInfo();

    console.log(`\n📧 Preparando notificações para evento: ${eventName}`);
    console.log(`📦 Repositório: ${repository}`);
    console.log(`👤 Ator: ${actor}`);

    // Enviar email para cada parceiro
    for (const partner of PARTNERS) {
      try {
        const html = generateEmailHTML(partner, eventInfo);

        const mailOptions = {
          from: process.env.SEND_FROM || 'noreply@vizzio.dev',
          to: partner.email,
          subject: `🔔 Vizzio Platform - ${eventInfo.badge} - Nova Atualização`,
          html: html,
          text: `Nova atualização no repositório ${repository}.\nVisit: ${eventInfo.link}`
        };

        const info = await transporter.sendMail(mailOptions);
        console.log(`✅ Email enviado para ${partner.name} (${partner.email})`);
        console.log(`   ID: ${info.messageId}`);
      } catch (error) {
        console.error(`❌ Erro ao enviar email para ${partner.name}:`, error.message);
      }
    }

    console.log('\n✅ Processo de notificação concluído!\n');
  } catch (error) {
    console.error('❌ Erro ao configurar transporte SMTP:', error);
    process.exit(1);
  }
}

// Executar
sendEmails().catch(error => {
  console.error('Erro fatal:', error);
  process.exit(1);
});
