import { Keypair, PublicKey } from "@solana/web3.js";
import { z } from "zod";
import { KatanascapeError } from "../errors";

export const PolicyRuleSchema = z.object({
  type: z.enum([
    "spendingLimit",
    "recipientWhitelist",
    "chainRestriction",
    "timeLock",
    "velocityLimit",
    "escalationThreshold"
  ]),
  value: z.number().int().nonnegative(),
  recipient: z.string().optional(),
  chain: z.string().optional()
});

export const AgentConfigSchema = z.object({
  name: z.string().min(1),
  chains: z.array(z.string().min(1)).min(1),
  policyRules: z.array(PolicyRuleSchema).default([]),
  parentWallet: z.string().optional()
});

export type AgentConfig = z.infer<typeof AgentConfigSchema>;

export interface CreatedAgent {
  agentId: string;
  wallet: PublicKey;
  chains: string[];
}

export async function createAgent(config: AgentConfig): Promise<CreatedAgent> {
  const validated = AgentConfigSchema.parse(config);

  try {
    const wallet = Keypair.generate().publicKey;

    return {
      agentId: `${validated.name.toLowerCase().replace(/\s+/g, "-")}-${Date.now()}`,
      wallet,
      chains: validated.chains
    };
  } catch (error) {
    throw new KatanascapeError(
      "AGENT_CREATE_FAILED",
      `Failed to create agent: ${(error as Error).message}`
    );
  }
}
