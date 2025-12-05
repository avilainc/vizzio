/**
 * Integrações Externas - Salesforce, HubSpot, Slack, etc
 */
export declare class SalesforceIntegration {
    private client;
    constructor(instanceUrl: string, accessToken: string);
    /**
     * Sincronizar leads
     */
    syncLeads(leads: any[]): Promise<any>;
    /**
     * Obter deals
     */
    getDeals(): Promise<any[]>;
}
export declare class SlackIntegration {
    private webhookUrl;
    constructor(webhookUrl: string);
    /**
     * Enviar mensagem para canal Slack
     */
    sendMessage(channel: string, message: string): Promise<boolean>;
    /**
     * Enviar notificação formatada
     */
    sendNotification(channel: string, title: string, message: string, color?: string): Promise<boolean>;
}
export declare class HubSpotIntegration {
    private client;
    constructor(apiKey: string);
    /**
     * Criar contato
     */
    createContact(email: string, properties: Record<string, any>): Promise<any>;
    /**
     * Obter contatos
     */
    getContacts(limit?: number): Promise<any[]>;
}
//# sourceMappingURL=Integrations.d.ts.map