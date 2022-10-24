use anchor_lang::prelude::*;
use solana_program::entrypoint::ProgramResult;
use std::mem::size_of; 
use anchor_lang::solana_program::log::{
    sol_log_compute_units
};
    
// This is your program's public key 
declare_id!("BAM2RzYFjcXCbU4Vm6xHnhaeATKGiS1n5XCBxctKMapV");
const USER_NAME_LENGTH: usize=100;
const USER_URL_LENGTH: usize=225 ;
const VIDEO_URL_LENGTH : usize=225;
const TEXT_LENGTH : usize=1024;
const NUMBER_OF_ALLOWED_LIKES_SPACE: usize=5


#[program]
mod my_tiktok {
    use super::*;
    
    pub fn create_user(ctx:Context<CreateUser>,name:String,profile_url:String)->ProgramResult{
    let user = &mut ctx.accounts.user;
    user.user_wallet_address = ctx.accounts.authority.key();
    user.user_name = name;
    user.user_profile_image_url = profile_url;
    msg!("User added successfuly!"); 
    sol_log_compute_units();
    ok(())
    }
// Create video function
    pub fun create_video(
         ctx:Context<CreateVideo>,
         description:String,
         video_url:String,
         creator_name:String,
         creator_url:String
        )->ProgramResult{
        msg!(&description)

    let video:&mut Account<VideoAccount> =&mut ctx.accounts.video;
    video.authority=ctx.accounts.authority.key();
    video.description=description;
    video.video_url=video_url;
    video.creator_name=creator_name;
    video.creator_url=creator_url;
    video.comment_count=0;
    video.creator_time=ctx.accounts.clock.unix_timestamp;
    vido.likes=0;
    msg!("Video has been addes!");
    sol_log_compute_units();
    ok(())
        }

// Create comment function
    pub fn create_commemt(
        ctx:<CreateComment>,
        text:String,
        commentor:String,
        commentor_url:String,
    )->ProgramResult{
        //note commenting means directly affecting the video
        let video: &mut Account<VideoAccount> = &mut ctx.accounts.video;
        let comment :&mut Account<CommentAccount> = &mut ctx.accounts.comment;
        comment.authority=ctx.accounts.authority;
        comment.text=text;
        comment.commentor_name=commentor_name;
        comment.commentor_url=commentor_url;
        comment.index=video.comment.index;
        comment.video_time=ctx.accounts.clock.unix_timestamp;
        video.comment_count +=1;
        ok(())
    }

// Video Like function
pub fn like_video(ctx:Context<LikeVideo>)->Programresult{
    let video : &mut Account<videoAccount> =&mut ctx.accounts.video
    // iterating accounts is safer than indexing(for loop)
    let mut iter : Iter<PubKey> = video.people_who_liked.iter();
        let user_liking_video : PubKey=ctx.accounts.authority.key();
        video.likes +=1;
        video.people_who_liked.push(user_liking_video);
       ok(())
} 
}

// UserAccount struct--------------------------------------->
#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(
        init,
        seeds=[b"user".as_ref(),authority.key().as_ref()],
        bump,
        payer = authority,
        space = size_of::<UserAccount>() + USER_NAME_LENGTH + VIDEO_URL_LENGTH + 8 
    )]
    pub user: Account<'info UserAccount>,

    //this is signer who paid the transaction fee
    #[account(mut)]
    pub authority:Signer<'info>,
    pub system_program:Unchecked<'info>,
    pub clock:Sysvar<'info,clock>,  
}

#[account]
pub struct UserAccount {
    pub user_name:String,
    pub user_wallet_address:Pubkey,
    pub user_profile_image_url:String,  
}
//---------------------------------------------------------->


// CreateVideo struct--------------------------------------->
#[derive(Accounts)]
pub struct createVideo<'info>{
    //first authenticate the video account
    #[account(
        init,
        seeds=[b"video".as_ref(),randomkey.key().as_ref()],
        bump,
        payer=authority,
        space=size_of::<VideoAccount>() + TEXT_LENGTH +
        USER_NAME_LENGTH +
        USER_URL_LENGTH + 
        VIDEO_URL_LENGTH + 8 + 32 *NUMBER_OF_ALLOWED_LIKES_SPACE 
    )]
    pub video:Account<'info,VideoAccount>,

    #[account(mut)]
    pub randomkey:AccountInfo<'info>,
    
    //this is signer who paid the transaction fee
    #[account(mut)]
    pub authority:Signer<'info>,
    pub system_program:Unchecked<'info>,
    pub clock:Sysvar<'info,clock>,   
}

#[account]
pub struct videoAccount{
    pub authority:Pubkey,
    pub description:String,
    pub video_url:String,
    pub creator_name:String,
    pub creator_url:String,
    pub comment_count:u64,
    pub index:u64,
    pub creator_time:i64,
    pub people_who_liked:Vec<PubKey>,
    pub likes:u8,
    pub remove:i64 
}
//---------------------------------------------------->

// CreateComment struct--------------------------------->
#[derive(Accounts)]
pub struct CreateComment<'info>{
    #[account(mut)]
    pub video:Account<'info,VideoAccount>,
    //authenticate the comment account
    #[account(
        init,
        seeds=[b"comment".as_ref(),video.key().as_ref,vido.comment_count.to_be_bytes().as_ref()],
        bump,
        payer=authority,
        space=size_of::<CommentAccount>() + TEXT_LENGTH + USER_NAME_LENGTH + 
        USER_URL_LENGTH + VIDEO_URL_LENGTH
    )]
    pub comment:Account<'info,CommentAccount>,
    //this is signer who paid the transaction fee
    #[account(mut)]
    pub authority:Signer<'info>,
    pub system_program:Program<'info,System>,
    pub clock:Sysvar<'info,clock>,   
}
#[account]
pub struct CommentAccount{
    pub authority: Pubkey;
    pub text : String,
    pub commentor_name:String,
    pub commentor_url:String,
    pub index:u64,
    pub video_time:i64,
}

// LikeVideo  struct-------------------------------------------->
 #[derive(Accounts)] 
pub struct LikeVideo<'info>{
    #[account(mut)]
    pub video:Account<'info,VideoAccount>,
    //this is signer who paid the transaction fee
    #[account(mut)]
    pub authority:Signer<'info>,
    pub system_program:Unchecked<'info>,
    pub clock:Sysvar<'info,clock>
}
//-------------------------------------------------------->



