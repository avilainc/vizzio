/**
 * Integrações Externas - Salesforce, HubSpot, Slack, etc
 */

import axios from 'axios';
import { Integration } from '@vizzio/core';

export class SalesforceIntegration {
  private client: ReturnType<typeof axios.create>;

  constructor(instanceUrl: string, accessToken: string) {
    this.client = axios.create({
      baseURL: `${instanceUrl}/services/data/v57.0`,
      headers: {
        Authorization: `Bearer ${accessToken}`,
        'Content-Type': 'application/json',
      },
    });
  }

  /**
   * Sincronizar leads
   */
  async syncLeads(leads: any[]): Promise<any> {
    try {
      const response = await this.client.post('/sobjects/Lead/batch', {
        records: leads,
      });
      return response.data;
    } catch (error) {
      console.error('Erro ao sincronizar leads com Salesforce:', error);
      throw error;
    }
  }

  /**
   * Obter deals
   */
  async getDeals(): Promise<any[]> {
    try {
      const response = await this.client.get('/query?q=SELECT+Id,Name,Amount+FROM+Opportunity');
      return (response.data as any).records;
    } catch (error) {
      console.error('Erro ao obter deals do Salesforce:', error);
      throw error;
    }
  }
}

export class SlackIntegration {
  private webhookUrl: string;

  constructor(webhookUrl: string) {
    this.webhookUrl = webhookUrl;
  }

  /**
   * Enviar mensagem para canal Slack
   */
  async sendMessage(channel: string, message: string): Promise<boolean> {
    try {
      await axios.post(this.webhookUrl, {
        channel,
        text: message,
      });
      return true;
    } catch (error) {
      console.error('Erro ao enviar mensagem para Slack:', error);
      return false;
    }
  }

  /**
   * Enviar notificação formatada
   */
  async sendNotification(
    channel: string,
    title: string,
    message: string,
    color: string = '#0099ff'
  ): Promise<boolean> {
    try {
      await axios.post(this.webhookUrl, {
        channel,
        attachments: [
          {
            color,
            title,
            text: message,
            ts: Math.floor(Date.now() / 1000),
          },
        ],
      });
      return true;
    } catch (error) {
      console.error('Erro ao enviar notificação para Slack:', error);
      return false;
    }
  }
}

export class HubSpotIntegration {
  private client: ReturnType<typeof axios.create>;

  constructor(apiKey: string) {
    this.client = axios.create({
      baseURL: 'https://api.hubapi.com',
      headers: {
        Authorization: `Bearer ${apiKey}`,
        'Content-Type': 'application/json',
      },
    });
  }

  /**
   * Criar contato
   */
  async createContact(email: string, properties: Record<string, any>): Promise<any> {
    try {
      const response = await this.client.post('/crm/v3/objects/contacts', {
        properties: {
          email,
          ...properties,
        },
      });
      return response.data;
    } catch (error) {
      console.error('Erro ao criar contato no HubSpot:', error);
      throw error;
    }
  }

  /**
   * Obter contatos
   */
  async getContacts(limit: number = 10): Promise<any[]> {
    try {
      const response = await this.client.get('/crm/v3/objects/contacts', {
        params: { limit },
      });
      return (response.data as any).results;
    } catch (error) {
      console.error('Erro ao obter contatos do HubSpot:', error);
      throw error;
    }
  }
}
