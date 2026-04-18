# KatanaScape — GitHub Copilot Instructions

> Private, cross-chain payment infrastructure for autonomous AI agents.
> TypeScript SDK + Solana on-chain programs (Anchor/Rust).

---

## Project Overview

KatanaScape is a TypeScript SDK that gives AI agents a complete financial identity —
custody, privacy, payments, and cross-chain reach — in a single package.

It integrates four external protocols and introduces three novel on-chain primitives,
all deployed on Solana and written in Rust (Anchor framework).

---

## Monorepo Structure

```
katanascape/
├── packages/
│   ├── core/                    # @katanascape/core — main TypeScript SDK
│   │   ├── src/
│   │   │   ├── agent/           # Agent identity, wallet init, lifecycle
│   │   │   ├── escrow/          # Escrow client (calls on-chain program)
│   │   │   ├── grc/             # Kill switch + policy engine client
│   │   │   ├── hierarchy/       # Hierarchical wallet tree client
│   │   │   ├── billing/         # Dodo Payments integration (invoicing, x402)
│   │   │   ├── privacy/         # MagicBlock PER + Encrypt FHE wrappers
│   │   │   ├── crosschain/      # Ika dWallet + viem EVM helpers
│   │   │   └── index.ts
│   │   └── package.json
│   └── sdk-types/               # Shared Zod schemas + TypeScript types
│
├── programs/                    # Solana on-chain programs (Rust + Anchor)
│   ├── policy-engine/           # GRC policy rules enforced pre-signature
│   ├── escrow/                  # Agent-to-agent conditional escrow
│   ├── wallet-registry/         # Hierarchical wallet tree state
│   └── kill-switch/             # Emergency freeze + fund recovery
│
├── tests/
│   ├── unit/                    # Vitest unit tests (TypeScript)
│   └── integration/             # Anchor test suite (TypeScript + localnet)
│
├── examples/
│   ├── 01-init-agent.ts
│   ├── 02-private-payment.ts
│   ├── 03-agent-escrow.ts
│   ├── 04-kill-switch.ts
│   ├── 05-hierarchy.ts
│   └── 06-invoice-client.ts
│
├── Anchor.toml
├── Cargo.toml
└── package.json                 # Root workspace (pnpm)
```

---

## Tech Stack

### TypeScript / SDK Layer

| Layer | Library |
|---|---|
| Language | TypeScript 5.x, strict mode |
| Runtime | Node.js 20+ |
| Package manager | pnpm workspaces |
| Agent framework | LangChain.js / Vercel AI SDK |
| Solana RPC | `@solana/web3.js` v1 |
| Cross-chain custody | `@ika-xyz/sdk` (2PC-MPC dWallets) |
| Private execution | `@magicblock/ephemeral-rollups-sdk` |
| FHE privacy | `@encrypt-xyz/fhe` |
| Stablecoin billing | `axios` → Dodo Payments REST API |
| EVM interaction | `viem` |
| Validation | `zod` |
| Testing | Vitest |

### On-Chain / Rust Layer

| Layer | Tool |
|---|---|
| Framework | Anchor 0.30.x |
| Language | Rust (2021 edition) |
| Localnet | `solana-test-validator` |
| Testing | `anchor test` (TypeScript client) |
| Token program | SPL Token / Token-2022 |

---

## Core SDK Modules

### `agent/`
- `createAgent(config)` — provisions a dWallet via Ika, registers with policy engine
- `AgentConfig` — zod-validated config (name, chains, policyRules, parentWallet?)
- Each agent has a **sovereign dWallet** — not a developer hot wallet

### `escrow/`
- `createEscrow(hirerAgent, workerAgent, amount, condition)` — locks funds on-chain
- `releaseEscrow(escrowId)` — condition met → auto-release to worker
- `cancelEscrow(escrowId)` — deadline/fail → returns to hirer
- Conditions: `taskHash`, `oracle`, `timeBased`, `multiSigApproval`
- All activity routes through MagicBlock PER (private until settlement)
- Supports **partial releases** for milestone-based tasks

### `grc/`
- `setPolicies(agentWallet, rules[])` — writes policy rules on-chain
- `triggerKillSwitch(agentWallet, invoker)` — freezes agent, cancels escrows, revokes dWallet
- Policy types enforced on-chain before signing:
  - `spendingLimit` — per-tx / hourly / daily cap
  - `recipientWhitelist` — allowlisted destinations only
  - `chainRestriction` — lock to specific chains
  - `timeLock` — allowed transaction windows
  - `velocityLimit` — max txs per period
  - `escalationThreshold` — above X USDC → requires manager approval

