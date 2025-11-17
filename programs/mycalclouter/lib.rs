use anchor_lang::prelude::*;

declare_id!("96Df3fCBXacgGpqZZWXgxu2YNMPK6aJbCFQqBoj7RKk5");

#[program]
pub mod mycalclouter {
    use super::*;

    pub fn create(ctx: Context<Create>, init_value: String) -> Result<()> {
        let claclouter = &mut ctx.accounts.claclouter;
        claclouter.greeting = init_value;
        Ok(())
    }

    pub fn add(ctx: Context<Create>, a: i64, b: i64) -> Result<()> {
        let claclouter = &mut ctx.accounts.claclouter;
        claclouter.result = a + b;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 264)]
    pub claclouter: Account<'info, Claclouter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Addtion<'info> {
    #[account(mut)]
    pub claclouter: Account<'info, Claclouter>,
}

#[account]
pub struct Claclouter {
    pub greeting: String,
    pub result: i64,
    pub remainder: i64,
}
