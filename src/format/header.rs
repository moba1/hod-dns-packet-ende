use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub mod one_bit_flag;
pub mod id;
pub mod opcode;
pub mod z;
pub mod rcode;
pub mod count;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, derive(PartialEq, Eq))]
struct Header {
    pub id:          id::Id,
    pub qr:          one_bit_flag::Qr,
    pub opcode:      opcode::Opcode,
    pub aa:          one_bit_flag::Aa,
    pub tc:          one_bit_flag::Tc,
    pub rd:          one_bit_flag::Rd,
    pub ra:          one_bit_flag::Ra,
    pub z:           z::Z,
    pub ad:          one_bit_flag::Ad,
    pub cd:          one_bit_flag::Cd,
    pub rcode:       rcode::Rcode,
    pub qd_zo_count: count::QdZoCount,
    pub an_pr_count: count::AnPrCount,
    pub ns_up_count: count::NsUpCount,
    pub arcount:     count::Arcount,
}

#[derive(Debug)]
struct InvalidValueError<InvalidValue>
    where InvalidValue: std::fmt::Display
{
    name: String,
    invalid_value: InvalidValue,
    additional_info: Option<String>,
}

impl<InvalidValue> std::fmt::Display for InvalidValueError<InvalidValue>
    where InvalidValue: std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.additional_info.is_none() {
            write!(f, "found invalid {} value: {}", self.name, self.invalid_value)
        } else {
            write!(f, "found invalid {} value ({}): {}", self.name, self.additional_info.clone().unwrap(), self.invalid_value)
        }
    }
}

impl<InvalidValue> std::error::Error for InvalidValueError<InvalidValue>
    where InvalidValue: std::fmt::Debug + std::fmt::Display {}

#[derive(Debug)]
struct HeaderReadError {
    cause: String,
    range: (u8, u8),
}

impl std::fmt::Display for HeaderReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "cannot read header from {} to {} bit", self.range.0, self.range.1)?;
        write!(f, "{}", self.cause)
    }
}

impl std::error::Error for HeaderReadError {}

#[derive(Debug)]
pub struct HeaderWriteError {
    cause: String,
    property_name: String,
}

impl std::fmt::Display for HeaderWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "cannot write header property: {}", self.property_name)?;
        write!(f, "{}", self.cause)
    }
}

impl std::error::Error for HeaderWriteError {}