### `hierarchy/`
- `spawnChildWallet(managerAgent, childConfig)` — creates child dWallet under manager
- `allocateBudget(managerAgent, childWallet, amount)` — child cannot exceed this
- `consolidateRevenue(managerAgent)` — sweeps leaf earnings up the tree
- `revokeChild(managerAgent, childWallet)` — triggers child kill switch, siblings unaffected
- Rules: budget delegation, policy inheritance (manager → child), upward escalation

### `billing/`
- `invoiceClient(agentWallet, clientAddress, amount, currency)` — creates Dodo Payments invoice
- `payForAPI(agentWallet, apiEndpoint, amount)` — x402 micropayment
- `receivePayment(agentWallet)` — USDC inbound listener
- Supports 220+ countries via Dodo Payments

### `privacy/`
- `wrapWithPER(transaction)` — routes tx through MagicBlock Ephemeral Rollup
- `encryptState(agentState)` — FHE encryption via Encrypt
- `decryptWithDelegatedKey(cipherState, delegatedKey)` — manager reads child state

### `crosschain/`
- `holdNativeAsset(agentWallet, chain, asset)` — BTC, ETH, SOL natively via Ika 2PC-MPC
- No bridges, no wrapped assets
- `evmTransfer(agentWallet, toAddress, amount)` — uses viem for EVM-side

---

## On-Chain Programs (Anchor / Rust)

### `policy-engine`
- Account: `PolicyAccount { agentPubkey, rules: Vec<PolicyRule>, bump }`
- Instruction: `set_policies`, `validate_transaction` (CPI hook before any tx)
- Error codes: `SpendLimitExceeded`, `RecipientNotWhitelisted`, `VelocityExceeded`, etc.

### `escrow`
- Account: `EscrowAccount { hirer, worker, amount, condition, state: EscrowState, bump }`
- States: `Locked → Released | Cancelled`
- Instructions: `create_escrow`, `release_escrow`, `cancel_escrow`, `partial_release`

### `wallet-registry`
- Account: `WalletNode { pubkey, parent: Option<Pubkey>, children: Vec<Pubkey>, budget, spent, bump }`
- Instructions: `register_root`, `spawn_child`, `allocate_budget`, `consolidate_revenue`, `revoke_child`

### `kill-switch`
- Account: `KillSwitchState { agentPubkey, frozen: bool, invokedBy, invokedAt, bump }`
- Instruction: `trigger_kill_switch` — time-locked by one block, emits `KillSwitchEvent`
- Invokers: developer, manager agent, DAO vote, circuit breaker

---

## Coding Conventions

### TypeScript
- **Strict mode** always (`"strict": true` in tsconfig)
- All public functions must have explicit return types
- Use **Zod** for all runtime validation at SDK boundaries
- Prefer `async/await` over Promise chains
- Name files in `kebab-case`, classes in `PascalCase`, functions in `camelCase`
- All errors must be typed: extend `KatanascapeError` base class
- Export a clean public API from each module's `index.ts`

### Rust / Anchor
- Use **Anchor account validation macros** (`#[account]`, `#[derive(Accounts)]`) — never manual deserialization
- All instructions must validate constraints with `#[account(constraint = ...)]`
- Custom errors via `#[error_code]` enum
- Emit events for all state-changing instructions via `emit!()`
- PDAs must use deterministic seeds documented in comments
- No `unwrap()` in production code — use `?` and `AnchorError`

### Testing
- Unit tests in `tests/unit/` with Vitest
- Integration tests with `anchor test` against localnet
- Every on-chain instruction needs at least: success case, unauthorized signer case, constraint violation case

---

## Environment Variables

```env
# Solana
SOLANA_RPC_URL=https://api.devnet.solana.com
ANCHOR_WALLET=~/.config/solana/id.json
ANCHOR_PROVIDER_URL=https://api.devnet.solana.com

# Ika
IKA_API_URL=
IKA_API_KEY=

# MagicBlock
MAGICBLOCK_RPC_URL=

# Encrypt
ENCRYPT_API_KEY=

# Dodo Payments
DODO_API_KEY=
DODO_WEBHOOK_SECRET=

# Optional: EVM (for cross-chain)
EVM_RPC_URL=
```

---

## Key Design Principles

1. **Non-human first** — every API is designed assuming no human in the loop
2. **Private by default** — all transactions route through MagicBlock PER; state is FHE-encrypted
3. **On-chain enforcement** — policies, escrow conditions, and kill switches live on Solana; no off-chain trust
4. **Sovereign identity** — agents get real dWallets via Ika, not borrowed developer keys
5. **Cross-chain native** — BTC, ETH, SOL via Ika 2PC-MPC; no bridges
6. **Hierarchical authority** — manager policies cascade down; cannot be overridden by children

