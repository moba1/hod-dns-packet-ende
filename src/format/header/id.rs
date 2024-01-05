#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Id(pub u16);

impl std::convert::Into<u16> for Id {
    fn into(self) -> u16 {
        self.0
    }
}

impl std::convert::From<u16> for Id {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_converts_from_u16_value() {
        assert_eq!(Id::from(0xABCD), Id(0xABCD));
    }

    #[test]
    fn it_converts_to_u16_value() {
        let value: u16 = Id(0xABCD).into();
        assert_eq!(value, 0xABCD_u16);
    }
}
