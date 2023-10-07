use anchor_lang::prelude::*;
use anchor_spl::{
	token,
	token::{MintTo, Token},
};

declare_id!("2sH2Q2kcFvWpxjXqv78THttyfVKS25eNFvuD3vaj6BiN");

#[program]
pub mod carbon_engine {
	use super::*;

	pub fn mint_token(ctx: Context<MintToken>) -> Result<()> {
		let cpi_accounts = MintTo {
			mint: ctx.accounts.mint.to_account_info(),
			to: ctx.accounts.token_account.to_account_info(),
			authority: ctx.accounts.payer.to_account_info(),
		};

		let cpi_program = ctx.accounts.token_program.to_account_info();
		let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

		token::mint_to(cpi_ctx, 10)?;
		Ok(())
	}
}

#[derive(Accounts)]
pub struct MintToken<'info> {
	/// CHECK: This not dangerous because we don't read/write
	#[account(mut)]
	pub mint: UncheckedAccount<'info>,
	pub token_program: Program<'info, Token>,
	/// CHECK: This not dangerous because we don't read/write
	#[account(mut)]
	pub token_account: UncheckedAccount<'info>,
	/// CHECK: This not dangerous because we don't read/write
	#[account(mut)]
	pub payer: AccountInfo<'info>,
}