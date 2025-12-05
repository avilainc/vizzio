/**
 * Automação de Marketing - Campanhas, Leads, Segmentação
 */
export interface Campaign {
    id: string;
    name: string;
    description: string;
    type: 'email' | 'sms' | 'push' | 'social';
    status: 'draft' | 'scheduled' | 'running' | 'completed';
    startDate: Date;
    endDate?: Date;
    targetAudience: string[];
    content: string;
    metrics: CampaignMetrics;
}
export interface CampaignMetrics {
    sent: number;
    opened: number;
    clicked: number;
    converted: number;
    openRate: number;
    clickRate: number;
    conversionRate: number;
}
export interface Lead {
    id: string;
    name: string;
    email: string;
    phone?: string;
    company?: string;
    source: string;
    score: number;
    stage: 'prospect' | 'qualified' | 'negotiation' | 'closed';
    createdAt: Date;
}
export declare class CampaignService {
    /**
     * Criar campanha
     */
    createCampaign(campaign: Campaign): Promise<string>;
    /**
     * Iniciar campanha
     */
    startCampaign(campaignId: string): Promise<boolean>;
    /**
     * Obter métricas
     */
    getCampaignMetrics(campaignId: string): Promise<CampaignMetrics>;
}
export declare class LeadService {
    /**
     * Importar leads
     */
    importLeads(leads: Lead[]): Promise<{
        imported: number;
        errors: string[];
    }>;
    /**
     * Atribuir score ao lead
     */
    scoreLeads(): Promise<void>;
    /**
     * Segmentar leads
     */
    segmentLeads(criteria: Record<string, any>): Promise<Lead[]>;
}
//# sourceMappingURL=MarketingAutomation.d.ts.map