impl Header {
    pub fn decode(header_buffer: &[u8]) -> Result<Header, Box<dyn std::error::Error>> {
        let mut buffer = Cursor::new(header_buffer);
        let id = buffer.read_u16::<BigEndian>()
            .map_err(|e| HeaderReadError { cause: e.to_string(), range: (1, 16) })?;
        let chunk = buffer.read_u16::<BigEndian>()
            .map_err(|e| HeaderReadError { cause: e.to_string(), range: (17, 32) })?;
        let qr     = ((chunk & 0b1000_0000_0000_0000_u16) >> 15) as u8;
        let opcode = ((chunk & 0b0111_1000_0000_0000_u16) >> 11) as u8;
        let aa     = ((chunk & 0b0000_0100_0000_0000_u16) >> 10) as u8;
        let tc     = ((chunk & 0b0000_0010_0000_0000_u16) >>  9) as u8;
        let rd     = ((chunk & 0b0000_0001_0000_0000_u16) >>  8) as u8;
        let ra     = ((chunk & 0b0000_0000_1000_0000_u16) >>  7) as u8;
        let z      = ((chunk & 0b0000_0000_0100_0000_u16) >>  6) as u8;
        let ad     = ((chunk & 0b0000_0000_0010_0000_u16) >>  5) as u8;
        let cd     = ((chunk & 0b0000_0000_0001_0000_u16) >>  4) as u8;
        let rcode     =  chunk & 0b0000_0000_0000_1111_u16;
        let qd_zo_count = buffer.read_u16::<BigEndian>()
            .map(count::QdZoCount)
            .map_err(|e| HeaderReadError { cause: e.to_string(), range: (33, 48) })?;
        let an_pr_count  = buffer.read_u16::<BigEndian>()
            .map(count::AnPrCount)
            .map_err(|e| HeaderReadError { cause: e.to_string(), range: (49, 64) })?;
        let ns_up_count = buffer.read_u16::<BigEndian>()
            .map(count::NsUpCount)
            .map_err(|e| HeaderReadError { cause: e.to_string(), range: (65, 80) })?;
        let arcount = buffer.read_u16::<BigEndian>()
            .map(count::Arcount)
            .map_err(|e| HeaderReadError { cause: e.to_string(), range: (81, 96) })?;

        let qr = one_bit_flag::Qr::try_from(qr)
            .map_err(|invalid_value| InvalidValueError { name: "QR".to_string(), invalid_value, additional_info: None })?;
        let opcode = opcode::Opcode::try_from(opcode)
            .map_err(
                |invalid_value| {
                    let cause = match invalid_value {
                        opcode::FromError::InvalidRange(invalid_value) => (invalid_value, "range invalid"     ),
                        opcode::FromError::Unassigned(invalid_value)   => (invalid_value, "unassigned in RFC"),
                    };
                    InvalidValueError {
                        name: "OPCODE".to_string(),
                        invalid_value: cause.0,
                        additional_info: Some(cause.1.to_string()),
                    }
                }
            )?;
        let aa = one_bit_flag::Aa::try_from(aa)
            .map_err(|invalid_value| InvalidValueError { name: "AA".to_string(), invalid_value, additional_info: None })?;
        let tc = one_bit_flag::Tc::try_from(tc)
            .map_err(|invalid_value| InvalidValueError { name: "TC".to_string(), invalid_value, additional_info: None })?;
        let rd = one_bit_flag::Rd::try_from(rd)
            .map_err(|invalid_value| InvalidValueError { name: "RD".to_string(), invalid_value, additional_info: None })?;
        let ra = one_bit_flag::Ra::try_from(ra)
            .map_err(|invalid_value| InvalidValueError { name: "RA".to_string(), invalid_value, additional_info: None })?;
        if z != 0_u8 {
            Err(InvalidValueError { name: "Z".to_string(), invalid_value: z, additional_info: None })?;
        }
        let ad = one_bit_flag::Ad::try_from(ad)
            .map_err(|invalid_value| InvalidValueError { name: "AD".to_string(), invalid_value, additional_info: None })?;
        let cd = one_bit_flag::Cd::try_from(cd)
            .map_err(|invalid_value| InvalidValueError { name: "CD".to_string(), invalid_value, additional_info: None })?;
        let rcode = rcode::Rcode::try_from(rcode)
            .map_err(
                |invalid_value| {
                    let cause = match invalid_value {
                        rcode::FromError::LogicError(value) => (value, "logic error"),
                        rcode::FromError::Unassigned(value) => (value, "unassigned in RFC")
                    };
                    InvalidValueError {
                        name: "RCODE".to_string(),
                        invalid_value: cause.0,
                        additional_info: Some(cause.1.to_string()),
                    }
                }
            )?;

        Ok(Header {
            id: id::Id(id),
            qr,
            opcode,
            aa,
            tc,
            rd,
            ra,
            z: z::Z,
            ad,
            cd,
            rcode,
            qd_zo_count,
            an_pr_count,
            arcount,
            ns_up_count
        })
    }

