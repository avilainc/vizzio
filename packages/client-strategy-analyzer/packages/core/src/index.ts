// Core exports
export interface Workflow {
  id: string;
  name: string;
  description: string;
  enabled: boolean;
}

export interface EmailTemplate {
  id: string;
  name: string;
  subject: string;
  html: string;
  htmlContent?: string;
  textContent?: string;
}

export interface Shortcut {
  id: string;
  type: string;
  binding: string;
  action: string;
  name?: string;
  metadata?: Record<string, any>;
}

export interface Integration {
  id: string;
  type: string;
  name: string;
}

export interface AutomationResult {
  workflowId: string;
  status: 'success' | 'failed' | 'pending';
  success?: boolean;
  message?: string;
  data?: any;
  error?: string;
  executedAt?: Date;
}

export interface WorkflowAction {
  type: string;
  config?: Record<string, any>;
}
