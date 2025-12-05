export interface Workflow {
    id: string;
    name: string;
    description: string;
    trigger: string;
    actions: WorkflowAction[];
    status: 'active' | 'inactive';
    createdAt: Date;
    updatedAt: Date;
}
export interface WorkflowAction {
    id: string;
    type: 'email' | 'webhook' | 'database' | 'external' | 'conditional';
    config: Record<string, any>;
    order: number;
}
export interface EmailTemplate {
    id: string;
    name: string;
    subject: string;
    htmlContent: string;
    textContent: string;
    variables: string[];
    language: 'pt-BR' | 'en-US';
}
export interface Shortcut {
    id: string;
    name: string;
    type: 'keyboard' | 'voice' | 'gesture' | 'custom';
    binding: string;
    action: string;
    metadata: Record<string, any>;
}
export interface Integration {
    id: string;
    name: string;
    provider: string;
    isActive: boolean;
    credentials: Record<string, any>;
    config: Record<string, any>;
}
export interface AutomationResult {
    success: boolean;
    message: string;
    data?: any;
    error?: string;
    executedAt: Date;
}
//# sourceMappingURL=types.d.ts.map