---

## Anchor Workspace Notes

- `Anchor.toml` declares all four programs under `[programs.localnet]`
- Program IDs are placeholders until `anchor build` generates keypairs
- Use `anchor deploy --provider.cluster devnet` for devnet deployment
- IDL files are auto-generated in `target/idl/` — do not edit manually
- TypeScript types are generated in `target/types/` — import from there in SDK

---

## Common Patterns to Follow

### Creating a new Anchor instruction
```rust
pub fn my_instruction(ctx: Context<MyInstruction>, param: u64) -> Result<()> {
    let account = &mut ctx.accounts.my_account;
    // validate
    require!(param > 0, KatanascapeError::InvalidParam);
    // mutate
    account.value = param;
    // emit
    emit!(MyEvent { pubkey: account.key(), value: param });
    Ok(())
}
```

### Creating a new SDK function
```typescript
export async function myFunction(
  connection: Connection,
  agent: AgentWallet,
  params: MyParams // zod-validated
): Promise<MyResult> {
  const validated = MyParamsSchema.parse(params);
  // interact with on-chain program or protocol
  return result;
}
```

### Error handling
```typescript
// Always use typed errors
throw new KatanascapeError('POLICY_VIOLATION', `Agent ${agentId} exceeded spend limit`);
```

---

## Out of Scope (do not implement)

- Web frontend / UI
- Centralized database or off-chain state store
- Any custody solution other than Ika dWallets
- Bridges or wrapped assets for cross-chain
- Human-approval flows (contradicts autonomous design)



