#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cd {
    DnssecEnabled,
    DnssecForbidden
}

impl Cd {
    pub fn to_hex_string(&self) -> String {
        let value: u8 = self.clone().into();
        format!("{:1}", value)
    }
}

impl std::convert::TryFrom<u8> for Cd {
    type Error = u8;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DnssecEnabled  ),
            1 => Ok(Self::DnssecForbidden),
            v => Err(v),
        }
    }
}

impl std::convert::Into<u8> for Cd {
    fn into(self) -> u8 {
        match self {
            Self::DnssecEnabled   => 0,
            Self::DnssecForbidden => 1
        }
    }
}

impl std::convert::From<bool> for Cd {
    fn from(value: bool) -> Self {
        if value { Self::DnssecForbidden } else { Self::DnssecEnabled }
    }
}

impl std::convert::Into<bool> for Cd {
    fn into(self) -> bool {
        match self {
            Self::DnssecForbidden => true,
            Self::DnssecEnabled   => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_converts_to_raw_value() {
        let value: u8 = Cd::DnssecEnabled.into();
        assert_eq!(value, 0);
        let value: u8 = Cd::DnssecForbidden.into();
        assert_eq!(value, 1);
    }

    #[test]
    fn it_converts_from_u8() {
        assert_eq!(Cd::try_from(0_u8), Ok(Cd::DnssecEnabled)  );
        assert_eq!(Cd::try_from(1_u8), Ok(Cd::DnssecForbidden));
    }

    #[test]
    fn it_does_not_convert_from_invalid_u8() {
        assert_eq!(Cd::try_from(2_u8), Err(2_u8));
    }

    #[test]
    fn it_converts_to_hex_string() {
        assert_eq!(Cd::DnssecEnabled.to_hex_string(),   "0");
        assert_eq!(Cd::DnssecForbidden.to_hex_string(), "1");
    }

    #[test]
    fn it_converts_to_boolean_value() {
        let value: bool = Cd::DnssecForbidden.into();
        assert_eq!(value, true);
        let value: bool = Cd::DnssecEnabled.into();
        assert_eq!(value, false);
    }

    #[test]
    fn it_converts_from_bool() {
        assert_eq!(Cd::from(false), Cd::DnssecEnabled  );
        assert_eq!(Cd::from(true),  Cd::DnssecForbidden);
    }
}
