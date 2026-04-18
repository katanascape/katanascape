import { triggerKillSwitch } from "../packages/core/src";

async function main(): Promise<void> {
  const result = await triggerKillSwitch({
    agentWallet: "agent-wallet",
    invoker: "manager-agent"
  });

  console.log(result);
}

void main();