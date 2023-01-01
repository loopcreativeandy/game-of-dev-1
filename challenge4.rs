use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh::try_from_slice_unchecked,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct HackerAccount {
    pub hacker: Pubkey,
    pub saved: bool,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Game of Dev - Challenge 4");
    msg!("Hack me!");

    let magic: u32 = 0xDEAD;
    let input = u32::from_le_bytes(instruction_data.try_into().unwrap());
    let deaths = input + magic;

    if deaths != 0 {
        msg!("Die {} deaths!", deaths);
        //Err(solana_program::program_error::ProgramError::Custom(1))
        Ok(())
    } else {
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
        &["CHALLENGE4".as_bytes(), hacker_mint.key.as_ref()],
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
        &[&["CHALLENGE4".as_bytes(), hacker_mint.key.as_ref(), &[bump]]],
    )?;

    let mut hacker_account =
        try_from_slice_unchecked::<HackerAccount>(&account.data.borrow()).unwrap();
    hacker_account.hacker = *hacker_mint.key;
    hacker_account.saved = true;
    hacker_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Your hacker {} is safe for challenge 4!", hacker_mint.key);

    Ok(())
}
