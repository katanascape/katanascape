import { allocateBudget, spawnChildWallet } from "../packages/core/src";

async function main(): Promise<void> {
  const child = await spawnChildWallet({
    managerAgent: "manager-agent",
    childName: "worker-1",
    budget: 1_000
  });

  const budget = await allocateBudget({
    managerAgent: "manager-agent",
    childWallet: child.childWallet,
    amount: 500
  });

  console.log({ child, budget });
}

void main();