import { z } from "zod";

export const PolicyTypeSchema = z.enum([
  "spendingLimit",
  "recipientWhitelist",
  "chainRestriction",
  "timeLock",
  "velocityLimit",
  "escalationThreshold"
]);

export const AgentIdentitySchema = z.object({
  agentId: z.string().min(1),
  wallet: z.string().min(1)
});

export type PolicyType = z.infer<typeof PolicyTypeSchema>;
export type AgentIdentity = z.infer<typeof AgentIdentitySchema>;
