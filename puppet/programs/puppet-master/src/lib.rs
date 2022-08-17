use anchor_lang::prelude::*;
use puppet::cpi::accounts::SetData; // features = ["cpi"] added in cargo.toml : allows us to use use puppet's types, its instruction builders and cpi functions.
                                    // because of enabling the cpi feature, the puppet-master program gets access to the puppet::cpi module

// the puppet-master uses the SetData instruction builder struct
// provided by the puppet::cpi::accounts module
// to submit the accounts the SetData instruction of the puppet program expects
use puppet::program::Puppet;
use puppet::{self, Data};

declare_id!("CChSa4393WhYyRioBchtAaJvEBnuPA9PyeepEiqrbKpM");

// #[program] module where we define business logic
// we create endpoints here, which can be called by clients or "programs"
#[program]
mod puppet_master {
    use super::*;
    // Context<PullStrings> : ctx is generic over Account type
    // ctx.accounts
    // ctx.program_id
    // #[derive(Accounts)] pull_strings provides 2 accounts:puppet, authority, and 1 program:puppet_program
    pub fn pull_strings(ctx: Context<PullStrings>, data: u64) -> Result<u64> {
        msg!("L12 endpoint pull_strings: data: {:#?}", data); // this prints all accounts available in this context
                                                              // msg!("L12 endpoint pull_strings: ctx.accounts: {:#?}", ctx.accounts); // this prints all accounts available in this context
        msg!(
            "L13 endpoint pull_strings: ctx.program_id: {:#?}",
            ctx.program_id
        ); // this prints program_id available in this context
        msg!(
            "L14 endpoint pull_strings: ctx.remaining_accounts: {:#?}",
            ctx.remaining_accounts
        ); // this prints remaining_accounts available in this context

        msg!(
            "L16 endpoint pull_strings: ctx.accounts.puppet: {:?}",
            ctx.accounts.puppet
        ); // Structs in Rust contains an extra level of visibility. // you cannot print more in details of nesting level from here.
           // msg!("L17 endpoint pull_strings: ctx.accounts.user: {:?}", ctx.program.puppet_program);
        msg!(
            "L18 endpoint pull_strings: ctx.accounts.system_program: {:?}",
            ctx.accounts.authority
        );

        // invoking set_data endpoint for first smart contract
        // invoking inside the endpoint of second contract
        // a program calling other program: cross program invocation (CPI)
        // using contract::cpi method to invoke endpoint of first smart contract
        // An Anchor generated module, providing a set of structs mirroring the structs deriving Accounts, where each field is an AccountInfo. This is useful for CPI

        // set_data_ctx() is implemented for current endpoint context to return CpiContext of set_data
        // set_data_ctx() returns "CpiContext" for the endpoint of first smart contract where set_data lies.
        // set_data_ctx() creates "CpiContext" using CpiContext::new(cpi_program, cpi_accounts)
        // puppet::cpi::set_data(ctx.accounts.set_data_ctx(), data)

        let result = puppet::cpi::set_data(ctx.accounts.set_data_ctx(), data)?;
        msg!(
            "🚀 ~ file: lib.rs ~ line 57 ~ pubfnpull_strings ~ result {:?}",
            result
        );
        ctx.accounts.puppet.reload()?; //reload method which will re-deserialize the account
        msg!(
            "🚀 ~ file: lib.rs ~ line 59 ~ pub fn pull_strings ~ ctx.accounts.puppet.data: {:?}",
            ctx.accounts.puppet.data
        ); // if above reload() is commented then we won't see the updated value

        // The below statement calls sol_get_return and deserializes the result.
        // `return_data` contains the return from `set_data`,
        // which in this example is just `data`.
        // let return_data = result.get(); // runs when function returns -> Result<()>
        // msg!("🚀 ~ file: lib.rs ~ line 70 ~ pubfnpull_strings ~ return_data {:?}", return_data);

        // Ok(())  // -> Result<()>
        Ok(ctx.accounts.puppet.data) // -> Result<u64>
    }
}

#[derive(Accounts)]
pub struct PullStrings<'info> {
    #[account(mut)]
    pub puppet: Account<'info, Data>,
    pub puppet_program: Program<'info, Puppet>,
    // Even though the puppet program already checks that authority is a signer
    // using the Signer type here is still required because the anchor ts client
    // can not infer signers from programs called via CPIs
    pub authority: Signer<'info>,
}

// implementation of the endpoint of the other smart contract
// for this contract's endpoint: pull_strings

// Setting up a CPI can distract from the business logic of the program,
// so it's recommended to move the CPI setup into the impl block
impl<'info> PullStrings<'info> {
    // CpiContext
    pub fn set_data_ctx(&self) -> CpiContext<'_, '_, '_, 'info, SetData<'info>> {
        let cpi_program = self.puppet_program.to_account_info(); // owner of program: BPF Loader
        msg!(
            "🚀 ~ file: lib.rs ~ line 53 ~ pub fn set_data_ctx ~ cpi_program-AccountInfo {:?}",
            cpi_program
        );
        // #[derive(Debug)] //may only be applied to struct enum or union
        let cpi_accounts = SetData {
            puppet: self.puppet.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        // msg!("🚀 ~ file: lib.rs ~ line 55 ~ pub fn set_data_ctx ~ cpi_accounts-SetData {:?}", cpi_accounts);
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
