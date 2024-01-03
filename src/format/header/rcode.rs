#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rcode {
    NoError,
    FormatError,
    ServerFailure,
    NameError,
    NotImplemented,
    Refused,
    YxDomain,
    YxRrset,
    NxRrset,
    NotAuth,
    NotZone,
    Dsotypeni,
    BadversOrBadsig,
    Badkey,
    Badtime,
    Badmode,
    Badname,
    Badalg,
    Badtrunc,
    Badcookie,
    Private(u16),
    Reserved,
}

impl Rcode {
    pub fn to_hex_string(&self) -> String {
        let value: u16 = self.clone().into();
        format!("{:04X}", value)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FromError {
    Unassigned(u16),
    LogicError(u16),
}

impl std::convert::TryFrom<u16> for Rcode {
    type Error = FromError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if (24 <= value && value <= 3840) || (4096 <= value && value <= 65534) {
            return Err(FromError::Unassigned(value))
        } else if 3841 <= value && value <= 4095 {
            return Ok(Self::Private(value))
        }
        match value {
            0     => Ok(Self::NoError        ),
            1     => Ok(Self::FormatError    ),
            2     => Ok(Self::ServerFailure  ),
            3     => Ok(Self::NameError      ),
            4     => Ok(Self::NotImplemented ),
            5     => Ok(Self::Refused        ),
            6     => Ok(Self::YxDomain       ),
            7     => Ok(Self::YxRrset        ),
            8     => Ok(Self::NxRrset        ),
            9     => Ok(Self::NotAuth        ),
            10    => Ok(Self::NotZone        ),
            11    => Ok(Self::Dsotypeni      ),
            16    => Ok(Self::BadversOrBadsig),
            17    => Ok(Self::Badkey         ),
            18    => Ok(Self::Badtime        ),
            19    => Ok(Self::Badmode        ),
            20    => Ok(Self::Badname        ),
            21    => Ok(Self::Badalg         ),
            22    => Ok(Self::Badtrunc       ),
            23    => Ok(Self::Badcookie      ),
            65535 => Ok(Self::Reserved       ),
            12 | 13 | 14 | 15 => Err(FromError::Unassigned(value)),
            v => Err(FromError::LogicError(v)),
        }
    }
}

impl std::convert::Into<u16> for Rcode {
    fn into(self) -> u16 {
        match self {
            Self::NoError         => 0,
            Self::FormatError     => 1,
            Self::ServerFailure   => 2,
            Self::NameError       => 3,
            Self::NotImplemented  => 4,
            Self::Refused         => 5,
            Self::YxDomain        => 6,
            Self::YxRrset         => 7,
            Self::NxRrset         => 8,
            Self::NotAuth         => 9,
            Self::NotZone         => 10,
            Self::Dsotypeni       => 11,
            Self::BadversOrBadsig => 16,
            Self::Badkey          => 17,
            Self::Badtime         => 18,
            Self::Badmode         => 19,
            Self::Badname         => 20,
            Self::Badalg          => 21,
            Self::Badtrunc        => 22,
            Self::Badcookie       => 23,
            Self::Private(v) => v,
            Self::Reserved        => 65535,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_converts_to_raw_value() {
        let value: u16 = Rcode::NoError.into();
        assert_eq!(value, 0);
        let value: u16 = Rcode::FormatError.into();
        assert_eq!(value, 1);
        let value: u16 = Rcode::ServerFailure.into();
        assert_eq!(value, 2);
        let value: u16 = Rcode::NameError.into();
        assert_eq!(value, 3);
        let value: u16 = Rcode::NotImplemented.into();
        assert_eq!(value, 4);
        let value: u16 = Rcode::Refused.into();
        assert_eq!(value, 5);
        let value: u16 = Rcode::YxDomain.into();
        assert_eq!(value, 6);
        let value: u16 = Rcode::YxRrset.into();
        assert_eq!(value, 7);
        let value: u16 = Rcode::NxRrset.into();
        assert_eq!(value, 8);
        let value: u16 = Rcode::NotAuth.into();
        assert_eq!(value, 9);
        let value: u16 = Rcode::NotZone.into();
        assert_eq!(value, 10);
        let value: u16 = Rcode::Dsotypeni.into();
        assert_eq!(value, 11);
        let value: u16 = Rcode::BadversOrBadsig.into();
        assert_eq!(value, 16);
        let value: u16 = Rcode::Badkey.into();
        assert_eq!(value, 17);
        let value: u16 = Rcode::Badtime.into();
        assert_eq!(value, 18);
        let value: u16 = Rcode::Badmode.into();
        assert_eq!(value, 19);
        let value: u16 = Rcode::Badname.into();
        assert_eq!(value, 20);
        let value: u16 = Rcode::Badalg.into();
        assert_eq!(value, 21);
        let value: u16 = Rcode::Badtrunc.into();
        assert_eq!(value, 22);
        let value: u16 = Rcode::Badcookie.into();
        assert_eq!(value, 23);
        let value: u16 = Rcode::Private(3841).into();
        assert_eq!(value, 3841);
        let value: u16 = Rcode::Reserved.into();
        assert_eq!(value, 65535);
    }

    #[test]
    fn it_converts_from_u16() {
        assert_eq!(Rcode::try_from(0_u16),     Ok(Rcode::NoError)          );
        assert_eq!(Rcode::try_from(1_u16),     Ok(Rcode::FormatError)      );
        assert_eq!(Rcode::try_from(2_u16),     Ok(Rcode::ServerFailure)    );
        assert_eq!(Rcode::try_from(3_u16),     Ok(Rcode::NameError)        );
        assert_eq!(Rcode::try_from(4_u16),     Ok(Rcode::NotImplemented)   );
        assert_eq!(Rcode::try_from(5_u16),     Ok(Rcode::Refused)          );
        assert_eq!(Rcode::try_from(6_u16),     Ok(Rcode::YxDomain)         );
        assert_eq!(Rcode::try_from(7_u16),     Ok(Rcode::YxRrset)          );
        assert_eq!(Rcode::try_from(8_u16),     Ok(Rcode::NxRrset)          );
        assert_eq!(Rcode::try_from(9_u16),     Ok(Rcode::NotAuth)          );
        assert_eq!(Rcode::try_from(10_u16),    Ok(Rcode::NotZone)          );
        assert_eq!(Rcode::try_from(11_u16),    Ok(Rcode::Dsotypeni)        );
        assert_eq!(Rcode::try_from(16_u16),    Ok(Rcode::BadversOrBadsig)  );
        assert_eq!(Rcode::try_from(17_u16),    Ok(Rcode::Badkey)           );
        assert_eq!(Rcode::try_from(18_u16),    Ok(Rcode::Badtime)          );
        assert_eq!(Rcode::try_from(19_u16),    Ok(Rcode::Badmode)          );
        assert_eq!(Rcode::try_from(20_u16),    Ok(Rcode::Badname)          );
        assert_eq!(Rcode::try_from(21_u16),    Ok(Rcode::Badalg)           );
        assert_eq!(Rcode::try_from(22_u16),    Ok(Rcode::Badtrunc)         );
        assert_eq!(Rcode::try_from(23_u16),    Ok(Rcode::Badcookie)        );
        assert_eq!(Rcode::try_from(3841_u16),  Ok(Rcode::Private(3841_u16)));
        assert_eq!(Rcode::try_from(4095_u16),  Ok(Rcode::Private(4095_u16)));
        assert_eq!(Rcode::try_from(65535_u16), Ok(Rcode::Reserved)         );
    }

    #[test]
    fn it_does_not_convert_from_invalid_u8() {
        assert_eq!(Rcode::try_from(12_u16),    Err(FromError::Unassigned(12_u16))   );
        assert_eq!(Rcode::try_from(15_u16),    Err(FromError::Unassigned(15_u16))   );
        assert_eq!(Rcode::try_from(24_u16),    Err(FromError::Unassigned(24_u16))   );
        assert_eq!(Rcode::try_from(3840_u16),  Err(FromError::Unassigned(3840_u16)) );
        assert_eq!(Rcode::try_from(4096_u16),  Err(FromError::Unassigned(4096_u16)) );
        assert_eq!(Rcode::try_from(65534_u16), Err(FromError::Unassigned(65534_u16)));
    }

    #[test]
    fn it_converts_to_hex_string() {
        assert_eq!(Rcode::NoError.to_hex_string(),         "0000");
        assert_eq!(Rcode::FormatError.to_hex_string(),     "0001");
        assert_eq!(Rcode::ServerFailure.to_hex_string(),   "0002");
        assert_eq!(Rcode::NameError.to_hex_string(),       "0003");
        assert_eq!(Rcode::NotImplemented.to_hex_string(),  "0004");
        assert_eq!(Rcode::Refused.to_hex_string(),         "0005");
        assert_eq!(Rcode::YxDomain.to_hex_string(),        "0006");
        assert_eq!(Rcode::YxRrset.to_hex_string(),         "0007");
        assert_eq!(Rcode::NxRrset.to_hex_string(),         "0008");
        assert_eq!(Rcode::NotAuth.to_hex_string(),         "0009");
        assert_eq!(Rcode::NotZone.to_hex_string(),         "000A");
        assert_eq!(Rcode::Dsotypeni.to_hex_string(),       "000B");
        assert_eq!(Rcode::BadversOrBadsig.to_hex_string(), "0010");
        assert_eq!(Rcode::Badkey.to_hex_string(),          "0011");
        assert_eq!(Rcode::Badtime.to_hex_string(),         "0012");
        assert_eq!(Rcode::Badmode.to_hex_string(),         "0013");
        assert_eq!(Rcode::Badname.to_hex_string(),         "0014");
        assert_eq!(Rcode::Badalg.to_hex_string(),          "0015");
        assert_eq!(Rcode::Badtrunc.to_hex_string(),        "0016");
        assert_eq!(Rcode::Badcookie.to_hex_string(),       "0017");
        assert_eq!(Rcode::Private(3481).to_hex_string(),   "0D99");
        assert_eq!(Rcode::Reserved.to_hex_string(),        "FFFF")
    }
}
