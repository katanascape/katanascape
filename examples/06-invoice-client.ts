import { invoiceClient, receivePayment } from "../packages/core/src";

async function main(): Promise<void> {
  const invoice = await invoiceClient({
    agentWallet: "agent-wallet",
    clientAddress: "client-address",
    amount: 250,
    currency: "USDC"
  });

  const payment = await receivePayment({ agentWallet: "agent-wallet" });
  console.log({ invoice, payment });
}

void main();