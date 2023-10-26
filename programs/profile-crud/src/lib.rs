use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("HFMCvkxw8mLvYkS8SvvrEoNghP3AyH6BGFkCi5dzB9Fk");

#[program]
mod profile_crud {
    use super::*;
    pub fn saveprofile(
        ctx: Context<Qprofile>,
        nickname: String,
        description: String,
        uri: String,
        level: u64,
    ) -> Result<()> {
        let qprofile = &mut ctx.accounts.qprofile;
        msg!("Previous Nickname: { }", qprofile.nickname);
        msg!("Previous Bio: { }", qprofile.description);
        qprofile.level = level;
        qprofile.nickname = nickname;
        qprofile.description = description;
        qprofile.uri = uri;
        qprofile.user = ctx.accounts.user.key();

        msg!("New Nickname: { }", qprofile.nickname);
        msg!("New Bio: { }", qprofile.description);

        Ok(())
    }
    pub fn updateprofile(
        ctx: Context<QprofileUpdate>,
        nickname: String,
        description: String,
        uri: String,
        level: u64,
    ) -> Result<()> {
        let qprofile = &mut ctx.accounts.qprofile;
        msg!("Previous Nickname: { }", qprofile.nickname);
        msg!("Previous Bio: { }", qprofile.description);
        qprofile.level = level;
        qprofile.nickname = nickname;
        qprofile.description = description;
        qprofile.uri = uri;
        qprofile.user = ctx.accounts.user.key();

        msg!("New Nickname: { }", qprofile.nickname);
        msg!("New Bio: { }", qprofile.description);

        Ok(())
    }
    pub fn deleteprofile(_ctx: Context<QprofileDelete>) -> Result<()> {
        msg!("Profile deleted");

        Ok(())
    }
}

#[account]
pub struct QprofileData {
    pub level: u64,
    pub nickname: String,
    pub description: String,
    pub uri: String,
    pub user: Pubkey,
}

#[derive(Accounts)]
#[instruction(nickname:String, description: String, uri: String)]
pub struct Qprofile<'info> {
    // CHECK: Manual validation
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 8 + 4 + nickname.len() + 4 + description.len() + 4 + uri.len() + 32,
        seeds = ["qprofile".as_bytes().as_ref(), user.key().as_ref()], 
        bump)]
    pub qprofile: Account<'info, QprofileData>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
#[instruction(nickname:String, description: String, uri: String)]
pub struct QprofileUpdate<'info> {
    // CHECK: Manual validation
    // CHECK: Manual validation
    #[account(  
        mut,
        realloc = 8 + 8 + 4 + nickname.len() + 4 + description.len() + 4 + uri.len() + 32,
        seeds = ["qprofile".as_bytes().as_ref(), user.key().as_ref()], 
        bump,
        realloc::payer = user,
        realloc::zero = true,)]
    pub qprofile: Account<'info, QprofileData>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct QprofileDelete<'info> {
    #[account(  
        mut,
        seeds = ["qprofile".as_bytes().as_ref(), user.key().as_ref()], 
        bump,
        close=user)]
    pub qprofile: Account<'info, QprofileData>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
