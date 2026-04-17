import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";

const LAMPORTS_PER_AIRDROP = 2_000_000_000;

async function airdrop(
  provider: anchor.AnchorProvider,
  pubkey: anchor.web3.PublicKey
): Promise<void> {
  const sig = await provider.connection.requestAirdrop(pubkey, LAMPORTS_PER_AIRDROP);
  await provider.connection.confirmTransaction(sig, "confirmed");
}

describe("katanascape-mvp", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const policyEngine = anchor.workspace.PolicyEngine as Program;
  const escrow = anchor.workspace.Escrow as Program;
  const walletRegistry = anchor.workspace.WalletRegistry as Program;
  const killSwitch = anchor.workspace.KillSwitch as Program;

  it("deploys all programs and runs the root->policy->escrow flow", async () => {
    expect(policyEngine.programId).to.not.equal(undefined);
    expect(escrow.programId).to.not.equal(undefined);
    expect(walletRegistry.programId).to.not.equal(undefined);
    expect(killSwitch.programId).to.not.equal(undefined);

    const rootAgent = anchor.web3.Keypair.generate();
    const workerAgent = anchor.web3.Keypair.generate();
    await airdrop(provider, rootAgent.publicKey);
    await airdrop(provider, workerAgent.publicKey);

    const [rootNodePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("wallet-node"), rootAgent.publicKey.toBuffer()],
      walletRegistry.programId
    );

    await walletRegistry.methods
      .registerRoot(new anchor.BN(1_000_000))
      .accounts({
        rootAgent: rootAgent.publicKey,
        rootNode: rootNodePda,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([rootAgent])
      .rpc();

    const [policyPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("policy"), rootAgent.publicKey.toBuffer()],
      policyEngine.programId
    );

    const spendingLimitRule = {
      policyType: { spendingLimit: {} },
      value: new anchor.BN(5_000_000),
      recipient: null,
      chain: null
    } as unknown;

    await policyEngine.methods
      .setPolicies([spendingLimitRule])
      .accounts({
        authority: provider.wallet.publicKey,
        agent: rootAgent.publicKey,
        policyAccount: policyPda,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .rpc();

    const escrowId = new anchor.BN(1);
    const [escrowPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        rootAgent.publicKey.toBuffer(),
        workerAgent.publicKey.toBuffer(),
        escrowId.toArrayLike(Buffer, "le", 8)
      ],
      escrow.programId
    );

    const condition = {
      taskHash: new Array(32).fill(7)
    } as unknown;

    await escrow.methods
      .createEscrow(escrowId, new anchor.BN(2_500_000), condition)
      .accounts({
        hirer: rootAgent.publicKey,
        worker: workerAgent.publicKey,
        escrowAccount: escrowPda,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([rootAgent])
      .rpc();

    await escrow.methods
      .releaseEscrow()
      .accounts({
        worker: workerAgent.publicKey,
        escrowAccount: escrowPda
      })
      .signers([workerAgent])
      .rpc();
  });
});
