use anchor_lang::prelude::*;
use anchor_spl::token::*;

declare_id!("AnChyn46WBUX6VE2EgkQd2XMqBdKAKmGw3znPNzYHjf7");

#[program]
pub mod ch7 {
    use super::*;

    pub fn save_hacker(ctx: Context<SaveHacker>, your_number: u64, my_number: i16) -> Result<()> {
        
        msg!("Game of Dev - Challenge 7");

        msg!("Your favourite number is {}", your_number);

        if my_number != -7 {
            msg!("You didn't get my number right! ({})", my_number);
            return err!(ChallengeError::WrongInputNumber);
        }

        ctx.accounts.save_account.hacker_mint = ctx.accounts.hacker_mint.key();
        ctx.accounts.save_account.is_save = true;

        msg!("Your hacker {} is safe ;)", ctx.accounts.hacker_mint.key());

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SaveHacker<'info> {
    #[account(init, payer = user, space = 8 + 32 + 1, 
        seeds = [b"CHALLENGE7", hacker_mint.key().as_ref()], bump)]
    pub save_account: Account<'info, HackerSaveAccount>,
    pub hacker_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct HackerSaveAccount {
    hacker_mint: Pubkey,
    is_save: bool
}

#[error_code]
pub enum ChallengeError {
    #[msg("Wrong input data provided.")]
    WrongInputNumber
}
