macro_rules! implement_u16_encoder_and_decoder {
    ($struct_name:ident) => {
        impl std::convert::From<u16> for $struct_name {
            fn from(value: u16) -> Self {
                $struct_name(value)
            }
        }

        impl std::convert::Into<u16> for $struct_name {
            fn into(self) -> u16 {
                let $struct_name(value) = self;
                value
            }
        }
    };
}
#[cfg(test)]
mod implement_u16_encoder_and_decoder_tests {
    use pretty_assertions::assert_eq;

    #[derive(Debug, PartialEq, Eq)]
    struct MockCount(u16);

    implement_u16_encoder_and_decoder!(MockCount);

    #[test]
    fn it_converts_to_u16_value() {
        let value: u16 = MockCount(255_u16).into();
        assert_eq!(value, 255_u16);
    }

    #[test]
    fn it_converts_from_u16_value() {
        assert_eq!(MockCount::from(0_u16), MockCount(0_u16));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QdZoCount(u16);
implement_u16_encoder_and_decoder!(QdZoCount);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnPrCount(u16);
implement_u16_encoder_and_decoder!(AnPrCount);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NsUpCount(u16);
implement_u16_encoder_and_decoder!(NsUpCount);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Arcount(u16);
implement_u16_encoder_and_decoder!(Arcount);
