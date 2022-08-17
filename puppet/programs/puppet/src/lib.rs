use anchor_lang::prelude::*;
// declared static program id
declare_id!("Dz3AEzB6PvDCtGrETS1miAAWob8AqGUT7WJkQYm9d6ps");

// #[program] module where we define business logic
// we create endpoints here, which can be called by clients or "programs"
#[program]
pub mod puppet {
    use super::*;
    // Context<Initialize> : ctx is generic over Account type
    // ctx.accounts
    // ctx.program_id
    // #[derive(Accounts)] Initialize provides 2 #[accounts]: puppet and user and 1 program:SystemProgram
    pub fn initialize(ctx: Context<Initialize>, authority: Pubkey) -> Result<()> {
        ctx.accounts.puppet.authority = authority;
        msg!("L16 endpoint initialize: ctx.accounts: {:#?}", ctx.accounts); // this prints all accounts available in this context
        msg!("L17 endpoint initialize: ctx.program_id: {:#?}", ctx.program_id); // this prints program_id available in this context // 184,284
        msg!("L18 endpoint initialize: ctx.remaining_accounts: {:#?}", ctx.remaining_accounts); // this prints remaining_accounts available in this context

        msg!("L20 endpoint initialize: ctx.accounts.puppet: {:?}", ctx.accounts.puppet); // Structs in Rust contains an extra level of visibility. // you cannot print more in details of nesting level from here.
        msg!("L21 endpoint initialize: ctx.accounts.user: {:?}", ctx.accounts.user); 
        msg!("L22 endpoint initialize: ctx.accounts.system_program: {:?}", ctx.accounts.system_program); 
        Ok(())
    }

    pub fn set_data(ctx: Context<SetData>, data: u64) -> Result<()> {
        msg!("L27 endpoint set_data: ctx.accounts (before setting data): {:#?}", ctx.accounts); // this prints all accounts available in this context
        let puppet = &mut ctx.accounts.puppet;
        puppet.data = data;
        msg!("L30 endpoint set_data: ctx.accounts (after setting data): {:#?}", ctx.accounts); // this prints all accounts available in this context
        msg!("L31 endpoint set_data: ctx.program_id: {:#?}", ctx.program_id); // this prints program_id available in this context
        msg!("L32 endpoint set_data: ctx.remaining_accounts: {:#?}", ctx.remaining_accounts); // this prints remaining_accounts available in this context

        msg!("L34 endpoint set_data: ctx.accounts.puppet (after setting data): {:?}", ctx.accounts.puppet); // Structs in Rust contains an extra level of visibility. // you cannot print more in details of nesting level from here.
        msg!("L35 endpoint set_data: ctx.accounts.authority (after setting data): {:?}", ctx.accounts.authority); 
        Ok(())
        // Ok(data)
    }
}

#[derive(Accounts, Debug)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8 + 32)] //Added 32 to the space constraint of the puppet field for the Pubkey field in the Data struct.
    pub puppet: Account<'info, Data>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts, Debug)]
pub struct SetData<'info> {
    // has one : same feild name: authority in Data struct === authority in SetData struct
    #[account(mut, has_one = authority)] // since we cannot access private fields of Account Struct in #[program] 
    pub puppet: Account<'info, Data>,   // therefore we'll need to add constraints here at the top for checking the account
                                        // instead of checking them at the #[program] level by accessing ctx.accounts.puppet.info.owner
                                        // we can put constraints directly here.
    pub authority: Signer<'info>,
}

#[account]
#[derive(Debug)]
pub struct Data {
    pub data: u64,
    pub authority: Pubkey, // this authority is checked by above SetData struct, because of has_one macro
}