KatanaScape
Private, cross-chain payment infrastructure for autonomous AI agents.
KatanaScape is a TypeScript SDK that gives AI agents a complete financial identity — custody, privacy, payments, and cross-chain reach — in a single package. It is the first infrastructure built specifically for non-human economic actors operating autonomously on Solana.
Table of Contents
What Is KatanaScape
The Problem
Core Features
New Primitives
Escrow Between Agents
GRC, Kill Switch & Policies
Hierarchical Wallet System
How It Works
Architecture
Stack & Technologies
Code Examples
The Vision
What Is KatanaScape
KatanaScape integrates four specialized protocols into a single unified TypeScript SDK:
Ika — cross-chain MPC custody via dWallets
MagicBlock — private transaction execution via Ephemeral Rollups
Encrypt — FHE-based state and reasoning privacy
Dodo Payments — stablecoin billing, invoicing, and x402 micropayments
On top of this foundation, KatanaScape introduces three new financial primitives that have never existed for autonomous agents before: escrow, governance and kill switches, and hierarchical wallet trees.
The Problem
AI agents are becoming independent economic actors. They browse, decide, and execute tasks without human intervention. But when it comes to money, they are completely exposed:
They run on hot wallets with naked private keys belonging to developers
Every transaction is fully visible on-chain before execution, leaking intent
They are locked to a single chain, unable to hold BTC, ETH, and SOL natively
Every payment still requires human approval — contradicting autonomous design
There is no conditional trust between agents — no way to escrow value for task completion
There are no guardrails — a compromised or runaway agent can drain everything
There is no hierarchy — a manager agent cannot govern employee agents financially
There is no financial infrastructure built for agents. Everything they use today was designed for humans. KatanaScape fixes that from the ground up.
Core Features
Sovereign Financial Identity
Each agent gets its own dWallet via Ika — non-custodial, cross-chain, and owned entirely by the agent from creation. Not a borrowed developer wallet. Not a shared API key. A genuine financial identity.
Private by Default
Every transaction routes through MagicBlock's Private Ephemeral Rollup. Agent state and reasoning are encrypted via Encrypt FHE. No footprint until final settlement.
Cross-Chain Without Bridges
Ika's 2PC-MPC lets agents hold and transact native BTC, ETH, and SOL simultaneously — no bridges, no wrapped assets, no custodians.
Full Billing Lifecycle
Via Dodo Payments, agents invoice clients, receive USDC, pay for APIs via x402 micropayments, and manage subscriptions — across 220+ countries.
New Primitives
1. Escrow Between Agents
The Problem it Solves
When one agent hires another to complete a task, there is currently no trustless way to hold payment conditionally. The hiring agent either pays upfront (trusts blindly) or pays after (worker agent trusts blindly). There is no middle ground.
How It Works
KatanaScape introduces an on-chain escrow program on Solana. When Agent A hires Agent B:
Agent A locks funds into an escrow account tied to a verifiable task condition
Agent B executes the task
When the condition is met — verified on-chain or via oracle — funds release automatically to Agent B
If the condition fails or deadline passes, funds return to Agent A
Conditions can be: task hash verification, oracle data, time-based, or multi-sig approval from a manager agent.
Key Properties
Escrow is created, held, and released entirely on-chain — no human intermediary
All escrow activity routes through MagicBlock PER — private until settlement
Supports partial releases for milestone-based tasks
Natively integrated with the hierarchical wallet system — manager agents can override
2. GRC, Kill Switch & Policies
GRC = Governance, Risk & Compliance
The Problem it Solves
A compromised, runaway, or malfunctioning agent can cause irreversible financial damage. Today there is no standardized way to set financial policies for agents, monitor their behaviour in real time, or shut them down instantly without revoking all access manually.
How It Works
KatanaScape introduces a Policy Engine — an on-chain program that every agent wallet is governed by at creation. Policies are programmable rules enforced at the Solana level before any transaction is signed.
Policy Types
Policy
Description
Spending Limit
Max spend per transaction, per hour, per day
Recipient Whitelist
Agent can only send to pre-approved addresses
Chain Restriction
Agent locked to specific chains only
Time Lock
Transactions only allowed within defined windows
Velocity Limit
Max number of transactions per time period
Escalation Threshold
Transactions above X USDC require manager approval
Kill Switch
Every agent wallet has a kill switch — a privileged instruction that can be invoked by:
The deploying developer
A designated manager agent
A DAO governance vote
An automated circuit breaker triggered by anomaly detection
When triggered, the kill switch:
Immediately freezes all outbound transactions
Cancels all pending escrows and returns funds
Revokes signing authority on the dWallet
Emits an on-chain event log for audit
The kill switch itself is time-locked by one block to prevent front-running, and its invocation is permanently recorded on-chain.
Compliance Layer
For enterprise deployments, KatanaScape supports a compliance module:
Transaction logs encrypted and accessible only to the deployer via FHE
Regulatory reporting exports for jurisdictions requiring agent activity disclosure
Audit trails that prove what an agent did without revealing how it did it
3. Hierarchical Wallet System
The Problem it Solves
In any real-world deployment, agents don't operate alone. A manager agent coordinates multiple employee agents working in parallel. Today there is no native financial hierarchy — every agent is a flat, equal peer with no concept of authority, delegation, or budget allocation.
How It Works
KatanaScape introduces a tree-structured wallet hierarchy enforced on-chain.
​
Rules of the Hierarchy
Budget Delegation — Manager allocates a budget to each child. Children cannot exceed their allocated budget regardless of their wallet balance.
Policy Inheritance — Policies set at the manager level cascade down. A manager-level spending limit cannot be overridden by a child.
Upward Escalation — Transactions above a child's threshold automatically escalate to the parent for approval.
Revenue Consolidation — Earnings from leaf agents can be programmatically swept up to the manager wallet on schedule or on trigger.
Revocation — A manager can revoke a child wallet's signing authority instantly, triggering that wallet's kill switch without affecting siblings.
Cross-Branch Payments — Worker agents in different branches can pay each other via internal transfer, settled at the manager level without touching the public chain.
Manager Agent Capabilities
A manager agent has elevated dWallet permissions that allow it to:
Spawn new child wallets on demand
Adjust child budgets in real time
Read (but not alter) encrypted child state via FHE with delegated key
Trigger any child's kill switch
Consolidate earnings across the entire tree
How It Works
End-to-End Workflow
​
Architecture
​
Stack & Technologies
Core SDK
Layer
Technology
Language
TypeScript
Runtime
Node.js 20+
Package
npm / @KatanaScape/core
Agent Framework
LangChain.js / Vercel AI SDK
Blockchain & Protocols
Protocol
Purpose
Solana
Control plane, program execution, settlement
Ika (2PC-MPC)
dWallet creation, cross-chain custody
MagicBlock PER
Private Ephemeral Rollup, hidden execution
Encrypt (REFHE)
Fully Homomorphic Encryption, state privacy
Dodo Payments
Stablecoin billing, invoicing, x402 payments
Solana Programs (On-Chain)
Program
Stack
Policy Engine
Rust, Anchor Framework
Escrow Program
Rust, Anchor Framework
Hierarchical Wallet Registry
Rust, Anchor Framework
Kill Switch Module
Rust, Anchor Framework
Supporting Infrastructure
Tool
Purpose
Anchor
Solana program framework
web3.js / @solana/web3.js
Solana RPC interaction
@ika-xyz/sdk
Ika dWallet SDK
@magicblock/ephemeral-rollups-sdk
MagicBlock PER integration
@encrypt-xyz/fhe
Encrypt FHE client
axios
Dodo Payments REST API
zod
Runtime type validation
viem
EVM chain interaction for cross-chain
Code Examples
1. Initialize KatanaScape and Create an Agent
​
2. Agent Pays for an API Privately
​
3. Create an Escrow Between Two Agents
​
4. Kill Switch — Emergency Stopx++
​
5. Hierarchical Wallet — Manager Spawning Workers
​
6. Agent Invoices a Client and Receives Payment
​
The Vision
Every payment network ever built assumed a human was somewhere in the chain — approving, signing, reviewing. KatanaScape is built on the assumption that humans are not required.
Agents hire each other. Pay each other. Invoice each other. Govern each other. A fully closed economic loop between non-human actors — with escrow for trust, kill switches for safety, and hierarchy for coordination — running on Solana, private by default, cross-chain by design.
The agentic economy is coming. KatanaScape is its financial layer.

