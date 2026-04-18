import { z } from "zod";

export const HoldNativeAssetParamsSchema = z.object({
  agentWallet: z.string().min(1),
  chain: z.string().min(1),
  asset: z.string().min(1)
});

export const EvmTransferParamsSchema = z.object({
  agentWallet: z.string().min(1),
  toAddress: z.string().min(1),
  amount: z.number().positive()
});

export type HoldNativeAssetParams = z.infer<typeof HoldNativeAssetParamsSchema>;
export type EvmTransferParams = z.infer<typeof EvmTransferParamsSchema>;

export interface CrosschainResult {
  transactionSignature: string;
  asset?: string;
}

export async function holdNativeAsset(
  params: HoldNativeAssetParams
): Promise<CrosschainResult> {
  const validated = HoldNativeAssetParamsSchema.parse(params);

  return {
    transactionSignature: "stub-hold-native-asset-tx",
    asset: `${validated.chain}:${validated.asset}`
  };
}

export async function evmTransfer(params: EvmTransferParams): Promise<CrosschainResult> {
  EvmTransferParamsSchema.parse(params);

  return {
    transactionSignature: "stub-evm-transfer-tx"
  };
}