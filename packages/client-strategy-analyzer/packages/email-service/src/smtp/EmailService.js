"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.EmailService = void 0;
const nodemailer_1 = __importDefault(require("nodemailer"));
/**
 * Serviço de E-mails - Gerencia envio de emails
 */
class EmailService {
    constructor(smtpHost, smtpPort, smtpUser, smtpPassword) {
        this.transporter = nodemailer_1.default.createTransport({
            host: smtpHost,
            port: smtpPort,
            secure: smtpPort === 465,
            auth: {
                user: smtpUser,
                pass: smtpPassword,
            },
        });
    }
    /**
     * Enviar email usando template
     */
    async sendFromTemplate(to, template, variables) {
        try {
            let htmlContent = template.htmlContent || template.html || '';
            let textContent = template.textContent || '';
            // Substituir variáveis
            for (const [key, value] of Object.entries(variables)) {
                const regex = new RegExp(`{{${key}}}`, 'g');
                htmlContent = htmlContent.replace(regex, String(value));
                textContent = textContent.replace(regex, String(value));
            }
            const info = await this.transporter.sendMail({
                from: process.env.EMAIL_FROM,
                to,
                subject: template.subject,
                text: textContent,
                html: htmlContent,
            });
            return { success: true, messageId: info.messageId };
        }
        catch (error) {
            console.error('Erro ao enviar email:', error);
            return { success: false };
        }
    }
    /**
     * Enviar email simples
     */
    async sendSimple(to, subject, html) {
        try {
            await this.transporter.sendMail({
                from: process.env.EMAIL_FROM,
                to,
                subject,
                html,
            });
            return true;
        }
        catch (error) {
            console.error('Erro ao enviar email simples:', error);
            return false;
        }
    }
    /**
     * Verificar conexão SMTP
     */
    async verifyConnection() {
        try {
            await this.transporter.verify();
            return true;
        }
        catch (error) {
            console.error('Erro ao verificar conexão SMTP:', error);
            return false;
        }
    }
}
exports.EmailService = EmailService;
//# sourceMappingURL=EmailService.js.map