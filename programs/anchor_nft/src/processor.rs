use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use anchor_spl::token::{self, SetAuthority, Token, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;
use crate::{PREFIX};
#[repr(C)]
#[derive(Copy, Clone, BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub enum OrderType {
    FixedPay,
    AuctionType,
}

#[repr(C)]
#[derive(Copy, Clone, BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub enum OrderStatus {
    Active,
    Bidded,
    UnderDownsideProtectionPhase,
    Completed,
    Cancelled,
}

#[repr(C)]
#[derive(Copy, Clone, BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub enum BidStatus {
    NotAccepted,
    Pending,
    Refunded,
    Excuted,
}

#[repr(C)]
#[derive(Copy, Clone, BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct Bid(pub u128, pub BidStatus);

#[repr(C)]
#[derive(Copy, Clone, BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct BidState(pub Pubkey, pub Bid);

#[account]
#[allow(non_snake_case)]
pub struct ProtectedMarketPalce
{
    pub depository: Pubkey,
    pub company: Pubkey,
    pub companyFeeRate: u64,
    pub orderIdCount: u128,
    pub subOrderIdCount: u128,
    pub orders: Vec<Order>,
    pub subOrders: Vec<SubOrder>,
    pub buyerBidStatus: Vec<BidState>
}

impl ProtectedMarketPalce {
    pub const LEN: usize = 1 + 32 + 32 + 64 + 128;
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq, Debug)]
#[allow(non_snake_case)]
pub struct Order {
    pub statusOrder: OrderStatus,
    pub typeOrder: OrderType,
    pub tokenAddress: Pubkey,
    pub nftTokenId: u128,
    pub sellerAddress: Pubkey,
    pub buyerAddress: Pubkey,
    pub tokenPrice: u128,
    //Protection
    pub protectionAmmount: u128,
    pub depositId: u128,
    pub protectionRate: u64,
    pub protectionTime: u128,
    pub soldTime: u128,
    //SubOrder
    pub offerClosingTime: u128,
    pub subOrderList: Vec<SubOrder>
}

#[repr(C)]
#[derive(Copy, Clone, BorshSerialize, BorshDeserialize, PartialEq, Debug)]
#[allow(non_snake_case)]
pub struct SubOrder{
    pub orderId: u128,
    pub buyerAddress: Pubkey,
    pub tokenPrice: u128,
    pub protectionRate: u64,
    pub protectionTime: u128,
    pub validUntil: u128,
}


pub fn initializeMarketPlace(
    ctx: Context<InitializeMarketplace>,
    companyFeeRate: u64,
) -> ProgramResult {
    ctx.accounts.market_account.company = *ctx.accounts.company.key;
    ctx.accounts.market_account.depository = 
    *ctx.accounts.depository.key;
    ctx.accounts.market_account.companyFeeRate = companyFeeRate;
    Ok(())
}

pub fn createSubOrder(
    ctx: Context<CreateSubOrder>,
    orderId: u128,
    tokenPrice: u128,
    protectionRate: u64,
    protectionTime: u128,
    validUntil: u128,
) -> ProgramResult{
    let orderid = orderId as usize;
    let mut order:Order = ctx.accounts.market_account.orders[orderid].clone();
    ctx.accounts.market_account.subOrderIdCount += 1;
    let mut sub = SubOrder{
        orderId: orderId,
        buyerAddress: *ctx.accounts.buyerAddress.key,
        tokenPrice: tokenPrice,
        protectionRate: protectionRate,
        protectionTime: protectionTime,
        validUntil: validUntil,
    };

    ctx.accounts.market_account.subOrders.push(sub);
    order.subOrderList.push(sub);

    Ok(())
}

pub fn buySubOrder(
    ctx: Context<MarketProgram>,
    orderId: u128,
    subOrderId: u128,
) -> ProgramResult{
    let orderid = orderId as usize;
    let mut order:Order = ctx.accounts.market_account.orders[orderid].clone();
    let subid = subOrderId as usize;
    let mut suborder:SubOrder = ctx.accounts.market_account.subOrders[subid];

    order.protectionRate = suborder.protectionRate;
    order.protectionTime = suborder.protectionTime;
    // _proceedPayments();
    Ok(())
}

pub fn buyFixedPayOrder(
    ctx: Context<MarketProgram>,
    orderId: u128
) -> ProgramResult{
    let orderid = orderId as usize;
    let mut order:Order = ctx.accounts.market_account.orders[orderid].clone();
    // _proceedPayments();
    // order.buyerAddress = !msg.sender;
    Ok(())
}

pub fn cancelOrder(
    ctx: Context<MarketProgram>,
    orderId: u128,
) -> ProgramResult{
    let orderid = orderId as usize;
    let mut order:Order = ctx.accounts.market_account.orders[orderid].clone();
    
    //Change NFT authority
    let (pda, _bump_seed) = Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
    // token::set_authority(ctx.accounts.into(), AuthorityType::AccountOwner, Some(pda))?;

    order.statusOrder = OrderStatus::Cancelled;
    Ok(())
}

pub fn claimDownsideProtectionAmount(
    ctx: Context<MarketProgram>,
    orderId: u128,
) -> ProgramResult{
    let orderid = orderId as usize;
    let mut order:Order = ctx.accounts.market_account.orders[orderid].clone();

    Ok(())
}

pub fn sellerCheckClaimDownsideProtectionAmount(
    ctx: Context<MarketProgram>,
    orderId: u128,
) -> ProgramResult{
    let orderid = orderId as usize;
    let mut order:Order = ctx.accounts.market_account.orders[orderid].clone();

    Ok(())
}

pub fn buyerCheckClaimDownsideProtectionAmount(
    ctx: Context<MarketProgram>,
    orderId: u128,
) -> ProgramResult{
    let orderid = orderId as usize;
    let mut order:Order = ctx.accounts.market_account.orders[orderid].clone();

    Ok(())
}

pub fn createBid(
    ctx: Context<MarketProgram>,
    orderId: u128,
) -> ProgramResult{
    let orderid = orderId as usize;
    let mut order:Order = ctx.accounts.market_account.orders[orderid].clone();

    Ok(())
}

pub fn executeBid(
    ctx: Context<MarketProgram>,
    orderId: u128,
) -> ProgramResult{
    let orderid = orderId as usize;
    let mut order:Order = ctx.accounts.market_account.orders[orderid].clone();

    Ok(())
}

pub fn setNewCompany(
    ctx: Context<MarketProgram>,
    orderId: u128,
) -> ProgramResult{
    let orderid = orderId as usize;
    let mut order:Order = ctx.accounts.market_account.orders[orderid].clone();

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeMarketplace<'info> {
    #[account(signer)]
    pub company: AccountInfo<'info>,
    #[account(mut)]
    pub depository: AccountInfo<'info>,
    #[account(init, payer = company, space = 8 + ProtectedMarketPalce::LEN)]
    pub market_account: Account<'info, ProtectedMarketPalce>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CreateSubOrder<'info>{
    #[account(mut)]
    pub market_account: Account<'info, ProtectedMarketPalce>,
    pub buyerAddress: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct MarketProgram<'info>
{
    #[account(mut)]
    pub market_account: Account<'info, ProtectedMarketPalce>,
}
