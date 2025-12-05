import { EmailTemplate } from '@vizzio/core';
/**
 * Serviço de E-mails - Gerencia envio de emails
 */
export declare class EmailService {
    private transporter;
    constructor(smtpHost: string, smtpPort: number, smtpUser: string, smtpPassword: string);
    /**
     * Enviar email usando template
     */
    sendFromTemplate(to: string, template: EmailTemplate, variables: Record<string, any>): Promise<{
        success: boolean;
        messageId?: string;
    }>;
    /**
     * Enviar email simples
     */
    sendSimple(to: string, subject: string, html: string): Promise<boolean>;
    /**
     * Verificar conexão SMTP
     */
    verifyConnection(): Promise<boolean>;
}
//# sourceMappingURL=EmailService.d.ts.map