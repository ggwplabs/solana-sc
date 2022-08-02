use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct ChangeAdmin {}

#[derive(Accounts)]
pub struct ChangeParams {}

#[derive(Accounts)]
pub struct Freeze {}

#[derive(Accounts)]
pub struct Unfreeze {}
