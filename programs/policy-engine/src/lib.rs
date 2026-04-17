use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

const MAX_RULES: usize = 16;
const POLICY_RULE_SIZE: usize = 96;
const POLICY_ACCOUNT_SPACE: usize = 8 + 32 + 4 + (MAX_RULES * POLICY_RULE_SIZE) + 1;

#[program]
pub mod policy_engine {
    use super::*;

    pub fn set_policies(ctx: Context<SetPolicies>, rules: Vec<PolicyRule>) -> Result<()> {
        require!(rules.len() <= MAX_RULES, PolicyEngineError::TooManyRules);

        let policy_account = &mut ctx.accounts.policy_account;
        policy_account.agent_pubkey = ctx.accounts.agent.key();
        policy_account.rules = rules;
        policy_account.bump = ctx.bumps.policy_account;

        emit!(PoliciesSet {
            agent: policy_account.agent_pubkey,
            rules_count: policy_account.rules.len() as u16,
        });

        Ok(())
    }

    pub fn validate_transaction(
        ctx: Context<ValidateTransaction>,
        amount: u64,
        recipient: Pubkey,
    ) -> Result<()> {
        let policy_account = &ctx.accounts.policy_account;
        require_keys_eq!(
            policy_account.agent_pubkey,
            ctx.accounts.agent.key(),
            PolicyEngineError::AgentMismatch
        );

        for rule in &policy_account.rules {
            match rule.policy_type {
                PolicyType::SpendingLimit => {
                    require!(amount <= rule.value, PolicyEngineError::SpendLimitExceeded);
                }
                PolicyType::RecipientWhitelist => {
                    let expected = rule
                        .recipient
                        .ok_or(PolicyEngineError::InvalidPolicyConfiguration)?;
                    require_keys_eq!(expected, recipient, PolicyEngineError::RecipientNotWhitelisted);
                }
                _ => {}
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetPolicies<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: Agent identity for policy binding.
    pub agent: UncheckedAccount<'info>,
    // PDA seeds: ["policy", agent_pubkey]
    #[account(
        init_if_needed,
        payer = authority,
        space = POLICY_ACCOUNT_SPACE,
        seeds = [b"policy", agent.key().as_ref()],
        bump
    )]
    pub policy_account: Account<'info, PolicyAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ValidateTransaction<'info> {
    pub agent: Signer<'info>,
    // PDA seeds: ["policy", agent_pubkey]
    #[account(
        seeds = [b"policy", agent.key().as_ref()],
        bump = policy_account.bump
    )]
    pub policy_account: Account<'info, PolicyAccount>,
}

#[account]
pub struct PolicyAccount {
    pub agent_pubkey: Pubkey,
    pub rules: Vec<PolicyRule>,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum PolicyType {
    SpendingLimit,
    RecipientWhitelist,
    ChainRestriction,
    TimeLock,
    VelocityLimit,
    EscalationThreshold,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PolicyRule {
    pub policy_type: PolicyType,
    pub value: u64,
    pub recipient: Option<Pubkey>,
    pub chain: Option<u16>,
}

#[event]
pub struct PoliciesSet {
    pub agent: Pubkey,
    pub rules_count: u16,
}

#[error_code]
pub enum PolicyEngineError {
    #[msg("Spend limit exceeded")]
    SpendLimitExceeded,
    #[msg("Recipient is not allowlisted")]
    RecipientNotWhitelisted,
    #[msg("Policy account does not match provided agent")]
    AgentMismatch,
    #[msg("Invalid policy configuration")]
    InvalidPolicyConfiguration,
    #[msg("Too many policy rules")]
    TooManyRules,
}
