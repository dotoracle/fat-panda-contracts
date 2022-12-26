//! Constants used by the ERC20 contract.

/// Name of named-key for `name`.
pub const NAME_KEY_NAME: &str = "name";
/// Name of named-key for `symbol`
pub const SYMBOL_KEY_NAME: &str = "symbol";
/// Name of named-key for `decimals`
pub const DECIMALS_KEY_NAME: &str = "decimals";
/// Name of named-key for `contract`
pub const ERC20_TOKEN_CONTRACT_KEY_NAME: &str = "erc20_token_contract";
/// Name of dictionary-key for `balances`
pub const BALANCES_KEY_NAME: &str = "balances";
/// Name of dictionary-key for `allowances`
pub const ALLOWANCES_KEY_NAME: &str = "allowances";
/// Name of named-key for `total_supply`
pub const TOTAL_SUPPLY_KEY_NAME: &str = "total_supply";

/// Name of `name` entry point.
pub const NAME_ENTRY_POINT_NAME: &str = "name";
pub const ENTRY_POINT_INIT : &str = "init";
pub const INIT_ENTRY_POINT_NAME : &str = "init";
/// Name of `symbol` entry point.
pub const SYMBOL_ENTRY_POINT_NAME: &str = "symbol";
/// Name of `decimals` entry point.
pub const DECIMALS_ENTRY_POINT_NAME: &str = "decimals";
/// Name of `balance_of` entry point.
pub const BALANCE_OF_ENTRY_POINT_NAME: &str = "balance_of";
/// Name of `transfer` entry point.
pub const TRANSFER_ENTRY_POINT_NAME: &str = "transfer";
/// Name of `approve` entry point.
pub const APPROVE_ENTRY_POINT_NAME: &str = "approve";
/// Name of `allowance` entry point.
pub const ALLOWANCE_ENTRY_POINT_NAME: &str = "allowance";
/// Name of `transfer_from` entry point.
pub const TRANSFER_FROM_ENTRY_POINT_NAME: &str = "transfer_from";
/// Name of `total_supply` entry point.
pub const TOTAL_SUPPLY_ENTRY_POINT_NAME: &str = "total_supply";

/// Name of `address` runtime argument.
pub const ADDRESS_RUNTIME_ARG_NAME: &str = "address";
/// Name of `owner` runtime argument.
pub const OWNER_RUNTIME_ARG_NAME: &str = "owner";
/// Name of `spender` runtime argument.
pub const SPENDER_RUNTIME_ARG_NAME: &str = "spender";
/// Name of `amount` runtime argument.
pub const AMOUNT_RUNTIME_ARG_NAME: &str = "amount";
/// Name of `recipient` runtime argument.
pub const RECIPIENT_RUNTIME_ARG_NAME: &str = "recipient";
/// Name of `name` runtime argument.
pub const NAME_RUNTIME_ARG_NAME: &str = "name";
/// Name of `symbol` runtime argument.
pub const SYMBOL_RUNTIME_ARG_NAME: &str = "symbol";
/// Name of `decimals` runtime argument.
pub const DECIMALS_RUNTIME_ARG_NAME: &str = "decimals";
/// Name of `total_supply` runtime argument.
pub const TOTAL_SUPPLY_RUNTIME_ARG_NAME: &str = "total_supply";

pub const REDEEM_ENTRY_POINT_NAME : &str = "redeem";
pub const DEPOSIT_ENTRY_POINT_NAME : &str = "deposit";
pub const SUPPORTED_TOKEN_DECIMALS : &str = "supported_token_decimals";
pub const ARG_SUPPORTED_TOKEN : &str = "supported_token";
pub const SUPPORTED_TOKEN : &str = "supported_token";
pub const ARG_ENABLED : &str = "enabled";
pub const ENABLED : &str = "enabled";
pub const ARG_DECIMALS : &str = "decimals";
pub const ARG_FEE : &str = "fee";
pub const FEE : &str = "fee";
pub const FEE_RECEIVER : &str = "fee_receiver";
pub const CONTRACT_OWNER : &str = "contract_owner";
///Redeem token
pub const REDEEM_TOKEN_RUNTIME_ARG_NAME : &str = "redeem_token";
///Deposit token
pub const DEPOSIT_TOKEN_RUNTIME_ARG_NAME : &str = "deposit_token";
pub const SET_SUPPORTED_TOKEN_ENTRY_POINT_NAME : &str = "set_supported_token";
pub const SET_SUPPORTED_TOKEN_DECIMALS_ENTRY_POINT_NAME : &str = "set_supported_token_decimals";
pub const CONTRACT_NAME : &str = "contract_name"; 
pub const SET_FEE_ENTRY_POINT_NAME : &str = "set_fee";
