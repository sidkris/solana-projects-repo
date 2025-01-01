use anchor_lang::prelude::*;

declare_id!("");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favourites {
    use super::*;
    pub fn set_favourites(
        ctx: Context<SetFavourites>,
        number: u64,
        colour: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        // Input validation
        if colour.len() > 50 {
            return Err(FavouritesError::ColorTooLong.into());
        }
        if hobbies.len() > 5 {
            return Err(FavouritesError::TooManyHobbies.into());
        }
        if hobbies.iter().any(|h| h.len() > 50) {
            return Err(FavouritesError::HobbyTooLong.into());
        }

        msg!("Greetings from {}", ctx.program_id);
        let user_public_key = ctx.accounts.user.key();
        msg!("User {user_public_key}'s favourite colour is {colour}.");

        ctx.accounts.favourites.set_inner(Favourites {
            number,
            colour,
            hobbies,
        });

        Ok(())
    }
}

#[error_code]
pub enum FavouritesError {
    #[msg("Colour is too long (max 50 characters)")]
    ColorTooLong,
    #[msg("Too many hobbies (max 5)")]
    TooManyHobbies,
    #[msg("Hobby is too long (max 50 characters)")]
    HobbyTooLong,
}

#[account]
#[derive(InitSpace)]
pub struct Favourites {
    pub number: u64,
    #[max_len(50)]
    pub colour: String,
    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct SetFavourites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed, 
        payer = user, 
        space = ANCHOR_DISCRIMINATOR_SIZE + Favourites::INIT_SPACE,
        seeds = [b"favourites", user.key().as_ref()], 
        bump
    )]
    pub favourites: Account<'info, Favourites>,
    pub system_program: Program<'info, System>,
}
