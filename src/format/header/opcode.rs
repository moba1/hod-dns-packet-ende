#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Query,
    InverseQuery,
    ServerStatus,
    Notify,
    Update,
    DnsStatefulOperations,
}

impl Opcode {
    pub fn to_hex_string(&self) -> String {
        let value: u8 = self.clone().into();
        format!("{:02X}", value)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FromError {
    Unassigned(u8),
    InvalidRange(u8),
}

impl std::convert::TryFrom<u8> for Opcode {
    type Error = FromError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Query                ),
            1 => Ok(Self::InverseQuery         ),
            2 => Ok(Self::ServerStatus         ),
            4 => Ok(Self::Notify               ),
            5 => Ok(Self::Update               ),
            6 => Ok(Self::DnsStatefulOperations),
            3 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => Err(FromError::Unassigned(value)),
            v => Err(FromError::InvalidRange(v)),
        }
    }
}

impl std::convert::Into<u8> for Opcode {
    fn into(self) -> u8 {
        match self {
            Self::Query                 => 0,
            Self::InverseQuery          => 1,
            Self::ServerStatus          => 2,
            Self::Notify                => 4,
            Self::Update                => 5,
            Self::DnsStatefulOperations => 6,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_converts_to_raw_value() {
        let value: u8 = Opcode::Query.into();
        assert_eq!(value, 0);
        let value: u8 = Opcode::InverseQuery.into();
        assert_eq!(value, 1);
        let value: u8 = Opcode::ServerStatus.into();
        assert_eq!(value, 2);
        let value: u8 = Opcode::Notify.into();
        assert_eq!(value, 4);
        let value: u8 = Opcode::Update.into();
        assert_eq!(value, 5);
        let value: u8 = Opcode::DnsStatefulOperations.into();
        assert_eq!(value, 6);
    }

    #[test]
    fn it_converts_from_u8() {
        assert_eq!(Opcode::try_from(0_u8), Ok(Opcode::Query)                );
        assert_eq!(Opcode::try_from(1_u8), Ok(Opcode::InverseQuery)         );
        assert_eq!(Opcode::try_from(2_u8), Ok(Opcode::ServerStatus)         );
        assert_eq!(Opcode::try_from(4_u8), Ok(Opcode::Notify)               );
        assert_eq!(Opcode::try_from(5_u8), Ok(Opcode::Update)               );
        assert_eq!(Opcode::try_from(6_u8), Ok(Opcode::DnsStatefulOperations));
    }

    #[test]
    fn it_does_not_convert_from_invalid_u8() {
        assert_eq!(Opcode::try_from(3_u8),  Err(FromError::Unassigned(3_u8))   );
        assert_eq!(Opcode::try_from(7_u8),  Err(FromError::Unassigned(7_u8))   );
        assert_eq!(Opcode::try_from(16_u8), Err(FromError::InvalidRange(16_u8)));
    }

    #[test]
    fn it_converts_to_hex_string() {
        assert_eq!(Opcode::Query.to_hex_string(),                 "00");
        assert_eq!(Opcode::InverseQuery.to_hex_string(),          "01");
        assert_eq!(Opcode::ServerStatus.to_hex_string(),          "02");
        assert_eq!(Opcode::Notify.to_hex_string(),                "04");
        assert_eq!(Opcode::Update.to_hex_string(),                "05");
        assert_eq!(Opcode::DnsStatefulOperations.to_hex_string(), "06");
    }
}

