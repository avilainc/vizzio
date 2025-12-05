/**
 * Ferramentas Financeiras - Invoicing, Expenses, Payments
 */
/// <reference types="node" />
/// <reference types="node" />
export interface Invoice {
    id: string;
    number: string;
    client: string;
    amount: number;
    currency: string;
    dueDate: Date;
    items: InvoiceItem[];
    status: 'draft' | 'sent' | 'paid' | 'overdue';
}
export interface InvoiceItem {
    description: string;
    quantity: number;
    unitPrice: number;
    tax: number;
}
export declare class InvoiceService {
    /**
     * Gerar fatura
     */
    generateInvoice(invoiceData: Invoice): Promise<Buffer>;
    /**
     * Enviar fatura por email
     */
    sendInvoiceByEmail(invoice: Invoice, recipientEmail: string): Promise<boolean>;
    /**
     * Rastrear pagamento
     */
    trackPayment(invoiceId: string): Promise<{
        isPaid: boolean;
        paidAt?: Date;
    }>;
}
export declare class ExpenseService {
    /**
     * Registrar despesa
     */
    recordExpense(amount: number, category: string, description: string, receipt?: Buffer): Promise<string>;
    /**
     * Gerar relatório de despesas
     */
    generateExpenseReport(startDate: Date, endDate: Date): Promise<any>;
}
export declare class PaymentService {
    /**
     * Processar pagamento via Stripe
     */
    processPayment(amount: number, currency: string, stripeToken: string): Promise<{
        success: boolean;
        transactionId?: string;
    }>;
}
//# sourceMappingURL=FinanceTools.d.ts.map