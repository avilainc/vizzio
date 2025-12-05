import nodemailer from 'nodemailer';
import { EmailTemplate } from '@vizzio/core';

/**
 * Serviço de E-mails - Gerencia envio de emails
 */
export class EmailService {
  private transporter: nodemailer.Transporter;

  constructor(
    smtpHost: string,
    smtpPort: number,
    smtpUser: string,
    smtpPassword: string
  ) {
    this.transporter = nodemailer.createTransport({
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
  async sendFromTemplate(
    to: string,
    template: EmailTemplate,
    variables: Record<string, any>
  ): Promise<{ success: boolean; messageId?: string }> {
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
    } catch (error) {
      console.error('Erro ao enviar email:', error);
      return { success: false };
    }
  }

  /**
   * Enviar email simples
   */
  async sendSimple(to: string, subject: string, html: string): Promise<boolean> {
    try {
      await this.transporter.sendMail({
        from: process.env.EMAIL_FROM,
        to,
        subject,
        html,
      });
      return true;
    } catch (error) {
      console.error('Erro ao enviar email simples:', error);
      return false;
    }
  }

  /**
   * Verificar conexão SMTP
   */
  async verifyConnection(): Promise<boolean> {
    try {
      await this.transporter.verify();
      return true;
    } catch (error) {
      console.error('Erro ao verificar conexão SMTP:', error);
      return false;
    }
  }
}
