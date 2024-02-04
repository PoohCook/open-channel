
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

struct Serialization(Vec<u8>);

impl Serialization {
    #[allow(unused)]
    fn get_vec(&self) -> &Vec<u8> {
        &self.0
    }

    fn pop_byte(&mut self) -> u8 {
        self.0.drain(0..1).next().unwrap()
    }

    fn pop_bytes(&mut self, num_bytes: usize) -> Vec<u8> {
        self.0.drain(0..num_bytes).collect()
    }
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
    fn deserialize(bytes: &mut Serialization) -> Option<Self> {

        match bytes.pop_byte() {
            1 => Some(Self::Ping),
            2 => Some(Self::Pong),
            3 => Some(Self::VersionQuery),
            4 => Some(Self::VersionData {
                major: bytes.pop_byte(),
                minor: bytes.pop_byte(),
                maintenance: bytes.pop_byte(),
                build: bytes.pop_byte(),
            }),
            5 => {
                Some(Self::AdcQuery {
                    channel: bytes.pop_byte(),
                    length: bytes.pop_byte(),
                    increment_usec: u32::from_le_bytes(bytes.pop_bytes(4).try_into().unwrap()),
                })
            }
            6 => {
                let chan = bytes.pop_byte();
                let length = u16::from_le_bytes(bytes.pop_bytes(2).try_into().unwrap());
                Some(Self::AdcData {
                    channel: chan,
                    data: bytes
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
                    rcv_count: u16::from_le_bytes(bytes.pop_bytes(2).try_into().unwrap()),
                    snd_count: u16::from_le_bytes(bytes.pop_bytes(2).try_into().unwrap()),
                    rcv_fails: u16::from_le_bytes(bytes.pop_bytes(2).try_into().unwrap()),
                })
            }
            8 => {
                Some(Self::SerialParams {
                    channel: bytes.pop_byte(),
                    baud: u32::from_le_bytes(bytes.pop_bytes(4).try_into().unwrap()),
                    parity: Parity::from_byte(&bytes.pop_byte()).unwrap_or(Parity::None),
                    stop: StopBits::from_byte(&bytes.pop_byte()).unwrap_or(StopBits::One),
                })
            }
            _ => None,
        }

    }

    #[allow(unused)]
    fn serialize(&self) -> Vec<u8> {
        let typeid = self.get_typeid();
        return match self {
            Self::Ping => [typeid].to_vec(),
            Self::Pong => [typeid].to_vec(),
            Self::VersionQuery => [typeid].to_vec(),
            Self::VersionData {
                major,
                minor,
                maintenance,
                build,
            } => {
                let vec = [typeid, *major, *minor, *maintenance, *build].to_vec();
                vec
            }
            Self::AdcQuery {
                channel,
                length,
                increment_usec,
            } => {
                let mut vec = [typeid, *channel, *length].to_vec();
                vec.extend(increment_usec.to_le_bytes().to_vec());
                vec
            }
            Self::AdcData { channel, data } => {
                let mut vec = [typeid, *channel].to_vec();
                let data: Vec<u8> = data.iter()
                    .flat_map(|&value| value.to_le_bytes().to_vec())
                    .collect();
                let length: u16 = data.len() as u16;
                vec.extend(length.to_le_bytes().to_vec());
                vec.extend(data);
                vec
            }
            Self::Status {
                rcv_count,
                snd_count,
                rcv_fails,
            } => {
                let mut vec = [typeid].to_vec();
                let data = [rcv_count, snd_count, rcv_fails].to_vec();
                vec.extend(data.iter().flat_map(|&value| value.to_le_bytes().to_vec()));
                vec
            }
            Self::SerialParams {
                channel,
                baud,
                parity,
                stop,
            } => {
                let mut vec = [typeid, *channel].to_vec();
                vec.extend(baud.to_le_bytes().to_vec());
                vec.extend([parity.get_byte(), stop.get_byte()].to_vec());
                vec
            }

            // TODO: kill this panic
            _ => panic!(),
        };
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
        let ping = Message::Ping;
        assert_eq!(ping, Message::Ping);

        let tx_bytes = ping.serialize();
        assert_eq!([1], tx_bytes[..]);

        let mut tx_bytes = Serialization(tx_bytes.to_owned());
        let rx_message = Message::deserialize(&mut tx_bytes);
        assert_eq!(rx_message.unwrap(), Message::Ping);

        let pong = Message::Pong;
        assert_eq!(pong, Message::Pong);

        let tx_bytes = pong.serialize();
        assert_eq!([2], tx_bytes[..]);

        let mut tx_bytes = Serialization(tx_bytes.to_owned());
        let rx_message = Message::deserialize(&mut tx_bytes);
        assert_eq!(rx_message.unwrap(), Message::Pong);

    }

