use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, initialize_account, initialize_mint, Burn, InitializeAccount, Mint, MintTo, TokenAccount,
    Transfer,
};

use anchor_lang::system_program;
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

    pub fn provide_liquidity(
        ctx: Context<ProvideLiquidity>,
        eth_amount: u64,
        sol_amount: u64,
    ) -> Result<()> {
        let liquidity_acc = &mut ctx.accounts.liquidity_acc;
        let token_program_acc = &ctx.accounts.token_program;
        let system_program_acc = &ctx.accounts.system_program;
        let user = &mut ctx.accounts.user.to_account_info();
        let eth_token_mint = &ctx.accounts.eth_token_mint;
        let eth_token_acc = &ctx.accounts.eth_token_acc;
        let pool_account = &mut ctx.accounts.pool_account;
        let lp_mint = &ctx.accounts.lp_mint;
        let lp_token_acc = &ctx.accounts.lp_token_acc;
        let pool_eth_token_acc = &ctx.accounts.pool_eth_token_acc;

        let transfer_accounts = token::Transfer {
            from: eth_token_acc.to_account_info(),
            to: pool_eth_token_acc.to_account_info(),
            authority: user.to_account_info(),
        };

        token::transfer(
            CpiContext::new_with_signer(
                token_program_acc.to_account_info(),
                transfer_accounts,
                &[&[
                    b"liquidity-account",
                    user.key().as_ref(),
                    &[liquidity_acc.bump],
                ]],
            ),
            eth_amount,
        )?;

        let transfer_accounts = system_program::Transfer {
            from: user.to_account_info(),
            to: liquidity_acc.to_account_info(),
        };

        system_program::transfer(
            CpiContext::new_with_signer(
                system_program_acc.to_account_info(),
                transfer_accounts,
                &[&[
                    b"liquidity-account",
                    user.key().as_ref(),
                    &[liquidity_acc.bump],
                ]],
            ),
            sol_amount,
        )?;

        if (pool_account.eth_value == 0) && (pool_account.sol_value == 0) {
            let amount_to_mint = sol_amount;
            let mint_to_accounts = MintTo {
                mint: lp_mint.to_account_info(),
                to: lp_token_acc.to_account_info(),
                authority: pool_account.to_account_info(),
            };

            token::mint_to(
                CpiContext::new_with_signer(
                    token_program_acc.to_account_info(),
                    mint_to_accounts,
                    &[&[b"pool", &[pool_account.bump]]],
                ),
                amount_to_mint,
            )?;

            pool_account.lp_value += amount_to_mint;
        } else {
            let amount_to_mint = (sol_amount * pool_account.lp_value) / pool_account.sol_value;
            let mint_to_accounts = MintTo {
                mint: lp_mint.to_account_info(),
                to: lp_token_acc.to_account_info(),
                authority: pool_account.to_account_info(),
            };

            token::mint_to(
                CpiContext::new_with_signer(
                    token_program_acc.to_account_info(),
                    mint_to_accounts,
                    &[&[b"pool", &[pool_account.bump]]],
                ),
                amount_to_mint,
            )?;

            pool_account.lp_value += amount_to_mint;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAmm<'info> {
    #[account(init, payer = user, space = 8 + 1 + 8 + 8 + 8 + 32 + 2 + 8, seeds = [b"pool"], bump)]
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
pub struct ProvideLiquidity<'info> {
    pub liquidity_acc: Account<'info, LiquidityAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: checked inside the instruct
    pub token_program: AccountInfo<'info>,
    /// CHECK: checked inside the instruct
    pub eth_token_mint: AccountInfo<'info>,
    #[account(mut)]
    pub eth_token_acc: Account<'info, TokenAccount>,
    pub pool_account: Account<'info, InitAmmAccount>,
    pub pool_eth_token_acc: Account<'info, TokenAccount>,
    pub lp_token_acc: Account<'info, TokenAccount>,
    pub lp_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

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
    mult_const: u64,
}
