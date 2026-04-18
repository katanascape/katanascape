use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111115");

const KILL_SWITCH_SPACE: usize = 8 + 32 + 32 + 1 + 1 + 32 + 8 + 8 + 1;

#[program]
pub mod kill_switch {
    use super::*;

    pub fn initialize_kill_switch(
        ctx: Context<InitializeKillSwitch>,
        authority: Pubkey,
    ) -> Result<()> {
        let state = &mut ctx.accounts.kill_switch_state;
        state.agent_pubkey = ctx.accounts.agent.key();
        state.authority = authority;
        state.frozen = false;
        state.pending = false;
        state.requested_by = Pubkey::default();
        state.requested_at_slot = 0;
        state.invoked_by = Pubkey::default();
        state.invoked_at = 0;
        state.bump = ctx.bumps.kill_switch_state;

        emit!(KillSwitchInitialized {
            agent: state.agent_pubkey,
            authority,
        });

        Ok(())
    }

    pub fn queue_kill_switch(ctx: Context<TriggerKillSwitch>) -> Result<()> {
        let state = &mut ctx.accounts.kill_switch_state;
        validate_invoker(
            ctx.accounts.invoker.key(),
            state.authority,
            state.agent_pubkey,
        )?;
        require!(!state.frozen, KillSwitchError::AlreadyFrozen);
        require!(!state.pending, KillSwitchError::KillSwitchAlreadyQueued);

        let clock = Clock::get()?;
        state.pending = true;
        state.requested_by = ctx.accounts.invoker.key();
        state.requested_at_slot = clock.slot;

        emit!(KillSwitchQueued {
            agent: state.agent_pubkey,
            requested_by: state.requested_by,
            requested_at_slot: state.requested_at_slot,
        });

        Ok(())
    }

    pub fn trigger_kill_switch(ctx: Context<TriggerKillSwitch>) -> Result<()> {
        let state = &mut ctx.accounts.kill_switch_state;
        validate_invoker(
            ctx.accounts.invoker.key(),
            state.authority,
            state.agent_pubkey,
        )?;
        require!(!state.frozen, KillSwitchError::AlreadyFrozen);
        require!(state.pending, KillSwitchError::KillSwitchNotQueued);

        let clock = Clock::get()?;
        require!(
            clock.slot > state.requested_at_slot,
            KillSwitchError::TimelockInEffect
        );

        state.pending = false;
        state.frozen = true;
        state.invoked_by = ctx.accounts.invoker.key();
        state.invoked_at = clock.unix_timestamp;

        emit!(KillSwitchTriggered {
            agent: state.agent_pubkey,
            invoked_by: state.invoked_by,
            invoked_at: state.invoked_at,
        });

        Ok(())
    }
}

fn validate_invoker(invoker: Pubkey, authority: Pubkey, agent: Pubkey) -> Result<()> {
    require!(
        invoker == authority || invoker == agent,
        KillSwitchError::UnauthorizedInvoker
    );

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeKillSwitch<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub agent: Signer<'info>,
    // PDA seeds: ["kill-switch", agent_pubkey]
    #[account(
        init,
        payer = payer,
        space = KILL_SWITCH_SPACE,
        seeds = [b"kill-switch", agent.key().as_ref()],
        bump
    )]
    pub kill_switch_state: Account<'info, KillSwitchState>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TriggerKillSwitch<'info> {
    #[account(mut)]
    pub invoker: Signer<'info>,
    /// CHECK: Agent public key bound to this kill-switch PDA.
    pub agent: UncheckedAccount<'info>,
    // PDA seeds: ["kill-switch", agent_pubkey]
    #[account(
        mut,
        seeds = [b"kill-switch", agent.key().as_ref()],
        bump = kill_switch_state.bump
    )]
    pub kill_switch_state: Account<'info, KillSwitchState>,
}

#[account]
pub struct KillSwitchState {
    pub agent_pubkey: Pubkey,
    pub authority: Pubkey,
    pub frozen: bool,
    pub pending: bool,
    pub requested_by: Pubkey,
    pub requested_at_slot: u64,
    pub invoked_by: Pubkey,
    pub invoked_at: i64,
    pub bump: u8,
}

#[event]
pub struct KillSwitchInitialized {
    pub agent: Pubkey,
    pub authority: Pubkey,
}

#[event]
pub struct KillSwitchQueued {
    pub agent: Pubkey,
    pub requested_by: Pubkey,
    pub requested_at_slot: u64,
}

#[event]
pub struct KillSwitchTriggered {
    pub agent: Pubkey,
    pub invoked_by: Pubkey,
    pub invoked_at: i64,
}

#[error_code]
pub enum KillSwitchError {
    #[msg("Invoker is not authorized to trigger the kill switch")]
    UnauthorizedInvoker,
    #[msg("Kill switch has already been queued")]
    KillSwitchAlreadyQueued,
    #[msg("Kill switch has not been queued")]
    KillSwitchNotQueued,
    #[msg("Kill switch timelock is still in effect")]
    TimelockInEffect,
    #[msg("Kill switch is already frozen")]
    AlreadyFrozen,
}
