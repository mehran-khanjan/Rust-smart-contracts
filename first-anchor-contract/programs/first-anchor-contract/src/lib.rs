use anchor_lang::prelude::*;

declare_id!("CKUxQPU5Jkxu1NMJywQA3ayv79dGoMqhKro27pGMqeFy");

#[program]
pub mod first_anchor_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = 0;
        my_account.authority = *ctx.accounts.authority.key;
        Ok(())
    }

    pub fn increase(ctx: Context<Update>) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data += 1;
        Ok(())
    }

    pub fn decrease(ctx: Context<Update>) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        if my_account.data == 0 {
            return Err(error!(ErrorCode::BelowZero));
        }
        my_account.data -= 1;
         Ok(())
    }

    pub fn set(ctx: Context<Update>, value: u64) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = value;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct Initialize<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, payer = authority, space = 8 + 32 + 8)]
    pub my_account: Account<'info, MyAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut, has_one = authority)]
    pub my_account: Account<'info, MyAccount>,
    pub authority: Signer<'info>
}

#[account]
#[derive(Default)]
pub struct MyAccount {
    authority: Pubkey,
     data: u64
}

#[error_code]
pub enum ErrorCode {
    #[msg("Cannot decrease value 0f 0.")]
     BelowZero,
}
