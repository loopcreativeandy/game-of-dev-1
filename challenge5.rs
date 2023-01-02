use borsh::{BorshDeserialize, BorshSerialize};
use std::str::FromStr;
use thiserror::Error;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh::try_from_slice_unchecked,
    entrypoint,
    entrypoint::ProgramResult,
    hash, msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct HackerAccount {
    pub hacker: Pubkey,
    pub saved: bool,
}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Wrong password provided")]
    PasswordMissmatch = 0xdead,
}
impl From<MyError> for ProgramError {
    fn from(e: MyError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Game of Dev - Challenge 5");
    msg!("Hack me!");

    let password = std::str::from_utf8(instruction_data).unwrap();

    msg!("Your guess is: {}", password);

    let password_hash = hash::hash(instruction_data);
    msg!("hashed password: {}", password_hash);

    let correct_hash =
        hash::Hash::from_str("716QNtdHN3hWoeEsgwEbG6rUUWk9vVC1U6xxz1vowWnH").unwrap();

    if password_hash != correct_hash {
        msg!("Wrong guess!");
        return Err(MyError::PasswordMissmatch.into());
    } else {
        msg!("Nailed it!");
        save_hacker(program_id, accounts)
    }
}

pub fn save_hacker(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let account = next_account_info(accounts_iter)?;
    let hacker_mint = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let account_len: usize = 33;
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);
    let (pda, bump) = Pubkey::find_program_address(
        &["CHALLENGE5".as_bytes(), hacker_mint.key.as_ref()],
        program_id,
    );

    if *account.key != pda {
        msg!("Wrong account provided! Try again!");
        msg!("Expected {}, Provided {}", pda, account.key);
    }

    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[payer.clone(), account.clone(), system_program.clone()],
        &[&["CHALLENGE5".as_bytes(), hacker_mint.key.as_ref(), &[bump]]],
    )?;

    let mut hacker_account =
        try_from_slice_unchecked::<HackerAccount>(&account.data.borrow()).unwrap();
    hacker_account.hacker = *hacker_mint.key;
    hacker_account.saved = true;
    hacker_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Your hacker {} is safe!", hacker_mint.key);

    Ok(())
}