## The Agentic Financial Layer for the Solana Frontier Hackathon

### 1. Executive Summary:

Katanascape is an autonomous financial orchestration layer that provides AI agents with "Economic Sovereignty." It enables agents to independently lend, borrow and settle cross-chain payments using the x402 protocol and ERC-8004 identity standards. Katanascape acts as a "Universal Bridge" that ensures liquidity is used efficiently through a policy-driven governance engine.

### 2. Core Problem & Solution
The Problem:

AI agents (like Clawdbots) are currently financial orphans. They cannot pay for their own compute, data or API access without a human "signer" or a centralized credit card. The Solution: Katanascape provides a decentralized banking stack for machines. It allows agents to manage their own balance sheets, borrow against their reputation and transact at machine speed.

### 3. Technical Architecture (The Stack)

#### A. **The Identity Layer (ERC-8004)**
**Purpose**: ***To give every agent a verifiable on-chain "Credit Score***."

**Mechanism**: Every agent registers an NFT-based identity. Transaction history (repayments, successful x402 settlements) builds "**Reputation Points**." Benefit: High-reputation agents get lower collateral requirements for borrowing.

#### B. The Payment Rail (x402 Protocol)
Purpose: *Native Machine-to-Machine (M2M) commerce.*

**Mechanism**: When an agent hits a resource (API/GPU) that returns an **HTTP 402 "Payment Required" code,** the Katanascape facilitator automatically triggers a gasless USDC settlement on Solana. 

**Benefit:** Zero human intervention for per-request billing.

#### C. The Policy Engine (Anchor Program)
Purpose: *Guardrails against "Rogue AI" or liquidity drainage.*

**Mechanism:** A set of smart contracts that enforce: Spending 

**Velocity**: Max transactions per minute.    

**Debt-to-Equity Ratios**: Preventing agents from over-leveraging.    

**White-listed Protocols:** Restricting borrowing/lending to audited pools.

#### D. The Universal Bridge
Purpose: *Frictionless movement of capital across chains (Base, Solana, Ethereum).*

**Mechanism:** Uses "Just-in-Time" (JIT) liquidity to move only the exact amount required for an agent’s transaction, minimizing exposure.

### 4. Key Use Cases

1. Autonomous SaaS Agents: An AI agent that buys its own OpenAI API credits when it runs low, borrowing USDC from a lending pool to cover the gap.   

2. Cross-Chain Arbitrage Bots: Agents that move liquidity between Solana and Base to capture yield, governed by Katanascape risk policies.   

3. Autonomous Researchers: Agents that pay for paywalled academic papers or data sets using x402 micropayments.

### 5. Development Roadmap (4-Week Sprint)

| Week | Phase | Milestone

|| Week 1 | Foundations | Identity Registry (ERC-8004) & Anchor Environment Setup. 

|| Week 2 | Payments | x402 Facilitator integration & Autonomous API payment loop. 

|| Week 3 | DeFi & Policy | Lending/Borrowing logic & Liquidity Guardrails contract. 

|| Week 4 | Polish & Pitch | React Dashboard for Monitoring & Final Colosseum Submission.

### 6. Strategic Value for Solana

Katanascape leverages Solana’s low latency and high throughput to support the "Agentic Economy." While other chains are too slow for machine-speed high-frequency payments, Katanascape on Solana allows for sub-penny, sub-second transactions that make AI autonomy viable.

#### 🛡️ Guardrail Policy (The "No Unnecessary Usage" Rule)To protect the liquidity providers (LPs), Katanascape implements a circuit-breaker.

If an agent’s "Reputation Score" drops below a threshold or it attempts a transaction that violates its pre-set spending policy, the account is frozen, and the remaining liquidity is returned to the pool.