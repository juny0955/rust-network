#[derive(Debug, PartialEq, Eq)]
pub enum CidrErr {
    InvalidFormat,
    InvalidIpAddress,
    InvalidPrefixLength(u8),
}
