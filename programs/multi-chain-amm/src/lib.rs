use anchor_lang::prelude::*;
use anchor_spl::token::{self, initialize_account , InitializeAccount,Mint, Burn, MintTo, TokenAccount, Transfer, initialize_mint};
use anchor_spl::{self};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod multi_chain_amm {
    use super::*;

    pub fn create_liquidity_acc(ctx: Context<CreateLiquidityAcc>) -> Result<()> {
        let liquidity_acc = &mut ctx.accounts.liquidity_acc;
        let token_program = ctx.accounts.token_program.to_account_info();
        let user = ctx.accounts.user.to_account_info();
        let bump = *ctx.bumps.get("liquidity-account").unwrap();
        let eth_token_mint = &ctx.accounts.eth_token_mint;
        let eth_token_acc = &ctx.accounts.eth_token_acc;

        let create_acc_accounts = InitializeAccount {
            account: eth_token_acc.to_account_info(),
            mint: eth_token_mint.to_account_info(),
            authority: liquidity_acc.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };

        initialize_account(CpiContext::new_with_signer(
            token_program,
            create_acc_accounts,
            &[&[b"liquidity-account",user.key().as_ref()]]
        ))?;

        liquidity_acc.eth = 0;
        liquidity_acc.sol = 0;
        liquidity_acc.bump = bump;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateLiquidityAcc<'info> {
    #[account(init, payer = user, space = 8 + 1 + 8 + 8, seeds = [b"liquidity-account", user.key().as_ref()], bump)]
    pub liquidity_acc: Account<'info, LiquidityAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: checked inside the instruct
    pub token_program: AccountInfo<'info>,
    /// CHECK: checked inside the instruct
    pub eth_token_mint: AccountInfo<'info>,
    pub eth_token_acc: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    #[account(
        init, 
        seeds = [b"mint".as_ref()], 
        bump,
        payer = user,
        mint::decimals = 0, 
        mint::authority = lp_mint
    )]
    pub lp_mint: Account<'info, Mint>,
}

#[account]
pub struct LiquidityAccount {
    pub bump: u8,
    pub eth: u64,
    pub sol: u64,
}
