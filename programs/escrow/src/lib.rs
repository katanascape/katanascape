use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111113");

const ESCROW_ACCOUNT_SPACE: usize = 8 + 32 + 32 + 8 + 8 + 8 + 8 + 1 + 1;

#[program]
pub mod escrow {
    use super::*;

    pub fn create_escrow(
        ctx: Context<CreateEscrow>,
        escrow_id: u64,
        amount: u64,
        condition: EscrowCondition,
    ) -> Result<()> {
        require!(amount > 0, EscrowError::InvalidAmount);

        let escrow_account = &mut ctx.accounts.escrow_account;
        escrow_account.hirer = ctx.accounts.hirer.key();
        escrow_account.worker = ctx.accounts.worker.key();
        escrow_account.escrow_id = escrow_id;
        escrow_account.amount = amount;
        escrow_account.released_amount = 0;
        escrow_account.condition = condition;
        escrow_account.state = EscrowState::Locked;
        escrow_account.bump = ctx.bumps.escrow_account;

        emit!(EscrowCreated {
            escrow: escrow_account.key(),
            hirer: escrow_account.hirer,
            worker: escrow_account.worker,
            amount,
        });

        Ok(())
    }

    pub fn release_escrow(ctx: Context<ReleaseEscrow>) -> Result<()> {
        let amount = {
            let escrow_account = &ctx.accounts.escrow_account;
            escrow_account.amount.saturating_sub(escrow_account.released_amount)
        };

        partial_release(ctx, amount)
    }

    pub fn partial_release(ctx: Context<ReleaseEscrow>, amount: u64) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;
        require!(
            escrow_account.state == EscrowState::Locked,
            EscrowError::InvalidEscrowState
        );
        require_keys_eq!(
            escrow_account.worker,
            ctx.accounts.worker.key(),
            EscrowError::UnauthorizedSigner
        );
        require!(amount > 0, EscrowError::InvalidAmount);

        let remaining = escrow_account
            .amount
            .checked_sub(escrow_account.released_amount)
            .ok_or(EscrowError::InvalidReleaseAmount)?;
        require!(amount <= remaining, EscrowError::InvalidReleaseAmount);

        escrow_account.released_amount = escrow_account
            .released_amount
            .checked_add(amount)
            .ok_or(EscrowError::InvalidReleaseAmount)?;
        if escrow_account.released_amount == escrow_account.amount {
            escrow_account.state = EscrowState::Released;
        }

        emit!(EscrowReleased {
            escrow: escrow_account.key(),
            worker: escrow_account.worker,
            amount,
        });

        Ok(())
    }

    pub fn cancel_escrow(ctx: Context<CancelEscrow>) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;
        require!(
            escrow_account.state == EscrowState::Locked,
            EscrowError::InvalidEscrowState
        );
        require_keys_eq!(
            escrow_account.hirer,
            ctx.accounts.hirer.key(),
            EscrowError::UnauthorizedSigner
        );

        escrow_account.state = EscrowState::Cancelled;

        emit!(EscrowCancelled {
            escrow: escrow_account.key(),
            hirer: escrow_account.hirer,
        });

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(escrow_id: u64)]
pub struct CreateEscrow<'info> {
    #[account(mut)]
    pub hirer: Signer<'info>,
    /// CHECK: Worker identity for escrow counterparty.
    pub worker: UncheckedAccount<'info>,
    // PDA seeds: ["escrow", hirer_pubkey, worker_pubkey, escrow_id_le_bytes]
    #[account(
        init,
        payer = hirer,
        space = ESCROW_ACCOUNT_SPACE,
        seeds = [
            b"escrow",
            hirer.key().as_ref(),
            worker.key().as_ref(),
            escrow_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReleaseEscrow<'info> {
    pub worker: Signer<'info>,
    #[account(mut)]
    pub escrow_account: Account<'info, EscrowAccount>,
}

#[derive(Accounts)]
pub struct CancelEscrow<'info> {
    pub hirer: Signer<'info>,
    #[account(mut)]
    pub escrow_account: Account<'info, EscrowAccount>,
}

#[account]
pub struct EscrowAccount {
    pub hirer: Pubkey,
    pub worker: Pubkey,
    pub escrow_id: u64,
    pub amount: u64,
    pub released_amount: u64,
    pub condition: EscrowCondition,
    pub state: EscrowState,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum EscrowCondition {
    TaskHash([u8; 32]),
    Oracle(Pubkey),
    TimeBased(i64),
    MultiSigApproval(u8),
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EscrowState {
    Locked,
    Released,
    Cancelled,
}

#[event]
pub struct EscrowCreated {
    pub escrow: Pubkey,
    pub hirer: Pubkey,
    pub worker: Pubkey,
    pub amount: u64,
}

#[event]
pub struct EscrowReleased {
    pub escrow: Pubkey,
    pub worker: Pubkey,
    pub amount: u64,
}

#[event]
pub struct EscrowCancelled {
    pub escrow: Pubkey,
    pub hirer: Pubkey,
}

#[error_code]
pub enum EscrowError {
    #[msg("Escrow amount must be greater than zero")]
    InvalidAmount,
    #[msg("Escrow release amount is invalid")]
    InvalidReleaseAmount,
    #[msg("Unauthorized signer for escrow action")]
    UnauthorizedSigner,
    #[msg("Escrow state does not allow this operation")]
    InvalidEscrowState,
}
