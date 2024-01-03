#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ra {
    RecursionUnavailable,
    RecursionAvailable
}

impl Ra {
    pub fn to_hex_string(&self) -> String {
        let value: u8 = self.clone().into();
        format!("{:1}", value)
    }
}

impl std::convert::TryFrom<u8> for Ra {
    type Error = u8;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::RecursionUnavailable),
            1 => Ok(Self::RecursionAvailable  ),
            v => Err(v),
        }
    }
}

impl std::convert::Into<u8> for Ra {
    fn into(self) -> u8 {
        match self {
            Self::RecursionUnavailable => 0,
            Self::RecursionAvailable   => 1
        }
    }
}

impl std::convert::From<bool> for Ra {
    fn from(value: bool) -> Self {
        if value { Self::RecursionAvailable } else { Self::RecursionUnavailable }
    }
}

impl std::convert::Into<bool> for Ra {
    fn into(self) -> bool {
        match self {
            Self::RecursionAvailable   => true,
            Self::RecursionUnavailable => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_converts_to_raw_value() {
        let value: u8 = Ra::RecursionUnavailable.into();
        assert_eq!(value, 0);
        let value: u8 = Ra::RecursionAvailable.into();
        assert_eq!(value, 1);
    }

    #[test]
    fn it_converts_from_u8() {
        assert_eq!(Ra::try_from(0_u8), Ok(Ra::RecursionUnavailable));
        assert_eq!(Ra::try_from(1_u8), Ok(Ra::RecursionAvailable)  );
    }

    #[test]
    fn it_does_not_convert_from_invalid_u8() {
        assert_eq!(Ra::try_from(2_u8), Err(2_u8));
    }

    #[test]
    fn it_converts_to_hex_string() {
        assert_eq!(Ra::RecursionUnavailable.to_hex_string(), "0");
        assert_eq!(Ra::RecursionAvailable.to_hex_string(),   "1");
    }

    #[test]
    fn it_converts_to_boolean_value() {
        let value: bool = Ra::RecursionAvailable.into();
        assert_eq!(value, true);
        let value: bool = Ra::RecursionUnavailable.into();
        assert_eq!(value, false);
    }

    #[test]
    fn it_converts_from_bool() {
        assert_eq!(Ra::from(false), Ra::RecursionUnavailable);
        assert_eq!(Ra::from(true),  Ra::RecursionAvailable  );
    }
}
