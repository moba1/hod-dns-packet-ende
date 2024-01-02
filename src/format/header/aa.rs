#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Aa {
    FromAuthority,
    FromNonAuthority
}

impl Aa {
    pub fn to_hex_string(&self) -> String {
        let value: u8 = self.clone().into();
        format!("{:1}", value)
    }
}

impl std::convert::TryFrom<u8> for Aa {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::FromNonAuthority),
            1 => Ok(Self::FromAuthority   ),
            v => Err(v),
        }
    }
}

impl std::convert::Into<u8> for Aa {
    fn into(self) -> u8 {
        match self {
            Self::FromNonAuthority => 0,
            Self::FromAuthority    => 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_converts_to_raw_value() {
        let value: u8 = Aa::FromAuthority.into();
        assert_eq!(value, 1);
        let value: u8 = Aa::FromNonAuthority.into();
        assert_eq!(value, 0);
    }

    #[test]
    fn it_converts_from_u8() {
        assert_eq!(Aa::try_from(0_u8), Ok(Aa::FromNonAuthority));
        assert_eq!(Aa::try_from(1_u8), Ok(Aa::FromAuthority)   );
    }

    #[test]
    fn it_does_not_convert_from_invalid_u8() {
        assert_eq!(Aa::try_from(3_u8 ), Err(3_u8));
    }

    #[test]
    fn it_converts_to_hex_string() {
        assert_eq!(Aa::FromNonAuthority.to_hex_string(), "0");
        assert_eq!(Aa::FromAuthority.to_hex_string(),    "1");
    }
}


