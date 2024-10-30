use anyhow::anyhow;
use anyhow::Result;
use itertools::Itertools;

pub type DecodedValue = alloy::dyn_abi::DynSolValue;

pub trait DecodedValueExt {
    /// Creates a fixed-byte decoded value from a slice.
    /// Fails if the source slice exceeds 32 bytes.
    fn fixed_bytes_from_slice(s: &[u8]) -> Result<DecodedValue>;

    /// Returns the decoded value as a string.
    /// The resulting string contains no type information.
    fn to_string(&self) -> String;
}

impl DecodedValueExt for DecodedValue {
    fn fixed_bytes_from_slice(s: &[u8]) -> Result<Self> {
        let num_bytes = s.len();

        if num_bytes > 32 {
            return Err(anyhow!(
                "input slice must contain a maximum of 32 bytes, got {num_bytes}"
            ));
        }

        let mut bytes = [0u8; 32];
        bytes[..num_bytes].copy_from_slice(s);

        Ok(Self::FixedBytes(bytes.into(), num_bytes))
    }

    fn to_string(&self) -> String {
        let s = |v: &[DecodedValue]| v.iter().map(|x| x.to_string()).collect_vec().join(",");

        match self {
            Self::Bool(v) => v.to_string(),
            Self::Int(v, _) => format!("{v:x}"),
            Self::Uint(v, _) => format!("{v:x}"),
            Self::FixedBytes(v, _) => hex::encode(v),
            Self::Address(v) => format!("{v:x}"),
            Self::Function(v) => format!("{v:x}"),
            Self::Bytes(v) => hex::encode(v),
            Self::String(v) => v.to_owned(),
            Self::Array(v) => format!("[{}]", s(v)),
            Self::FixedArray(v) => format!("[{}]", s(v)),
            Self::Tuple(v) => format!("({})", s(v)),
        }
    }
}
