//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

pub(crate) mod r#buy_exact_in;
pub(crate) mod r#buy_exact_out;
pub(crate) mod r#claim_platform_fee;
pub(crate) mod r#claim_vested_token;
pub(crate) mod r#collect_fee;
pub(crate) mod r#collect_migrate_fee;
pub(crate) mod r#create_config;
pub(crate) mod r#create_platform_config;
pub(crate) mod r#create_vesting_account;
pub(crate) mod r#initialize;
pub(crate) mod r#migrate_to_amm;
pub(crate) mod r#migrate_to_cpswap;
pub(crate) mod r#sell_exact_in;
pub(crate) mod r#sell_exact_out;
pub(crate) mod r#update_config;
pub(crate) mod r#update_platform_config;

pub use self::{
    r#buy_exact_in::*, r#buy_exact_out::*, r#claim_platform_fee::*, r#claim_vested_token::*,
    r#collect_fee::*, r#collect_migrate_fee::*, r#create_config::*, r#create_platform_config::*,
    r#create_vesting_account::*, r#initialize::*, r#migrate_to_amm::*, r#migrate_to_cpswap::*,
    r#sell_exact_in::*, r#sell_exact_out::*, r#update_config::*, r#update_platform_config::*,
};