    #[test]
    fn check_version() {
        let query = Message::VersionQuery;
        assert_eq!(query, Message::VersionQuery);

        let tx_bytes = query.serialize();
        assert_eq!([3], tx_bytes[..]);

        let mut tx_bytes = Serialization(tx_bytes.to_owned());
        let rx_message = Message::deserialize(&mut tx_bytes);
        assert_eq!(rx_message.unwrap(), query);

        let response = Message::VersionData {
            major: 1,
            minor: 2,
            maintenance: 3,
            build: 4,
        };

        let tx_bytes = response.serialize();
        assert_eq!([4, 1, 2, 3, 4], tx_bytes[..]);

        let mut tx_bytes = Serialization(tx_bytes.to_owned());
        let rx_message = Message::deserialize(&mut tx_bytes);
        assert_eq!(rx_message.unwrap(), response);

    }

    #[test]
    fn check_adc() {
        // Create a UUT AdcQuery
        let query = Message::AdcQuery {
            channel: 1,
            length: 100,
            increment_usec: 5,
        };

        assert_eq!(5, query.get_typeid());

        let tx_bytes = query.serialize();
        assert_eq!([5, 1, 100, 5, 0, 0, 0], tx_bytes[..]);

        let mut tx_bytes = Serialization(tx_bytes.to_owned());
        let rx_message = Message::deserialize(&mut tx_bytes);
        assert_eq!(rx_message.unwrap(), query);

        let response = Message::AdcData {
            channel: 1,
            data: [1024, 1999, 0, -800, -900].to_vec(),
        };

        let tx_bytes = response.serialize();
        assert_eq!(
            [6, 1, 10, 0, 0, 4, 207, 7, 0, 0, 224, 252, 124, 252],
            tx_bytes[..]
        );

        let mut tx_bytes = Serialization(tx_bytes.to_owned());
        let rx_message = Message::deserialize(&mut tx_bytes);
        assert_eq!(rx_message.unwrap(), response);

    }

    #[test]
    fn check_status() {

        let status = Message::Status {
            rcv_count: 260,
            snd_count: 270,
            rcv_fails: 0,
        };

        assert_eq!(7, status.get_typeid());

        let tx_bytes = status.serialize();
        assert_eq!([7, 4, 1, 14, 1, 0, 0], tx_bytes[..]);

        let mut tx_bytes = Serialization(tx_bytes.to_owned());
        let rx_message = Message::deserialize(&mut tx_bytes);
        assert_eq!(rx_message.unwrap(), status);
    }

    #[test]
    fn check_parms() {
        let params = Message::SerialParams {
            channel: 1,
            baud: 19200,
            parity: Parity::Even,
            stop: StopBits::One,
        };

        assert_eq!(8, params.get_typeid());

        let tx_bytes = params.serialize();
        assert_eq!([8, 1, 0, 75, 0, 0, 1, 1], tx_bytes[..]);

        let mut tx_bytes = Serialization(tx_bytes.to_owned());
        let rx_message = Message::deserialize(&mut tx_bytes);
        assert_eq!(rx_message.unwrap(), params);
    }


    #[test]
    fn check_multiple_deserialize() {
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

        let mut all_bytes: Vec<u8> = Vec::new();

        let tx_bytes = params.serialize();
        assert_eq!([8, 1, 0, 75, 0, 0, 1, 1], tx_bytes[..]);
        all_bytes.extend(tx_bytes);

        let tx_bytes = response.serialize();
        assert_eq!(
            [6, 1, 10, 0, 0, 4, 207, 7, 0, 0, 224, 252, 124, 252],
            tx_bytes[..]
        );
        all_bytes.extend(tx_bytes);
        assert_eq!(
            [8, 1, 0, 75, 0, 0, 1, 1, 6, 1, 10, 0, 0, 4, 207, 7, 0, 0, 224, 252, 124, 252],
            all_bytes[..]
        );

        let mut all_bytes = Serialization(all_bytes.to_owned());

        let rx_message = Message::deserialize(&mut all_bytes);
        assert_eq!(rx_message.unwrap(), params);

        assert_eq!(
            [6, 1, 10, 0, 0, 4, 207, 7, 0, 0, 224, 252, 124, 252],
            all_bytes.get_vec()[..]
        );

        let rx_message = Message::deserialize(&mut all_bytes);
        assert_eq!(rx_message.unwrap(), response);


    }


}
