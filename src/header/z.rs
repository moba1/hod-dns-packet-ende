#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Z;

impl Z {
    pub fn to_hex_string(&self) -> String {
        "0".to_string()
    }
}

impl std::convert::TryFrom<u8> for Z {
    type Error = u8;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Z),
            v => Err(v),
        }
    }
}

impl std::convert::Into<u8> for Z {
    fn into(self) -> u8 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_converts_to_raw_value() {
        let value: u8 = Z.into();
        assert_eq!(value, 0);
    }

    #[test]
    fn it_converts_from_u8() {
        assert_eq!(Z::try_from(0_u8), Ok(Z));
    }

    #[test]
    fn it_does_not_convert_from_invalid_u8() {
        assert_eq!(Z::try_from(1_u8), Err(1_u8));
    }

    #[test]
    fn it_converts_to_hex_string() {
        assert_eq!(Z.to_hex_string(), "0");
    }
}
