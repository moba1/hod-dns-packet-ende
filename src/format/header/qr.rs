#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Qr {
    Query,
    Response
}

impl Qr {
    pub fn to_hex_string(&self) -> String {
        let value: u8 = self.clone().into();
        format!("{:1}", value)
    }
}

impl std::convert::TryFrom<u8> for Qr {
    type Error = u8;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Query   ),
            1 => Ok(Self::Response),
            v => Err(v),
        }
    }
}

impl std::convert::Into<u8> for Qr {
    fn into(self) -> u8 {
        match self {
            Self::Query    => 0,
            Self::Response => 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_converts_to_raw_value() {
        let value: u8 = Qr::Query.into();
        assert_eq!(value, 0);
        let value: u8 = Qr::Response.into();
        assert_eq!(value, 1);
    }

    #[test]
    fn it_converts_from_u8() {
        assert_eq!(Qr::try_from(0_u8), Ok(Qr::Query)   );
        assert_eq!(Qr::try_from(1_u8), Ok(Qr::Response));
    }

    #[test]
    fn it_does_not_convert_from_invalid_u8() {
        assert_eq!(Qr::try_from(2_u8), Err(2_u8));
    }

    #[test]
    fn it_converts_to_hex_string() {
        assert_eq!(Qr::Query.to_hex_string(),    "0");
        assert_eq!(Qr::Response.to_hex_string(), "1");
    }
}
