import { PublicKey } from "@solana/web3.js";
import { z } from "zod";

export const PolicyTypeSchema = z.enum([
  "spendingLimit",
  "recipientWhitelist",
  "chainRestriction",
  "timeLock",
  "velocityLimit",
  "escalationThreshold"
]);

export const PolicySchema = z.object({
  type: PolicyTypeSchema,
  value: z.number().int().nonnegative(),
  recipient: z.string().optional(),
  chain: z.string().optional()
});

export const SetPoliciesParamsSchema = z.object({
  agentWallet: z.string().min(1),
  rules: z.array(PolicySchema)
});

export const TriggerKillSwitchParamsSchema = z.object({
  agentWallet: z.string().min(1),
  invoker: z.string().min(1)
});

export type SetPoliciesParams = z.infer<typeof SetPoliciesParamsSchema>;
export type TriggerKillSwitchParams = z.infer<typeof TriggerKillSwitchParamsSchema>;

export interface GrcTxResult {
  transactionSignature: string;
  programId: PublicKey;
}

export async function setPolicies(params: SetPoliciesParams): Promise<GrcTxResult> {
  SetPoliciesParamsSchema.parse(params);

  return {
    transactionSignature: "stub-set-policies-tx",
    programId: new PublicKey("11111111111111111111111111111112")
  };
}

export async function triggerKillSwitch(params: TriggerKillSwitchParams): Promise<GrcTxResult> {
  TriggerKillSwitchParamsSchema.parse(params);

  return {
    transactionSignature: "stub-trigger-kill-switch-tx",
    programId: new PublicKey("11111111111111111111111111111115")
  };
}
