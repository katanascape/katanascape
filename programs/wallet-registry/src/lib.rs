use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111114");

const MAX_CHILDREN: usize = 32;
const WALLET_NODE_SPACE: usize = 8 + 32 + 1 + 32 + 4 + (MAX_CHILDREN * 32) + 8 + 8 + 1;

#[program]
pub mod wallet_registry {
    use super::*;

    pub fn register_root(ctx: Context<RegisterRoot>, budget: u64) -> Result<()> {
        let root_node = &mut ctx.accounts.root_node;
        root_node.pubkey = ctx.accounts.root_agent.key();
        root_node.parent = None;
        root_node.children = Vec::new();
        root_node.budget = budget;
        root_node.spent = 0;
        root_node.bump = ctx.bumps.root_node;

        emit!(RootRegistered {
            root: root_node.pubkey,
            budget,
        });

        Ok(())
    }

    pub fn spawn_child(
        ctx: Context<SpawnChild>,
        child_pubkey: Pubkey,
        child_budget: u64,
    ) -> Result<()> {
        let parent_node = &mut ctx.accounts.parent_node;
        require!(
            parent_node.children.len() < MAX_CHILDREN,
            WalletRegistryError::TooManyChildren
        );

        parent_node.children.push(child_pubkey);

        let child_node = &mut ctx.accounts.child_node;
        child_node.pubkey = child_pubkey;
        child_node.parent = Some(parent_node.pubkey);
        child_node.children = Vec::new();
        child_node.budget = child_budget;
        child_node.spent = 0;
        child_node.bump = ctx.bumps.child_node;

        emit!(ChildSpawned {
            parent: parent_node.pubkey,
            child: child_pubkey,
            budget: child_budget,
        });

        Ok(())
    }

    pub fn allocate_budget(
        ctx: Context<AllocateBudget>,
        child_pubkey: Pubkey,
        amount: u64,
    ) -> Result<()> {
        let parent_node = &ctx.accounts.parent_node;
        let child_node = &mut ctx.accounts.child_node;

        require!(
            child_node.parent == Some(parent_node.pubkey),
            WalletRegistryError::InvalidHierarchy
        );
        require_keys_eq!(
            child_node.pubkey,
            child_pubkey,
            WalletRegistryError::ChildMismatch
        );

        child_node.budget = amount;

        emit!(BudgetAllocated {
            parent: parent_node.pubkey,
            child: child_pubkey,
            budget: amount,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterRoot<'info> {
    #[account(mut)]
    pub root_agent: Signer<'info>,
    // PDA seeds: ["wallet-node", root_agent_pubkey]
    #[account(
        init,
        payer = root_agent,
        space = WALLET_NODE_SPACE,
        seeds = [b"wallet-node", root_agent.key().as_ref()],
        bump
    )]
    pub root_node: Account<'info, WalletNode>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(child_pubkey: Pubkey)]
pub struct SpawnChild<'info> {
    #[account(mut)]
    pub parent_agent: Signer<'info>,
    // PDA seeds: ["wallet-node", parent_agent_pubkey]
    #[account(
        mut,
        seeds = [b"wallet-node", parent_agent.key().as_ref()],
        bump = parent_node.bump,
        constraint = parent_node.pubkey == parent_agent.key() @ WalletRegistryError::InvalidParent
    )]
    pub parent_node: Account<'info, WalletNode>,
    // PDA seeds: ["wallet-node", child_pubkey]
    #[account(
        init,
        payer = parent_agent,
        space = WALLET_NODE_SPACE,
        seeds = [b"wallet-node", child_pubkey.as_ref()],
        bump
    )]
    pub child_node: Account<'info, WalletNode>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(child_pubkey: Pubkey)]
pub struct AllocateBudget<'info> {
    pub parent_agent: Signer<'info>,
    // PDA seeds: ["wallet-node", parent_agent_pubkey]
    #[account(
        seeds = [b"wallet-node", parent_agent.key().as_ref()],
        bump = parent_node.bump,
        constraint = parent_node.pubkey == parent_agent.key() @ WalletRegistryError::InvalidParent
    )]
    pub parent_node: Account<'info, WalletNode>,
    // PDA seeds: ["wallet-node", child_pubkey]
    #[account(
        mut,
        seeds = [b"wallet-node", child_pubkey.as_ref()],
        bump = child_node.bump
    )]
    pub child_node: Account<'info, WalletNode>,
}

#[account]
pub struct WalletNode {
    pub pubkey: Pubkey,
    pub parent: Option<Pubkey>,
    pub children: Vec<Pubkey>,
    pub budget: u64,
    pub spent: u64,
    pub bump: u8,
}

#[event]
pub struct RootRegistered {
    pub root: Pubkey,
    pub budget: u64,
}

#[event]
pub struct ChildSpawned {
    pub parent: Pubkey,
    pub child: Pubkey,
    pub budget: u64,
}

#[event]
pub struct BudgetAllocated {
    pub parent: Pubkey,
    pub child: Pubkey,
    pub budget: u64,
}

#[error_code]
pub enum WalletRegistryError {
    #[msg("Invalid parent signer")]
    InvalidParent,
    #[msg("Parent/child relationship is invalid")]
    InvalidHierarchy,
    #[msg("Child public key mismatch")]
    ChildMismatch,
    #[msg("Maximum child wallets reached")]
    TooManyChildren,
}
