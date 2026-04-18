import { z } from "zod";

export const InvoiceClientParamsSchema = z.object({
  agentWallet: z.string().min(1),
  clientAddress: z.string().min(1),
  amount: z.number().positive(),
  currency: z.string().min(1)
});

export const PayForApiParamsSchema = z.object({
  agentWallet: z.string().min(1),
  apiEndpoint: z.string().min(1),
  amount: z.number().positive()
});

export const ReceivePaymentParamsSchema = z.object({
  agentWallet: z.string().min(1)
});

export type InvoiceClientParams = z.infer<typeof InvoiceClientParamsSchema>;
export type PayForApiParams = z.infer<typeof PayForApiParamsSchema>;
export type ReceivePaymentParams = z.infer<typeof ReceivePaymentParamsSchema>;

export interface BillingResult {
  transactionSignature: string;
  invoiceId?: string;
}

export async function invoiceClient(
  params: InvoiceClientParams
): Promise<BillingResult> {
  const validated = InvoiceClientParamsSchema.parse(params);

  return {
    transactionSignature: "stub-invoice-client-tx",
    invoiceId: `${validated.agentWallet}-${Date.now()}`
  };
}

export async function payForAPI(params: PayForApiParams): Promise<BillingResult> {
  PayForApiParamsSchema.parse(params);

  return {
    transactionSignature: "stub-pay-for-api-tx"
  };
}

export async function receivePayment(
  params: ReceivePaymentParams
): Promise<BillingResult> {
  ReceivePaymentParamsSchema.parse(params);

  return {
    transactionSignature: "stub-receive-payment-tx"
  };
}