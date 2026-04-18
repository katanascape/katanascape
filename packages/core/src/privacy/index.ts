import { z } from "zod";

export const WrapWithPerParamsSchema = z.object({
  transaction: z.string().min(1)
});

export const EncryptStateParamsSchema = z.object({
  agentState: z.string().min(1)
});

export const DecryptWithDelegatedKeyParamsSchema = z.object({
  cipherState: z.string().min(1),
  delegatedKey: z.string().min(1)
});

export type WrapWithPerParams = z.infer<typeof WrapWithPerParamsSchema>;
export type EncryptStateParams = z.infer<typeof EncryptStateParamsSchema>;
export type DecryptWithDelegatedKeyParams = z.infer<typeof DecryptWithDelegatedKeyParamsSchema>;

export interface PrivacyResult {
  payload: string;
}

export async function wrapWithPER(params: WrapWithPerParams): Promise<PrivacyResult> {
  const validated = WrapWithPerParamsSchema.parse(params);

  return {
    payload: `wrapped:${validated.transaction}`
  };
}

export async function encryptState(params: EncryptStateParams): Promise<PrivacyResult> {
  const validated = EncryptStateParamsSchema.parse(params);

  return {
    payload: `encrypted:${validated.agentState}`
  };
}

export async function decryptWithDelegatedKey(
  params: DecryptWithDelegatedKeyParams
): Promise<PrivacyResult> {
  const validated = DecryptWithDelegatedKeyParamsSchema.parse(params);

  return {
    payload: `decrypted:${validated.cipherState}`
  };
}