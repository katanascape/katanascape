use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111115");

const KILL_SWITCH_SPACE: usize = 8 + 32 + 1 + 32 + 8 + 1;

#[program]
pub mod kill_switch {
    use super::*;

    pub fn trigger_kill_switch(ctx: Context<TriggerKillSwitch>) -> Result<()> {
        let state = &mut ctx.accounts.kill_switch_state;
        state.agent_pubkey = ctx.accounts.agent.key();
        state.frozen = true;
        state.invoked_by = ctx.accounts.invoker.key();
        state.invoked_at = Clock::get()?.unix_timestamp;
        state.bump = ctx.bumps.kill_switch_state;

        emit!(KillSwitchTriggered {
            agent: state.agent_pubkey,
            invoked_by: state.invoked_by,
            invoked_at: state.invoked_at,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct TriggerKillSwitch<'info> {
    #[account(mut)]
    pub invoker: Signer<'info>,
    /// CHECK: Agent public key bound to this kill-switch PDA.
    pub agent: UncheckedAccount<'info>,
    // PDA seeds: ["kill-switch", agent_pubkey]
    #[account(
        init_if_needed,
        payer = invoker,
        space = KILL_SWITCH_SPACE,
        seeds = [b"kill-switch", agent.key().as_ref()],
        bump
    )]
    pub kill_switch_state: Account<'info, KillSwitchState>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct KillSwitchState {
    pub agent_pubkey: Pubkey,
    pub frozen: bool,
    pub invoked_by: Pubkey,
    pub invoked_at: i64,
    pub bump: u8,
}

#[event]
pub struct KillSwitchTriggered {
    pub agent: Pubkey,
    pub invoked_by: Pubkey,
    pub invoked_at: i64,
}
