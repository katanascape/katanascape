import { createAgent } from "../packages/core/src";

async function main(): Promise<void> {
  const agent = await createAgent({
    name: "Demo Agent",
    chains: ["solana"],
    policyRules: []
  });

  console.log("Created agent", agent);
}

void main();