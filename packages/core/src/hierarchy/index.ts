import { PublicKey } from "@solana/web3.js";
import { z } from "zod";

export const SpawnChildWalletParamsSchema = z.object({
  managerAgent: z.string().min(1),
  childName: z.string().min(1),
  budget: z.number().nonnegative()
});

export const AllocateBudgetParamsSchema = z.object({
  managerAgent: z.string().min(1),
  childWallet: z.string().min(1),
  amount: z.number().nonnegative()
});

export type SpawnChildWalletParams = z.infer<typeof SpawnChildWalletParamsSchema>;
export type AllocateBudgetParams = z.infer<typeof AllocateBudgetParamsSchema>;

export interface HierarchyResult {
  childWallet: string;
  transactionSignature: string;
  programId: PublicKey;
}

export async function spawnChildWallet(
  params: SpawnChildWalletParams
): Promise<HierarchyResult> {
  const validated = SpawnChildWalletParamsSchema.parse(params);

  return {
    childWallet: `${validated.managerAgent}-${validated.childName}-${Date.now()}`,
    transactionSignature: "stub-spawn-child-wallet-tx",
    programId: new PublicKey("11111111111111111111111111111114")
  };
}

export async function allocateBudget(
  params: AllocateBudgetParams
): Promise<HierarchyResult> {
  const validated = AllocateBudgetParamsSchema.parse(params);

  return {
    childWallet: validated.childWallet,
    transactionSignature: "stub-allocate-budget-tx",
    programId: new PublicKey("11111111111111111111111111111114")
  };
}
