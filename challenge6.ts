// this is the source code for a hacking challenge
// it contains vulnerabulities!
// do not use this in production!

use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh::try_from_slice_unchecked,
    entrypoint,
    entrypoint::ProgramResult,
     msg,
    program::{invoke_signed, invoke},
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar, self}, hash::Hash, program_error::ProgramError,
};

const TREASURY_ID : Pubkey = solana_program::pubkey!("TreEezyGib76ooHKtwsVA1iEM1Mr9QpskeCCkqP2t1Z");

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct HackerAccount {
    pub hacker: Pubkey,
    pub saved: bool,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Game of Dev - Challenge 6");
    msg!("Hack me!");

    let treasury = &accounts[4];
    let sysvar_slothahses_account = &accounts[5];

    if *sysvar_slothahses_account.key != sysvar::slot_hashes::id() {
        msg!("Invalid SlotHashes sysvar");
        return Err(ProgramError::InvalidArgument);
    }

    if *treasury.key != TREASURY_ID {
        msg!("Hey! I want those funds!");
        return Err(ProgramError::InvalidArgument);
    }

    let slot_data = sysvar_slothahses_account.try_borrow_data()?;

    // let num_slot_hashes = u64::from_le_bytes(data[0..8].try_into().unwrap());
    let hash_nr = 100;
    let mut offset = 8 // u64 storing number of hashes
        + (8 + 32) * hash_nr; // more recent entries
    let slot_number = u64::from_le_bytes(slot_data[offset..offset + 8].try_into().unwrap());
    offset+=8; // slot number
    let slot_hash = &slot_data[offset..offset + 32];

    msg!("Using hash from slot {}: {}", slot_number, Hash::new(slot_hash));

    let random_number = u64::from_le_bytes(slot_hash[12..20].try_into().unwrap());
    msg!("Calculated pseudo-random number: {}", random_number);

    if random_number % 100 == 42 {
        msg!("You got lucky!");
        save_hacker(program_id, accounts)
    } else {
        msg!("Bad luck!");
        punish_hacker(accounts)
    }
}

pub fn punish_hacker(accounts: &[AccountInfo]) -> ProgramResult {
    let payer = &accounts[0];
    let treasury = &accounts[4];
    let system_program = &accounts[3];
    invoke(
        &system_instruction::transfer(payer.key, treasury.key, 10000000),
        &[payer.clone(), treasury.clone(), system_program.clone()],
    )?;
    Ok(())
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
        &["CHALLENGE6".as_bytes(), hacker_mint.key.as_ref()],
        program_id,
    );

    if *account.key != pda {
        msg!("Wrong account provided! Try again!");
        msg!("Expected {}, Provided {}", pda, account.key);
        return Err(ProgramError::InvalidArgument);
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
        &[&["CHALLENGE6".as_bytes(), hacker_mint.key.as_ref(), &[bump]]],
    )?;

    let mut hacker_account =
        try_from_slice_unchecked::<HackerAccount>(&account.data.borrow()).unwrap();
    hacker_account.hacker = *hacker_mint.key;
    hacker_account.saved = true;
    hacker_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Your hacker {} is safe!", hacker_mint.key);

    Ok(())
}
