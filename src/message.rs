
use super::serial_params::Parity;
use super::serial_params::StopBits;


#[derive(PartialEq, Debug)]
enum Message {
    Ping,
    Pong,
    VersionQuery,
    VersionData {
        major: u8,
        minor: u8,
        maintenance: u8,
        build: u8,
    },
    AdcQuery {
        channel: u8,
        length: u8,
        increment_usec: u32,
    },
    AdcData { channel: u8, data: Vec<i16> },
    Status {
        rcv_count: u16,
        snd_count: u16,
        rcv_fails: u16,
    },
    SerialParams {
        channel: u8,
        baud: u32,
        parity: Parity,
        stop: StopBits,
    },
    // SerialData {channel: u8, data: String},
}

pub struct Package(Vec<u8>);

impl Package {
    #[allow(unused)]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[allow(unused)]
    pub fn get_vec(&self) -> &Vec<u8> {
        &self.0
    }

    pub fn pop_byte(&mut self) -> u8 {
        self.0.drain(0..1).next().unwrap()
    }

    pub fn pop_bytes(&mut self, num_bytes: usize) -> Vec<u8> {
        self.0.drain(0..num_bytes).collect()
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) {
        self.0.extend(bytes)
    }
}

pub trait Cereal {
    fn get_id(&self) -> u8;
    fn serialize(&self, out: &mut Package) -> Result<(), String>;
    fn deserialize(&mut self, source: &mut Package) -> Result<(), String>;
}

impl Message {
    #[allow(unused)]
    fn get_typeid(&self) -> u8 {
        return match self {
            Self::Ping => 1,
            Self::Pong => 2,
            Self::VersionQuery => 3,
            Self::VersionData { .. } => 4,
            Self::AdcQuery { .. } => 5,
            Self::AdcData { .. } => 6,
            Self::Status { .. } => 7,
            Self::SerialParams { .. } => 8,
            // Self::SerialData { .. } => 9
        };
    }

    #[allow(unused)]
    fn deserialize(source: &mut Package) -> Option<Self> {

        match source.pop_byte() {
            1 => Some(Self::Ping),
            2 => Some(Self::Pong),
            3 => Some(Self::VersionQuery),
            4 => Some(Self::VersionData {
                major: source.pop_byte(),
                minor: source.pop_byte(),
                maintenance: source.pop_byte(),
                build: source.pop_byte(),
            }),
            5 => {
                Some(Self::AdcQuery {
                    channel: source.pop_byte(),
                    length: source.pop_byte(),
                    increment_usec: u32::from_le_bytes(source.pop_bytes(4).try_into().unwrap()),
                })
            }
            6 => {
                let chan = source.pop_byte();
                let length = u16::from_le_bytes(source.pop_bytes(2).try_into().unwrap());
                Some(Self::AdcData {
                    channel: chan,
                    data: source
                        .pop_bytes(length as usize)
                        .chunks(2)
                        .map(|chunk| {
                            let mut bytes = [0; 2];
                            bytes.copy_from_slice(chunk);
                            i16::from_le_bytes(bytes)
                        })
                        .collect(),
                })
            }
            7 => {
                Some(Self::Status {
                    rcv_count: u16::from_le_bytes(source.pop_bytes(2).try_into().unwrap()),
                    snd_count: u16::from_le_bytes(source.pop_bytes(2).try_into().unwrap()),
                    rcv_fails: u16::from_le_bytes(source.pop_bytes(2).try_into().unwrap()),
                })
            }
            8 => {
                Some(Self::SerialParams {
                    channel: source.pop_byte(),
                    baud: u32::from_le_bytes(source.pop_bytes(4).try_into().unwrap()),
                    parity: Parity::from_byte(&source.pop_byte()).unwrap_or(Parity::None),
                    stop: StopBits::from_byte(&source.pop_byte()).unwrap_or(StopBits::One),
                })
            }
            _ => None,
        }

    }

