#[derive(Debug, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum CidrErr {
    InvalidFormat,
    InvalidIpAddress,
    InvalidPrefixLength(u8),
}
