import { PublicKey } from "@solana/web3.js";
import { z } from "zod";

import { KatanascapeError } from "../errors";

export const CreateEscrowParamsSchema = z.object({
  hirerAgent: z.string().min(1),
  workerAgent: z.string().min(1),
  amount: z.number().positive(),
  condition: z.enum(["taskHash", "oracle", "timeBased", "multiSigApproval"])
});

export const ReleaseEscrowParamsSchema = z.object({
  escrowId: z.string().min(1)
});

export const PartialReleaseEscrowParamsSchema = z.object({
  escrowId: z.string().min(1),
  amount: z.number().positive()
});

export type CreateEscrowParams = z.infer<typeof CreateEscrowParamsSchema>;
export type ReleaseEscrowParams = z.infer<typeof ReleaseEscrowParamsSchema>;
export type PartialReleaseEscrowParams = z.infer<typeof PartialReleaseEscrowParamsSchema>;

export interface EscrowResult {
  escrowId: string;
  transactionSignature: string;
  programId: PublicKey;
}

export async function createEscrow(params: CreateEscrowParams): Promise<EscrowResult> {
  const validated = CreateEscrowParamsSchema.parse(params);

  if (validated.amount <= 0) {
    throw new KatanascapeError("INVALID_ESCROW_AMOUNT", "Escrow amount must be positive");
  }

  return {
    escrowId: `${validated.hirerAgent}-${validated.workerAgent}-${Date.now()}`,
    transactionSignature: "stub-create-escrow-tx",
    programId: new PublicKey("11111111111111111111111111111113")
  };
}

export async function releaseEscrow(params: ReleaseEscrowParams): Promise<EscrowResult> {
  const validated = ReleaseEscrowParamsSchema.parse(params);

  return {
    escrowId: validated.escrowId,
    transactionSignature: "stub-release-escrow-tx",
    programId: new PublicKey("11111111111111111111111111111113")
  };
}

export async function partialReleaseEscrow(
  params: PartialReleaseEscrowParams
): Promise<EscrowResult> {
  const validated = PartialReleaseEscrowParamsSchema.parse(params);

  return {
    escrowId: validated.escrowId,
    transactionSignature: "stub-partial-release-escrow-tx",
    programId: new PublicKey("11111111111111111111111111111113")
  };
}
