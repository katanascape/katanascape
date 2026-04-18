import { payForAPI, wrapWithPER } from "../packages/core/src";

async function main(): Promise<void> {
  const wrapped = await wrapWithPER({ transaction: "api-request" });
  const payment = await payForAPI({
    agentWallet: "agent-wallet",
    apiEndpoint: "https://api.example.com",
    amount: 1
  });

  console.log({ wrapped, payment });
}

void main();