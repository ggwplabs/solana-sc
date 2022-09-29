use anchor_lang::prelude::*;
use core::fmt;

const DESCRIMINATOR_LEN: usize = 8;

pub const USER_INFO_SEED: &str = "user_info";
pub const GPASS_BURN_AUTH_SEED: &str = "gpass_burn_auth";
pub const GAME_INFO_SEED: &str = "game_info";

#[account]
#[derive(Default, Debug)]
pub struct FightingSettings {
    pub admin: Pubkey,
    pub update_auth: Pubkey,

    pub afk_timeout: i64,
    pub gpass_burn_auth_bump: u8,
}

impl FightingSettings {
    pub const LEN: usize = DESCRIMINATOR_LEN +
        32 + // admin pk
        32 + // update auth pk
        8 + // AFK timeout
        1; // gpass burn auth bump
}

#[account]
#[derive(Default, Debug)]
pub struct UserFightingInfo {
    pub in_game: bool,
    pub in_game_time: i64,
}

impl UserFightingInfo {
    pub const LEN: usize = DESCRIMINATOR_LEN +
        1 + // in game status
        8 // in game time
        ;
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug, PartialEq)]
pub enum GameResult {
    Draw,
    Win,
    Loss,
}

impl GameResult {
    pub const LEN: usize = 1;
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameResult::Draw => write!(f, "GameResult::Draw"),
            GameResult::Win => write!(f, "GameResult::Win"),
            GameResult::Loss => write!(f, "GameResult::Loss"),
        }
    }
}

impl Default for GameResult {
    fn default() -> Self {
        GameResult::Draw
    }
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug, PartialEq)]
pub enum Identity {
    Player,
    Bot,
}

impl Identity {
    pub const LEN: usize = 1;
}

impl fmt::Display for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Identity::Player => write!(f, "Identity::Player"),
            Identity::Bot => write!(f, "Identity::Bot"),
        }
    }
}

impl Default for Identity {
    fn default() -> Self {
        Identity::Player
    }
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug, PartialEq)]
pub enum Action {
    None,
    // Arm hits
    ArmShort,
    ArmLong,
    // Leg hits
    LegShortPush,
    LegShort,
    LegLong,
    // Blocks
    BlockMain,
    BlockDown,
    // Moves
    MoveRight,
    MoveLeft,
    MoveSit,
    MoveJump,
    MoveSpecial,
    // Skills
    SkillBigGuy,
    SkillDiva,
    SkillExCop,
    SkillBusinessman,
    SkillYakuza,
    SkillInformer,
}

impl Action {
    pub const LEN: usize = 1;
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::None => write!(f, "Action::None"),
            // Arm hits
            Action::ArmShort => write!(f, "Action::ArmShort"),
            Action::ArmLong => write!(f, "Action::ArmLong"),
            // Leg hits
            Action::LegShortPush => write!(f, "Action::LegShortPush"),
            Action::LegShort => write!(f, "Action::LegShort"),
            Action::LegLong => write!(f, "Action::LegLong"),
            // Blocks
            Action::BlockMain => write!(f, "Action::BlockMain"),
            Action::BlockDown => write!(f, "Action::BlockDown"),
            // Moves
            Action::MoveRight => write!(f, "Action::MoveRight"),
            Action::MoveLeft => write!(f, "Action::MoveLeft"),
            Action::MoveSit => write!(f, "Action::MoveSit"),
            Action::MoveJump => write!(f, "Action::MoveJump"),
            Action::MoveSpecial => write!(f, "Action::MoveSpecial"),
            // Skills
            Action::SkillBigGuy => write!(f, "Action::BigGuy"),
            Action::SkillDiva => write!(f, "Action::Diva"),
            Action::SkillExCop => write!(f, "Action::ExCop"),
            Action::SkillBusinessman => write!(f, "Action::Businessman"),
            Action::SkillYakuza => write!(f, "Action::Yakuza"),
            Action::SkillInformer => write!(f, "Action::Informer"),
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        Action::None
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug, PartialEq)]
pub struct IdentityAction {
    pub who: Identity,
    pub action: Action,
}

const PLAYERS_MAX: usize = 2;
const PLAYER_ACTIONS_MAX: usize = 3 * 9;
pub const ACTIONS_VEC_MAX: usize = PLAYER_ACTIONS_MAX * PLAYERS_MAX;
const ACTIONS_VEC_LEN_MAX: usize = (Action::LEN + Identity::LEN) * ACTIONS_VEC_MAX;

#[account]
#[derive(Default, Debug)]
pub struct GameInfo {
    pub id: u64,
    pub result: GameResult,
    pub actions_log: Vec<IdentityAction>,
}

impl GameInfo {
    pub const LEN: usize = DESCRIMINATOR_LEN + 8 + GameResult::LEN + ACTIONS_VEC_LEN_MAX;
}
