#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rd {
    RecursiveQuery,
    NonRecursiveQuery
}

impl Rd {
    pub fn to_hex_string(&self) -> String {
        let value: u8 = self.clone().into();
        format!("{:1}", value)
    }
}

impl std::convert::TryFrom<u8> for Rd {
    type Error = u8;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Rd::NonRecursiveQuery),
            1 => Ok(Rd::RecursiveQuery   ),
            v => Err(v),
        }
    }
}

impl std::convert::Into<u8> for Rd {
    fn into(self) -> u8 {
        match self {
            Rd::NonRecursiveQuery => 0,
            Rd::RecursiveQuery    => 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_converts_to_raw_value() {
        let value: u8 = Rd::NonRecursiveQuery.into();
        assert_eq!(value, 0);
        let value: u8 = Rd::RecursiveQuery.into();
        assert_eq!(value, 1);
    }

    #[test]
    fn it_converts_from_u8() {
        assert_eq!(Rd::try_from(0_u8), Ok(Rd::NonRecursiveQuery));
        assert_eq!(Rd::try_from(1_u8), Ok(Rd::RecursiveQuery)   );
    }

    #[test]
    fn it_does_not_convert_from_invalid_u8() {
        assert_eq!(Rd::try_from(2_u8), Err(2_u8));
    }

    #[test]
    fn it_converts_to_hex_string() {
        assert_eq!(Rd::NonRecursiveQuery.to_hex_string(), "0");
        assert_eq!(Rd::RecursiveQuery.to_hex_string(),    "1");
    }
}
