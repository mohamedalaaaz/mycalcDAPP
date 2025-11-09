use anchor_lang::prelude::*;

declare_id!("96Df3fCBXacgGpqZZWXgxu2YNMPK6aJbCFQqBoj7RKk5");

#[program]
pub mod mycalclouter {
    use super::*;

    pub fn create(ctx:context<Create>,init_value:String)->programresult{
        let claclouter=&mut ctx.accounts.claclouter;
        claclouter.greeting=init_value;
        Ok(())

    }

    
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer=user, space=264)]
    pub claclouter: Account<'info, Claclouter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
