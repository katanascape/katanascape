import { describe, expect, it } from "vitest";

import {
  AgentConfigSchema,
  ConsolidateRevenueParamsSchema,
  CreateEscrowParamsSchema,
  HoldNativeAssetParamsSchema,
  InitializeKillSwitchParamsSchema,
  InvoiceClientParamsSchema,
  PartialReleaseEscrowParamsSchema,
  PolicyRuleSchema,
  QueueKillSwitchParamsSchema,
  RevokeChildParamsSchema,
  SpawnChildWalletParamsSchema
} from "../../packages/core/src";

describe("katanaScape core schemas", () => {
  it("validates agent config", () => {
    const parsed = AgentConfigSchema.parse({
      name: "Demo Agent",
      chains: ["solana"],
      policyRules: []
    });

    expect(parsed.name).toBe("Demo Agent");
  });

  it("validates escrow params", () => {
    expect(
      CreateEscrowParamsSchema.parse({
        hirerAgent: "hirer",
        workerAgent: "worker",
        amount: 1,
        condition: "taskHash"
      }).amount
    ).toBe(1);
  });

  it("validates billing params", () => {
    const parsed = InvoiceClientParamsSchema.parse({
      agentWallet: "agent",
      clientAddress: "client",
      amount: 10,
      currency: "USDC"
    });

    expect(parsed.currency).toBe("USDC");
  });

  it("validates crosschain params", () => {
    const parsed = HoldNativeAssetParamsSchema.parse({
      agentWallet: "agent",
      chain: "solana",
      asset: "SOL"
    });

    expect(parsed.asset).toBe("SOL");
  });

  it("validates hierarchy params", () => {
    const parsed = SpawnChildWalletParamsSchema.parse({
      managerAgent: "manager",
      childName: "worker",
      budget: 100
    });

    expect(parsed.budget).toBe(100);
  });

  it("validates policy rule params", () => {
    const parsed = PolicyRuleSchema.parse({
      type: "spendingLimit",
      value: 100
    });

    expect(parsed.type).toBe("spendingLimit");
  });

  it("validates partial escrow release params", () => {
    const parsed = PartialReleaseEscrowParamsSchema.parse({
      escrowId: "escrow-1",
      amount: 5
    });

    expect(parsed.amount).toBe(5);
  });

  it("validates hierarchy management params", () => {
    const consolidated = ConsolidateRevenueParamsSchema.parse({
      managerAgent: "manager",
      childWallet: "child-wallet"
    });
    const revoked = RevokeChildParamsSchema.parse({
      managerAgent: "manager",
      childWallet: "child-wallet"
    });

    expect(consolidated.childWallet).toBe("child-wallet");
    expect(revoked.childWallet).toBe("child-wallet");
  });

  it("validates kill switch orchestration params", () => {
    const initialized = InitializeKillSwitchParamsSchema.parse({
      agentWallet: "agent-wallet",
      authority: "manager-wallet"
    });
    const queued = QueueKillSwitchParamsSchema.parse({
      agentWallet: "agent-wallet",
      invoker: "manager-wallet"
    });

    expect(initialized.authority).toBe("manager-wallet");
    expect(queued.invoker).toBe("manager-wallet");
  });
});