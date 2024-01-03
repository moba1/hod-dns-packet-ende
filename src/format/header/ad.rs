#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ad {
    SuccessDnssecValidationOrSupportedAdBit,
    DnssecValidationFailureOrNotSUpportedAdBit
}

impl Ad {
    pub fn to_hex_string(&self) -> String {
        let value: u8 = self.clone().into();
        format!("{:1}", value)
    }
}

impl std::convert::TryFrom<u8> for Ad {
    type Error = u8;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DnssecValidationFailureOrNotSUpportedAdBit),
            1 => Ok(Self::SuccessDnssecValidationOrSupportedAdBit   ),
            v => Err(v),
        }
    }
}

impl std::convert::Into<u8> for Ad {
    fn into(self) -> u8 {
        match self {
            Self::DnssecValidationFailureOrNotSUpportedAdBit => 0,
            Self::SuccessDnssecValidationOrSupportedAdBit    => 1
        }
    }
}

impl std::convert::From<bool> for Ad {
    fn from(value: bool) -> Self {
        if value {
            Self::SuccessDnssecValidationOrSupportedAdBit
        } else {
            Self::DnssecValidationFailureOrNotSUpportedAdBit
        }
    }
}

impl std::convert::Into<bool> for Ad {
    fn into(self) -> bool {
        match self {
            Self::SuccessDnssecValidationOrSupportedAdBit    => true,
            Self::DnssecValidationFailureOrNotSUpportedAdBit => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_converts_to_raw_value() {
        let value: u8 = Ad::DnssecValidationFailureOrNotSUpportedAdBit.into();
        assert_eq!(value, 0);
        let value: u8 = Ad::SuccessDnssecValidationOrSupportedAdBit.into();
        assert_eq!(value, 1);
    }

    #[test]
    fn it_converts_from_u8() {
        assert_eq!(Ad::try_from(0_u8), Ok(Ad::DnssecValidationFailureOrNotSUpportedAdBit));
        assert_eq!(Ad::try_from(1_u8), Ok(Ad::SuccessDnssecValidationOrSupportedAdBit)   );
    }

    #[test]
    fn it_does_not_convert_from_invalid_u8() {
        assert_eq!(Ad::try_from(2_u8), Err(2_u8));
    }

    #[test]
    fn it_converts_to_hex_string() {
        assert_eq!(Ad::DnssecValidationFailureOrNotSUpportedAdBit.to_hex_string(), "0");
        assert_eq!(Ad::SuccessDnssecValidationOrSupportedAdBit.to_hex_string(),    "1");
    }

    #[test]
    fn it_converts_to_boolean_value() {
        let value: bool = Ad::SuccessDnssecValidationOrSupportedAdBit.into();
        assert_eq!(value, true);
        let value: bool = Ad::DnssecValidationFailureOrNotSUpportedAdBit.into();
        assert_eq!(value, false);
    }

    #[test]
    fn it_converts_from_bool() {
        assert_eq!(Ad::from(false), Ad::DnssecValidationFailureOrNotSUpportedAdBit);
        assert_eq!(Ad::from(true),  Ad::SuccessDnssecValidationOrSupportedAdBit   );
    }
}
