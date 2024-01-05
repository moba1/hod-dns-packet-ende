#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Id(pub u16);

impl Id {
    pub fn to_hex_string(&self) -> String {
        format!("{:04X}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_converts_to_hex_string() {
        assert_eq!(Id(0x0000_u16).to_hex_string(), "0000");
        assert_eq!(Id(0xffff_u16).to_hex_string(), "FFFF");
    }
}