    #[allow(unused)]
    fn serialize(&self, out: &mut Package) -> Result<(), String> {
        let typeid = self.get_typeid();
        match self {
            Self::Ping => {out.push_bytes(&[typeid]);},
            Self::Pong => {out.push_bytes(&[typeid]);},
            Self::VersionQuery => {out.push_bytes(&[typeid]);},
            Self::VersionData {
                major,
                minor,
                maintenance,
                build,
            } => {
                out.push_bytes(&[typeid, *major, *minor, *maintenance, *build]);
            },
            Self::AdcQuery {
                channel,
                length,
                increment_usec,
            } => {
                out.push_bytes(&[typeid, *channel, *length].to_vec());
                out.push_bytes(&increment_usec.to_le_bytes());
            },
            Self::AdcData { channel, data } => {
                out.push_bytes(&[typeid, *channel]);
                let data: Vec<u8> = data.iter()
                    .flat_map(|&value| value.to_le_bytes().to_vec())
                    .collect();
                let length: u16 = data.len() as u16;
                out.push_bytes(&length.to_le_bytes().to_vec());
                out.push_bytes(&data);
            }
            Self::Status {
                rcv_count,
                snd_count,
                rcv_fails,
            } => {
                out.push_bytes(&[typeid]);
                out.push_bytes(&rcv_count.to_le_bytes());
                out.push_bytes(&snd_count.to_le_bytes());
                out.push_bytes(&rcv_fails.to_le_bytes());
            }
            Self::SerialParams {
                channel,
                baud,
                parity,
                stop,
            } => {
                out.push_bytes(&[typeid, *channel]);
                out.push_bytes(&baud.to_le_bytes());
                out.push_bytes(&[parity.get_byte(), stop.get_byte()]);
            }

            // TODO: kill this panic
            _ => {return Err(String::from("unknown type id encountered"));},
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scribble() {
        let bytes = b"\x08\x09\xA0\xA1 hello".to_vec();
        let le_u32 = u32::from_le_bytes((&bytes[0..4]).try_into().unwrap());

        assert_eq!(0xa1a00908, le_u32);

        let input_data: Vec<i16> = vec![100, 200, 300];

        let result_bytes: Vec<u8> = input_data
            .iter()
            .flat_map(|&value| value.to_le_bytes().to_vec())
            .collect();

        assert_eq!(result_bytes, [100, 0, 200, 0, 44, 1]);
    }

    #[test]
    fn check_ping_pong() {
        let mut serial = Package::new();

        let ping = Message::Ping;

        ping.serialize(& mut serial).unwrap();
        assert_eq!([1], serial.get_vec()[..]);

        let rx_message = Message::deserialize(&mut serial);
        assert_eq!(rx_message.unwrap(), Message::Ping);

        let pong = Message::Pong;

        pong.serialize(&mut serial).unwrap();
        assert_eq!([2], serial.get_vec()[..]);

         let rx_message = Message::deserialize(&mut serial);
        assert_eq!(rx_message.unwrap(), Message::Pong);

    }

    #[test]
    fn check_version() {
        let mut serial = Package::new();

        let query = Message::VersionQuery;

        query.serialize(&mut serial).unwrap();
        assert_eq!([3], serial.get_vec()[..]);

        let rx_message = Message::deserialize(&mut serial);
        assert_eq!(rx_message.unwrap(), query);

        let response = Message::VersionData {
            major: 1,
            minor: 2,
            maintenance: 3,
            build: 4,
        };

        response.serialize(&mut serial).unwrap();
        assert_eq!([4, 1, 2, 3, 4], serial.get_vec()[..]);

        let rx_message = Message::deserialize(&mut serial);
        assert_eq!(rx_message.unwrap(), response);

    }

    #[test]
    fn check_adc() {
        let mut serial = Package::new();

        let query = Message::AdcQuery {
            channel: 1,
            length: 100,
            increment_usec: 5,
        };

        query.serialize(&mut serial).unwrap();
        assert_eq!([5, 1, 100, 5, 0, 0, 0], serial.get_vec()[..]);

        let rx_message = Message::deserialize(&mut serial);
        assert_eq!(rx_message.unwrap(), query);

        let response = Message::AdcData {
            channel: 1,
            data: [1024, 1999, 0, -800, -900].to_vec(),
        };

        response.serialize(&mut serial).unwrap();
        assert_eq!(
            [6, 1, 10, 0, 0, 4, 207, 7, 0, 0, 224, 252, 124, 252],
            serial.get_vec()[..]
        );

        let rx_message = Message::deserialize(&mut serial);
        assert_eq!(rx_message.unwrap(), response);

    }

    #[test]
    fn check_status() {
        let mut serial = Package::new();

        let status = Message::Status {
            rcv_count: 260,
            snd_count: 270,
            rcv_fails: 0,
        };

        status.serialize(&mut serial).unwrap();
        assert_eq!([7, 4, 1, 14, 1, 0, 0], serial.get_vec()[..]);

        let rx_message = Message::deserialize(&mut serial);
        assert_eq!(rx_message.unwrap(), status);
    }

    #[test]
    fn check_parms() {
        let mut serial = Package::new();

        let params = Message::SerialParams {
            channel: 1,
            baud: 19200,
            parity: Parity::Even,
            stop: StopBits::One,
        };

        params.serialize(&mut serial).unwrap();
        assert_eq!([8, 1, 0, 75, 0, 0, 1, 1], serial.get_vec()[..]);

        let rx_message = Message::deserialize(&mut serial);
        assert_eq!(rx_message.unwrap(), params);
    }


    #[test]
    fn check_multiple_deserialize() {
        let mut serial = Package::new();

        let params = Message::SerialParams {
            channel: 1,
            baud: 19200,
            parity: Parity::Even,
            stop: StopBits::One,
        };
        let response = Message::AdcData {
            channel: 1,
            data: [1024, 1999, 0, -800, -900].to_vec(),
        };

        params.serialize(&mut serial).unwrap();
        assert_eq!([8, 1, 0, 75, 0, 0, 1, 1], serial.get_vec()[..]);

        response.serialize(&mut serial).unwrap();
        assert_eq!(
            [8, 1, 0, 75, 0, 0, 1, 1, 6, 1, 10, 0, 0, 4, 207, 7, 0, 0, 224, 252, 124, 252],
            serial.get_vec()[..]
        );

        let rx_message = Message::deserialize(&mut serial);
        assert_eq!(rx_message.unwrap(), params);

        assert_eq!(
            [6, 1, 10, 0, 0, 4, 207, 7, 0, 0, 224, 252, 124, 252],
            serial.get_vec()[..]
        );

        let rx_message = Message::deserialize(&mut serial);
        assert_eq!(rx_message.unwrap(), response);


    }


}
