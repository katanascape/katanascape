import { createEscrow, releaseEscrow } from "../packages/core/src";

async function main(): Promise<void> {
  const escrow = await createEscrow({
    hirerAgent: "hirer-agent",
    workerAgent: "worker-agent",
    amount: 100,
    condition: "taskHash"
  });

  const released = await releaseEscrow({ escrowId: escrow.escrowId });
  console.log({ escrow, released });
}

void main();