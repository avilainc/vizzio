import nodemailer from 'nodemailer';

export interface EmailOptions {
  to: string;
  subject: string;
  html: string;
  clientName: string;
}

export class EmailService {
  private transporter: nodemailer.Transporter;

  constructor(
    host: string,
    port: number,
    user: string,
    password: string
  ) {
    this.transporter = nodemailer.createTransport({
      host,
      port,
      secure: port === 465,
      auth: {
        user,
        pass: password,
      },
    });
  }

  async sendProposal(options: EmailOptions): Promise<boolean> {
    try {
      const mailOptions = {
        from: process.env.EMAIL_USER,
        to: options.to,
        subject: options.subject,
        html: options.html,
      };

      const info = await this.transporter.sendMail(mailOptions);
      console.log('Email enviado:', info.messageId);
      return true;
    } catch (error) {
      console.error('Erro ao enviar email:', error);
      throw error;
    }
  }

  async verifyConnection(): Promise<boolean> {
    try {
      await this.transporter.verify();
      console.log('Conexão de email verificada com sucesso');
      return true;
    } catch (error) {
      console.error('Erro ao verificar conexão de email:', error);
      return false;
    }
  }
}
