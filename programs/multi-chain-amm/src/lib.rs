use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, initialize_account, initialize_mint, Burn, InitializeAccount, Mint, MintTo, TokenAccount,
    Transfer,
};
use anchor_spl::{self};

declare_id!("8mD7hwb1rqXnhCLq67RT4KHg1H93hiWekjHjvy7z6Q3U");

#[program]
pub mod multi_chain_amm {
    use super::*;

    pub fn init_amm(ctx: Context<InitAmm>, fee: u16) -> Result<()> {
        let pool = &mut ctx.accounts.pool_authority;
        let token_program = ctx.accounts.token_program.to_account_info();
        let eth_token_mint = &ctx.accounts.eth_token_mint;
        let eth_token_acc = &ctx.accounts.eth_token_acc;
        let rent = ctx.accounts.rent.to_account_info();
        let bump = *ctx.bumps.get("pool").unwrap();

        let create_acc_accounts = InitializeAccount {
            account: eth_token_acc.to_account_info(),
            mint: eth_token_mint.to_account_info(),
            authority: pool.to_account_info(),
            rent: rent.to_account_info(),
        };

        initialize_account(CpiContext::new_with_signer(
            token_program,
            create_acc_accounts,
            &[&[b"pool", &[bump]]],
        ))?;

        pool.bump = bump;
        pool.eth_value = 0;
        pool.sol_value = 0;
        pool.lp_value = 0;
        pool.eth_token_account = eth_token_acc.key();
        pool.fee = fee;

        Ok(())
    }

    pub fn create_liquidity_token(ctx: Context<CreateLiquidityToken>) -> Result<()> {
        Ok(())
    }

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
            &[&[b"liquidity-account", user.key().as_ref(), &[bump]]],
        ))?;

        liquidity_acc.eth = 0;
        liquidity_acc.sol = 0;
        liquidity_acc.bump = bump;
        Ok(())
    }

    pub fn provide_liquidity(ctx: Context<ProvideLiquidity>) -> Result<()> {
        // Todo
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAmm<'info> {
    #[account(init, payer = user, space = 8 + 1 + 8 + 8 + 8 + 32 + 2, seeds = [b"pool"], bump)]
    pub pool_authority: Account<'info, InitAmmAccount>,
    pub eth_token_acc: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: checked inside the instruct
    pub token_program: AccountInfo<'info>,
    /// CHECK: checked inside the instruct
    pub eth_token_mint: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CreateLiquidityToken<'info> {
    pub pool_account: Account<'info, InitAmmAccount>,
    #[account(
        init,
        seeds = [b"lp-mint".as_ref()],
        bump,
        payer = user,
        mint::decimals = 0,
        mint::authority = pool_account
    )]
    pub lp_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub eth_token_acc: Account<'info, TokenAccount>,
    /// CHECK: checked inside the instruct
    pub eth_token_mint: AccountInfo<'info>,
    /// CHECK: checked inside the instruct
    pub token_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
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
}

#[derive(Accounts)]
pub struct ProvideLiquidity {}

#[account]
pub struct LiquidityAccount {
    pub bump: u8,
    pub eth: u64,
    pub sol: u64,
}

#[account]
pub struct InitAmmAccount {
    bump: u8,
    eth_value: u64,
    sol_value: u64,
    lp_value: u64,
    eth_token_account: Pubkey,
    fee: u16,
}
