mod decoded_param;
mod decoded_value;

pub use alloy::dyn_abi::EventExt;
pub use alloy::dyn_abi::FunctionExt;
pub use alloy::dyn_abi::JsonAbiExt as ContractExt;

pub use alloy::json_abi::Event;
pub use alloy::json_abi::Function;
pub use alloy::json_abi::StateMutability;

pub use alloy::primitives::LogData;
pub use alloy::primitives::I256;
pub use alloy::primitives::U256;

pub use self::decoded_param::decode_log;
pub use self::decoded_param::DecodedParam;
pub use self::decoded_value::DecodedValue;
pub use self::decoded_value::DecodedValueExt;

pub type ParamType = alloy::dyn_abi::DynSolType;

pub type Contract = alloy::json_abi::JsonAbi;
