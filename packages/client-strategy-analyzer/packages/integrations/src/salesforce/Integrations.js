"use strict";
/**
 * Integrações Externas - Salesforce, HubSpot, Slack, etc
 */
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.HubSpotIntegration = exports.SlackIntegration = exports.SalesforceIntegration = void 0;
const axios_1 = __importDefault(require("axios"));
class SalesforceIntegration {
    constructor(instanceUrl, accessToken) {
        this.client = axios_1.default.create({
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
    async syncLeads(leads) {
        try {
            const response = await this.client.post('/sobjects/Lead/batch', {
                records: leads,
            });
            return response.data;
        }
        catch (error) {
            console.error('Erro ao sincronizar leads com Salesforce:', error);
            throw error;
        }
    }
    /**
     * Obter deals
     */
    async getDeals() {
        try {
            const response = await this.client.get('/query?q=SELECT+Id,Name,Amount+FROM+Opportunity');
            return response.data.records;
        }
        catch (error) {
            console.error('Erro ao obter deals do Salesforce:', error);
            throw error;
        }
    }
}
exports.SalesforceIntegration = SalesforceIntegration;
class SlackIntegration {
    constructor(webhookUrl) {
        this.webhookUrl = webhookUrl;
    }
    /**
     * Enviar mensagem para canal Slack
     */
    async sendMessage(channel, message) {
        try {
            await axios_1.default.post(this.webhookUrl, {
                channel,
                text: message,
            });
            return true;
        }
        catch (error) {
            console.error('Erro ao enviar mensagem para Slack:', error);
            return false;
        }
    }
    /**
     * Enviar notificação formatada
     */
    async sendNotification(channel, title, message, color = '#0099ff') {
        try {
            await axios_1.default.post(this.webhookUrl, {
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
        }
        catch (error) {
            console.error('Erro ao enviar notificação para Slack:', error);
            return false;
        }
    }
}
exports.SlackIntegration = SlackIntegration;
class HubSpotIntegration {
    constructor(apiKey) {
        this.client = axios_1.default.create({
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
    async createContact(email, properties) {
        try {
            const response = await this.client.post('/crm/v3/objects/contacts', {
                properties: {
                    email,
                    ...properties,
                },
            });
            return response.data;
        }
        catch (error) {
            console.error('Erro ao criar contato no HubSpot:', error);
            throw error;
        }
    }
    /**
     * Obter contatos
     */
    async getContacts(limit = 10) {
        try {
            const response = await this.client.get('/crm/v3/objects/contacts', {
                params: { limit },
            });
            return response.data.results;
        }
        catch (error) {
            console.error('Erro ao obter contatos do HubSpot:', error);
            throw error;
        }
    }
}
exports.HubSpotIntegration = HubSpotIntegration;
//# sourceMappingURL=Integrations.js.map