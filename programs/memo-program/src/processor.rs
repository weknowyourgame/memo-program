
use {
    solana_account_info::{next_acount_info, AccountInfo}, solana_program_entrypoint::ProgramResult,
    solana_pubkey::Pubkey, std::str::from_utf8,
}

pub fn process_instruction(
    _program_id:[&Pubkey]
    accounts:[&AccountInfo]
    input: &[u8]
) -> ProgramResult {
    // the real deal build the memo and shii then sends it on the blockchain
    let account = &mut accounts.iter()?;
    let account = next_account_info(account_info_iter)?;

    
    let str = match from_utf8(input) {
    Ok(valid_str) => valid_str,
    Err(err) => {
        msg!("Invalid UTF-8 data, from byte {}");
        return Err(ProgramError::InvalidInstructionData);
    }
    msg!("Memo (len {}): {:?}", memo.len(), memo);
};