    pub fn encode(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut buffer = vec![];

        buffer.write_u16::<BigEndian>(self.id.into())
            .map_err(|e| HeaderWriteError { cause: e.to_string(), property_name: "ID".to_string() })?;
        let qr: u8     = self.qr.into();
        let opcode: u8 = self.opcode.into();
        let aa: u8     = self.aa.into();
        let tc: u8     = self.tc.into();
        let rd: u8     = self.rd.into();
        let ra: u8     = self.rd.into();
        let ad: u8     = self.ad.into();
        let cd: u8     = self.cd.into();
        let rcode: u16 = self.rcode.into();
        let chunk: u16 =
              ((qr as u16)     << 15)
            | ((opcode as u16) << 11)
            | ((aa as u16)     << 10)
            | ((tc as u16)     <<  9)
            | ((rd as u16)     <<  8)
            | ((ra as u16)     <<  7)
            | ((ad as u16)     <<  5)
            | ((cd as u16)     <<  4)
            | rcode;
        buffer.write_u16::<BigEndian>(chunk)
            .map_err(|e| HeaderWriteError { cause: e.to_string(), property_name: format!("{} ~ {} bit", 17, 32) })?;
        buffer.write_u16::<BigEndian>(self.qd_zo_count.into())
            .map_err(|e| HeaderWriteError { cause: e.to_string(), property_name: "QDCOUNT / ZOCOUNT".to_string() })?;
        buffer.write_u16::<BigEndian>(self.an_pr_count.into())
            .map_err(|e| HeaderWriteError { cause: e.to_string(), property_name: "ANCOUNT / PRCOUNT".to_string() })?;
        buffer.write_u16::<BigEndian>(self.ns_up_count.into())
            .map_err(|e| HeaderWriteError { cause: e.to_string(), property_name: "NSCOUNT / UPCOUNT".to_string() })?;
        buffer.write_u16::<BigEndian>(self.arcount.into())
            .map_err(|e| HeaderWriteError { cause: e.to_string(), property_name: "ARCOUNT".to_string() })?;

        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_decodes_from_u8_slice() {
        let header: &[u8] = &[
            0xAB, 0xCD, // ID = 0xABCD
            0b1_0000_1_1_0, 0b1_0_1_0_0000,
            //^ ^    ^ ^ ^       ^ ^ ^ ^ ^
            //| |    | | |       | | | | `- RCODE  = No error
            //| |    | | |       | | | `--- CD     = DNSSEC enabled
            //| |    | | |       | | `----- AD     = Success DNSSEC validation / Supported AD bit
            //| |    | | |       | `------- Z      = reserved (only 0)
            //| |    | | |        `-------- RA     = Supported recuesive query
            //| |    | | `----------------- RD     = Server recursion undesired
            //| |    | `------------------- TC     = Truncated DNS packet
            //| |    `--------------------- AA     = from Authoritative Server
            //| `-------------------------- OPCODE = Query
            //`---------------------------- QR     = Response
            0x00, 0x01, // QDCOUNT / ZOCOUNT = 0x0001
            0x01, 0x02, // ANCOUNT / PRCOUNT = 0x0102
            0x03, 0x04, // NSCOUNT / UPCOUNT = 0x0304
            0x05, 0x06, // ARCOUNT           = 0x0506
        ];
        let decoded_header = Header::decode(header);
        assert!(decoded_header.is_ok());
        let decoded_header = decoded_header.unwrap();
        let expected_header = Header {
            id:          id::Id(0xABCD),
            qr:          one_bit_flag::Qr::Response,
            opcode:      opcode::Opcode::Query,
            aa:          one_bit_flag::Aa::FromAuthority,
            tc:          one_bit_flag::Tc::Truncated,
            rd:          one_bit_flag::Rd::RecursiveUndesired,
            ra:          one_bit_flag::Ra::RecursionAvailable,
            z:           z::Z,
            ad:          one_bit_flag::Ad::SuccessDnssecValidationOrSupportedAdBit,
            cd:          one_bit_flag::Cd::DnssecEnabled,
            rcode:       rcode::Rcode::NoError,
            qd_zo_count: count::QdZoCount(0x0001),
            an_pr_count: count::AnPrCount(0x0102),
            ns_up_count: count::NsUpCount(0x0304),
            arcount:     count::Arcount(0x0506),
        };
        assert_eq!(decoded_header, expected_header);
    }

    #[test]
    fn it_reports_error_when_header_lack() {
        let buffer: &[u8] = &[];
        assert!(Header::decode(buffer).is_err(), "should not decode empty slice");

        let buffer: &[u8] = &[
            0xAB_u8, 0xCD_u8, // ID = 0xABCD
            0b1_0000_1_1_0_u8, 0b1_0_1_0_0000_u8,
            //^ ^    ^ ^ ^       ^ ^ ^ ^ ^
            //| |    | | |       | | | | `- RCODE  = No error
            //| |    | | |       | | | `--- CD     = DNSSEC enabled
            //| |    | | |       | | `----- AD     = Success DNSSEC validation / Supported AD bit
            //| |    | | |       | `------- Z      = reserved (only 0)
            //| |    | | |        `-------- RA     = Supported recuesive query
            //| |    | | `----------------- RD     = Server recursion undesired
            //| |    | `------------------- TC     = Truncated DNS packet
            //| |    `--------------------- AA     = from Authoritative Server
            //| `-------------------------- OPCODE = Query
            //`---------------------------- QR     = Response
            0x00, 0x01, // QDCOUNT / ZOCOUNT = 0x0001
            0x01, 0x02, // ANCOUNT / PRCOUNT = 0x0102
            0x03, 0x04, // NSCOUNT / UPCOUNT = 0x0304
            0x05,       // ARCOUNT (!!!LACK!!! length == 16bit == 2byte)
        ];
        assert!(Header::decode(buffer).is_err(), "should not decode lack slidce");
    }

    #[test]
    fn it_reports_error_when_found_invalid_value() {
        let buffer: &[u8] = &[
            0xAB, 0xCD, // ID = 0xABCD
            0b1_0111_1_1_0, 0b1_0_1_0_0000,
            //^ ^    ^ ^ ^       ^ ^ ^ ^ ^
            //| |    | | |       | | | | `- RCODE  = No error
            //| |    | | |       | | | `--- CD     = DNSSEC enabled
            //| |    | | |       | | `----- AD     = Success DNSSEC validation / Supported AD bit
            //| |    | | |       | `------- Z      = reserved (only 0)
            //| |    | | |       `--------- RA     = Supported recuesive query
            //| |    | | `----------------- RD     = Server recursion undesired
            //| |    | `------------------- TC     = Truncated DNS packet
            //| |    `--------------------- AA     = from Authoritative Server
            //| `-------------------------- OPCODE = unassigned value (!!!INVALID VALUE!!! `7` is not assigned in RFC)
            //`---------------------------- QR     = Response
            0x00, 0x01, // QDCOUNT / ZOCOUNT = 0x0001
            0x01, 0x02, // ANCOUNT / PRCOUNT = 0x0102
            0x03, 0x04, // NSCOUNT / UPCOUNT = 0x0304
            0x05, 0x06, // ARCOUNT           = 0x0506
        ];
        assert!(Header::decode(buffer).is_err(), "should not decode invalid DNS packet (OPCODE)");

        let buffer: &[u8] = &[
            0xAB, 0xCD, // ID = 0xABCD
            0b1_0100_1_1_0, 0b1_0_1_0_1100,
            //^ ^    ^ ^ ^    ^ ^ ^ ^ ^
            //| |    | | |    | | | | `---- RCODE  = unassigned value (!!!INVALID VALUE!!! `12` is not assigned in RFC)
            //| |    | | |    | | | `------ CD     = DNSSEC enabled
            //| |    | | |    | | `-------- AD     = Success DNSSEC validation / Supported AD bit
            //| |    | | |    | `---------- Z      = reserved (only 0)
            //| |    | | |    `------------ RA     = Supported recuesive query
            //| |    | | `----------------- RD     = Server recursion undesired
            //| |    | `------------------- TC     = Truncated DNS packet
            //| |    `--------------------- AA     = from Authoritative Server
            //| `-------------------------- OPCODE = Notify
            //`---------------------------- QR     = Response
            0x00, 0x01, // QDCOUNT / ZOCOUNT = 0x0001
            0x01, 0x02, // ANCOUNT / PRCOUNT = 0x0102
            0x03, 0x04, // NSCOUNT / UPCOUNT = 0x0304
            0x05, 0x06, // ARCOUNT           = 0x0506
        ];
        assert!(Header::decode(buffer).is_err(), "should not decode invalid DNS packet (RCODE)");
    }

    #[test]
    fn it_encodes_to_u8_array() {
        let header = Header {
            id:          id::Id(0xABCD),
            qr:          one_bit_flag::Qr::Query,
            opcode:      opcode::Opcode::Query,
            aa:          one_bit_flag::Aa::FromAuthority,
            tc:          one_bit_flag::Tc::NotTruncated,
            rd:          one_bit_flag::Rd::RecursiveUndesired,
            ra:          one_bit_flag::Ra::RecursionUnavailable,
            z:           z::Z,
            ad:          one_bit_flag::Ad::SuccessDnssecValidationOrSupportedAdBit,
            cd:          one_bit_flag::Cd::DnssecForbidden,
            rcode:       rcode::Rcode::NoError,
            qd_zo_count: count::QdZoCount(0x0102),
            an_pr_count: count::AnPrCount(0x0304),
            ns_up_count: count::NsUpCount(0x0405),
            arcount:     count::Arcount(0x0607),
        };
        let expected_buffer: Vec<u8> = vec![
            0xAB, 0xCD, // ID
            0b0_0000_1_0_0, 0b0_0_1_1_0000,
            //^ ^    ^ ^ ^    ^ ^ ^ ^ ^
            //| |    | | |    | | | | `---- RCODE
            //| |    | | |    | | | `------ CD
            //| |    | | |    | | `-------- AD
            //| |    | | |    | `---------- Z
            //| |    | | |    `------------ RA
            //| |    | | `----------------- RD
            //| |    | `------------------- TC
            //| |    `--------------------- AA
            //| `-------------------------- OPCODE
            //`---------------------------- QR
            0x01, 0x02, // QDCOUNT / ZOCOUNT
            0x03, 0x04, // ANCOUNT / PRCOUNT
            0x04, 0x05, // NSCOUNT / UPCOUNT
            0x06, 0x07, // ARCOUNT
        ];
        let encoded_buffer = header.encode();
        assert!(encoded_buffer.is_ok());
        let encoded_buffer = encoded_buffer.unwrap();
        assert_eq!(encoded_buffer, expected_buffer);
    }
}
