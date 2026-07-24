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

    pub fn subnet_mask(&self) -> [u8; 4] {
        let mut subnet_mask = [0, 0, 0, 0];

        let finish = self.prefix_len / 8;
        let remaining = self.prefix_len % 8;
        let last_mask = if remaining != 0 {
            u8::MAX << (8 - remaining)
        } else {
            0
        };

        for i in 0..4 {
            if i < finish {
                subnet_mask[i as usize] = 255;
            } else {
                subnet_mask[i as usize] = last_mask;
            }
        }

        subnet_mask
    }

    pub fn network_address(&self) -> [u8; 4] {
        let subnet_mask = self.subnet_mask();

        [
            self.ip[0] & subnet_mask[0],
            self.ip[1] & subnet_mask[1],
            self.ip[2] & subnet_mask[2],
            self.ip[3] & subnet_mask[3],
        ]
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

    #[test]
    fn subnet_mask_계산() {
        let prefix_0 = Cidr::parse("1.1.1.1/0").expect("유요한 CIDR은 파싱되어야 한다");
        let prefix_24 = Cidr::parse("1.1.1.1/24").expect("유요한 CIDR은 파싱되어야 한다");
        let prefix_30 = Cidr::parse("1.1.1.1/30").expect("유요한 CIDR은 파싱되어야 한다");
        let prefix_32 = Cidr::parse("1.1.1.1/32").expect("유요한 CIDR은 파싱되어야 한다");

        assert_eq!(prefix_0.subnet_mask(), [0, 0, 0, 0]);
        assert_eq!(prefix_24.subnet_mask(), [255, 255, 255, 0]);
        assert_eq!(prefix_30.subnet_mask(), [255, 255, 255, 252]);
        assert_eq!(prefix_32.subnet_mask(), [255, 255, 255, 255]);
    }

    #[test]
    fn network_address_계산() {
        let test1 = Cidr::parse("192.168.1.10/24").expect("유요한 CIDR은 파싱되어야 한다");
        let test2 = Cidr::parse("192.168.1.10/30").expect("유요한 CIDR은 파싱되어야 한다");
        let test3 = Cidr::parse("10.0.0.1/0").expect("유요한 CIDR은 파싱되어야 한다");
        let test4 = Cidr::parse("10.0.0.1/32").expect("유요한 CIDR은 파싱되어야 한다");

        assert_eq!(test1.network_address(), [192, 168, 1, 0]);
        assert_eq!(test2.network_address(), [192, 168, 1, 8]);
        assert_eq!(test3.network_address(), [0, 0, 0, 0]);
        assert_eq!(test4.network_address(), [10, 0, 0, 1]);
    }
}
