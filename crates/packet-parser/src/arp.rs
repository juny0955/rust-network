#[derive(Debug)]
pub struct Arp {
    hardware_type: u16,
    protocol_type: u16,
    hardware_length: u8,
    protocol_length: u8,
    operation: ArpOperation,
    sender_mac: [u8; 6],
    sender_ip: [u8; 4],
    target_mac: [u8; 6],
    target_ip: [u8; 4],
}

#[derive(Debug, PartialEq, Eq)]
pub enum ArpOperation {
    Request,
    Reply,
}

impl ArpOperation {
    pub fn parse(byte: u16) -> Result<Self, ArpError> {
        match byte {
            1 => Ok(ArpOperation::Request),
            2 => Ok(ArpOperation::Reply),
            _ => Err(ArpError::UnspportedOperation),
        }
    }
}

#[derive(Debug)]
pub enum ArpError {
    InvalidByteLength,
    UnspportedOperation,
}

impl Arp {
    pub fn parse(bytes: &[u8]) -> Result<Self, ArpError> {
        if bytes.len() < 28 {
            return Err(ArpError::InvalidByteLength);
        }

        let hardware_type = u16::from_be_bytes([bytes[0], bytes[1]]);
        let protocol_type = u16::from_be_bytes([bytes[2], bytes[3]]);
        let hardware_length = bytes[4];
        let protocol_length = bytes[5];
        let operation = ArpOperation::parse(u16::from_be_bytes([bytes[6], bytes[7]]))?;
        let sender_mac = bytes[8..14]
            .try_into()
            .map_err(|_| ArpError::InvalidByteLength)?;
        let sender_ip = bytes[14..18]
            .try_into()
            .map_err(|_| ArpError::InvalidByteLength)?;
        let target_mac = bytes[18..24]
            .try_into()
            .map_err(|_| ArpError::InvalidByteLength)?;
        let target_ip = bytes[24..28]
            .try_into()
            .map_err(|_| ArpError::InvalidByteLength)?;

        Ok(Self {
            hardware_type,
            protocol_type,
            hardware_length,
            protocol_length,
            operation,
            sender_mac,
            sender_ip,
            target_mac,
            target_ip,
        })
    }
}
