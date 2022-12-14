use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod multi_chain_amm {
    use super::*;

    pub fn create_liquidity_acc(ctx: Context<CreateLiquidityAcc>) -> Result<()> {
        let liquidity_acc = &mut ctx.accounts.liquidity_acc;
        let bump = *ctx.bumps.get("liquidity-account").unwrap();

        liquidity_acc.eth = 0;
        liquidity_acc.sol = 0;
        liquidity_acc.usdc = 0;
        liquidity_acc.bump = bump;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateLiquidityAcc<'info> {
    #[account(init, payer = user, space = 8 + 1, seeds = [b"liquidity-account", user.key().as_ref()], bump)]
    pub liquidity_acc: Account<'info, LiquidityAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct LiquidityAccount {
    pub bump: u8,
    pub eth: u64,
    pub sol: u64,
    pub usdc: u64,
}
