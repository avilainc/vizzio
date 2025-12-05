import { Workflow, AutomationResult } from '@vizzio/core';
/**
 * Motor de Workflows - Executa automações
 */
export declare class WorkflowEngine {
    private queue;
    constructor(redisUrl?: string);
    /**
     * Registra um novo workflow
     */
    registerWorkflow(workflow: Workflow): Promise<void>;
    /**
     * Executa um workflow
     */
    executeWorkflow(workflowId: string, triggerData: Record<string, any>): Promise<AutomationResult>;
    /**
     * Lista workflows ativos
     */
    listActiveWorkflows(): Promise<Workflow[]>;
    /**
     * Para um workflow
     */
    stopWorkflow(workflowId: string): Promise<void>;
}
//# sourceMappingURL=WorkflowEngine.d.ts.map