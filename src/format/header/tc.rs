#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tc {
    Truncated,
    NotTruncated
}

impl Tc {
    pub fn to_hex_string(&self) -> String {
        let value: u8 = self.clone().into();
        format!("{:1}", value)
    }
}

impl std::convert::TryFrom<u8> for Tc {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Tc::NotTruncated),
            1 => Ok(Tc::Truncated   ),
            v => Err(v),
        }
    }
}

impl std::convert::Into<u8> for Tc {
    fn into(self) -> u8 {
        match self {
            Tc::NotTruncated => 0,
            Tc::Truncated    => 1
        }
    }
}

impl std::convert::From<bool> for Tc {
    fn from(value: bool) -> Self {
        if value { Tc::Truncated } else { Tc::NotTruncated }
    }
}

impl std::convert::Into<bool> for Tc {
    fn into(self) -> bool {
        match self {
            Tc::NotTruncated => false,
            Tc::Truncated    => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_converts_to_raw_value() {
        let value: u8 = Tc::Truncated.into();
        assert_eq!(value, 1);
        let value: u8 = Tc::NotTruncated.into();
        assert_eq!(value, 0);
    }

    #[test]
    fn it_converts_from_u8() {
        assert_eq!(Tc::try_from(0_u8), Ok(Tc::NotTruncated));
        assert_eq!(Tc::try_from(1_u8), Ok(Tc::Truncated)   );
    }

    #[test]
    fn it_does_not_convert_from_invalid_u8() {
        assert_eq!(Tc::try_from(3_u8 ), Err(3_u8));
    }

    #[test]
    fn it_converts_to_hex_string() {
        assert_eq!(Tc::NotTruncated.to_hex_string(), "0");
        assert_eq!(Tc::Truncated.to_hex_string(),    "1");
    }

    #[test]
    fn it_converts_to_boolean_value() {
        let value: bool = Tc::Truncated.into();
        assert_eq!(value, true);
        let value: bool = Tc::NotTruncated.into();
        assert_eq!(value, false);
    }

    #[test]
    fn it_converts_from_bool() {
        assert_eq!(Tc::from(false), Tc::NotTruncated);
        assert_eq!(Tc::from(true),  Tc::Truncated   );
    }
}


