use crate::error::CidrErr;

#[derive(Debug, PartialEq, Eq)]
pub struct Cidr {
    pub ip: [u8; 4],
    pub prefix_len: u8,
}

impl Cidr {
    pub fn parse(raw: &str) -> Result<Self, CidrErr> {
        let Some((ip, prefix_len)) = raw.split_once('/') else {
            return Err(CidrErr::InvalidFormat);
        };

        let prefix_len = prefix_len
            .parse::<u8>()
            .map_err(|_| CidrErr::InvalidFormat)?;

        if prefix_len > 32 {
            return Err(CidrErr::InvalidPrefixLength(prefix_len));
        }

        let octets = ip
            .split('.')
            .map(|part| part.parse::<u8>().map_err(|_| CidrErr::InvalidIpAddress))
            .collect::<Result<Vec<_>, _>>()?;

        let ip: [u8; 4] = octets.try_into().map_err(|_| CidrErr::InvalidIpAddress)?;

        Ok(Self { ip, prefix_len })
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use super::*;

    #[test]
    fn 정상_파싱() {
        let cidr = Cidr::parse("192.168.1.10/24").expect("유효한 CIDR은 파싱되어야 한다");

        assert_eq!(cidr.ip, [192, 168, 1, 10]);
        assert_eq!(cidr.prefix_len, 24);
    }

    #[test]
    fn 최소_prefix() {
        let cidr = Cidr::parse("10.0.0.1/0").expect("유효한 CIDR은 파싱되어야 한다");

        assert_eq!(cidr.ip, [10, 0, 0, 1]);
        assert_eq!(cidr.prefix_len, 0);
    }

    #[test]
    fn 최대_prefix() {
        let cidr = Cidr::parse("10.0.0.1/32").expect("유효한 CIDR은 파싱되어야 한다");

        assert_eq!(cidr.ip, [10, 0, 0, 1]);
        assert_eq!(cidr.prefix_len, 32);
    }

    #[test]
    fn 구분자_누락() {
        let cidr = Cidr::parse("10.0.0.024");
        assert_matches!(cidr, Err(CidrErr::InvalidFormat));
    }

    #[test]
    fn 구분자_중복() {
        let cidr = Cidr::parse("10.0.0.0/24/111");
        assert_matches!(cidr, Err(CidrErr::InvalidFormat));
    }

    #[test]
    fn 옥텟_부족() {
        let cidr = Cidr::parse("10.0.1/0");
        assert_matches!(cidr, Err(CidrErr::InvalidIpAddress));
    }

    #[test]
    fn 옥텟_과다() {
        let cidr = Cidr::parse("10.0.0.0.1/0");
        assert_matches!(cidr, Err(CidrErr::InvalidIpAddress));
    }

    #[test]
    fn 숫자_아닌_prefix() {
        let cidr = Cidr::parse("10.0.0.1/aa");
        assert_matches!(cidr, Err(CidrErr::InvalidFormat));
    }

    #[test]
    fn 범위_큰_prefix() {
        let cidr = Cidr::parse("10.0.0.1/33");
        assert_matches!(cidr, Err(CidrErr::InvalidPrefixLength(33)));
    }

    #[test]
    fn 범위_큰_옥텟() {
        let cidr = Cidr::parse("300.0.0.1/24");
        assert_matches!(cidr, Err(CidrErr::InvalidIpAddress));
    }
}
