macro_rules! implement_u8_encoder_and_decoder {
    ($enum_name:ty, $zero_value:path, $one_value:path $(,)?) => {
        impl std::convert::TryFrom<u8> for $enum_name {
            type Error = u8;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    0 => Ok($zero_value),
                    1 => Ok($one_value ),
                    v => Err(v),
                }
            }
        }

        impl std::convert::Into<u8> for $enum_name {
            fn into(self) -> u8 {
                match self {
                    $zero_value => 0,
                    $one_value  => 1
                }
            }
        }
    };    
}
#[cfg(test)]
mod implement_u8_encoder_and_decoder_tests {
    use pretty_assertions::assert_eq;

    #[derive(Debug, PartialEq, Eq)]
    enum OneBitFlagMock {
        ZeroValue,
        OneValue,
    }

    implement_u8_encoder_and_decoder!(
        OneBitFlagMock,
        OneBitFlagMock::ZeroValue,
        OneBitFlagMock::OneValue,
    );

    #[test]
    fn it_converts_to_valid_u8_value() {
        let value: u8 = OneBitFlagMock::ZeroValue.into();
        assert_eq!(value, 0_u8);
        let value: u8 = OneBitFlagMock::OneValue.into();
        assert_eq!(value, 1_u8);
    }

    #[test]
    fn it_converts_from_u8_value() {
        assert_eq!(OneBitFlagMock::try_from(0_u8), Ok(OneBitFlagMock::ZeroValue));
        assert_eq!(OneBitFlagMock::try_from(1_u8), Ok(OneBitFlagMock::OneValue) );
    }

    #[test]
    fn it_does_not_convert_from_invalid_u8_value() {
        assert_eq!(OneBitFlagMock::try_from(2_u8), Err(2_u8));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Qr {
    Query,
    Response
}
implement_u8_encoder_and_decoder!(
    Qr,
    Self::Query,
    Self::Response,
);
#[cfg(test)]
mod qr_flag_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_converts_to_raw_value() {
        let value: u8 = Qr::Query.into();
        assert_eq!(value, 0);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Aa {
    FromAuthority,
    FromNonAuthority
}
implement_u8_encoder_and_decoder!(
    Aa,
    Self::FromNonAuthority,
    Self::FromAuthority,
);
#[cfg(test)]
mod aa_flag_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_converts_to_raw_bit_value() {
        let value: u8 = Aa::FromNonAuthority.into();
        assert_eq!(value, 0);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tc {
    Truncated,
    NotTruncated
}
implement_u8_encoder_and_decoder!(
    Tc,
    Tc::NotTruncated,
    Tc::Truncated,
);
impl std::convert::From<bool> for Tc {
    fn from(value: bool) -> Self {
        if value { Self::Truncated } else { Self::NotTruncated }
    }
}
impl std::convert::Into<bool> for Tc {
    fn into(self) -> bool {
        match self {
            Self::Truncated    => true,
            Self::NotTruncated => false,
        }
    }
}
#[cfg(test)]
mod tc_flag_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_converts_to_raw_value() {
        let value: u8 = Tc::NotTruncated.into();
        assert_eq!(value, 0);
    }

    #[test]
    fn it_converts_to_boolean_value() {
        let value: bool = Tc::NotTruncated.into();
        assert_eq!(value, false);
        let value: bool = Tc::Truncated.into();
        assert_eq!(value, true);
    }

    #[test]
    fn it_converts_from_bool() {
        assert_eq!(Tc::from(false), Tc::NotTruncated);
        assert_eq!(Tc::from(true),  Tc::Truncated   );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rd {
    RecursiveDesired,
    RecursiveUndesired,
}
implement_u8_encoder_and_decoder!(
    Rd,
    Self::RecursiveUndesired,
    Self::RecursiveDesired,
);
#[cfg(test)]
mod rd_flag_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_converts_to_raw_value() {
        let value: u8 = Rd::RecursiveUndesired.into();
        assert_eq!(value, 0);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ra {
    RecursionUnavailable,
    RecursionAvailable
}
implement_u8_encoder_and_decoder!(
    Ra,
    Self::RecursionUnavailable,
    Self::RecursionAvailable,
);
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
mod ra_flag_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_converts_to_raw_value() {
        let value: u8 = Ra::RecursionUnavailable.into();
        assert_eq!(value, 0);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ad {
    SuccessDnssecValidationOrSupportedAdBit,
    DnssecValidationFailureOrNotSupportedAdBit
}
implement_u8_encoder_and_decoder!(
    Ad,
    Self::DnssecValidationFailureOrNotSupportedAdBit,
    Self::SuccessDnssecValidationOrSupportedAdBit,
);
#[cfg(test)]
mod ad_flag_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_converts_to_raw_bit_value() {
        let value: u8 = Ad::DnssecValidationFailureOrNotSupportedAdBit.into();
        assert_eq!(value, 0);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cd {
    DnssecEnabled,
    DnssecForbidden
}
implement_u8_encoder_and_decoder!(
    Cd,
    Self::DnssecEnabled,
    Self::DnssecForbidden,
);
#[cfg(test)]
mod cd_flag_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_converts_to_raw_bit_value() {
        let value: u8 = Cd::DnssecEnabled.into();
        assert_eq!(value, 0);
    }
}
