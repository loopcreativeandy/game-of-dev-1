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
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hack me!");

    let accounts_iter = &mut accounts.iter();

    // expecting the following accoints in this order:
    // - fee payer: the signing wallet to pay for account rent
    // - hacker account: if this account is created then your hacker is safe!
    // - your hacker nft (token mint account) : used just to derive the hacker account
    // - the system program: to create a new account
    let payer = next_account_info(accounts_iter)?;
    let account = next_account_info(accounts_iter)?;
    let hacker_mint = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let account_len: usize = 33;
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    // we use the following seeds for the hacker account: "CHALLENGE3"+hacker mint key
    let (pda, bump) = Pubkey::find_program_address(
        &["CHALLENGE3".as_bytes(), hacker_mint.key.as_ref()],
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
        &[&["CHALLENGE3".as_bytes(), hacker_mint.key.as_ref(), &[bump]]],
    )?;

    let mut hacker_account =
        try_from_slice_unchecked::<HackerAccount>(&account.data.borrow()).unwrap();
    hacker_account.hacker = *hacker_mint.key;
    hacker_account.saved = true;
    hacker_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Your hacker {} is safe!", hacker_mint.key);

    Ok(())
}
