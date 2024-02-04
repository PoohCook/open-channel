use std::io::Read;

use super::serial_params::Parity;
use super::serial_params::StopBits;

extern crate bytes;
use bytes::Bytes;

#[derive(PartialEq, Debug)]
pub enum Message {
    Ping,
    Pong,
    VersionQuery,
    VersionData { major: u8, minor: u8, maintenance: u8, build: u8},
    AdcQuery {channel: u8, length: u8, increment_usec: u32},
    AdcData {channel: u8, data: Vec<i16>},
    Status { rcv_count: u16, snd_count: u16, rcv_fails: u16},
    SerialParams { channel: u8, baud: u32, parity: Parity, stop: StopBits},
    SerialData {channel: u8, data: String},
}


impl Message{
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
            Self::SerialData { .. } => 9
        }
    }

    fn get_bytes(&self) -> Bytes {
        let typeid = self.get_typeid();
        return match self {
            Self::Ping => Bytes::from([typeid].to_vec()),
            Self::Pong => Bytes::from([typeid].to_vec()),
            Self::VersionQuery => Bytes::from([typeid].to_vec()),
            Self::VersionData { major, minor, maintenance, build } => {
                let vec = [typeid, *major, *minor, *maintenance, *build].to_vec();
                Bytes::from(vec)
            },
            Self::AdcQuery { channel, length, increment_usec } => {
                let mut vec = [typeid, *channel, *length].to_vec();
                vec.extend(increment_usec.to_le_bytes().to_vec());
                Bytes::from(vec)
            },
            Self::AdcData { channel, data } => {
                let mut vec = [typeid, *channel].to_vec();
                vec.extend(data.iter().flat_map(|&value| value.to_le_bytes().to_vec()));
                Bytes::from(vec)
            },
            Self::Status { rcv_count, snd_count, rcv_fails } => {
                let mut vec = [typeid,].to_vec();
                let data = [rcv_count, snd_count, rcv_fails].to_vec();
                vec.extend(data.iter().flat_map(|&value| value.to_le_bytes().to_vec()));
                Bytes::from(vec)
            },
            Self::SerialParams { channel, baud, parity, stop } => {
                let mut vec = [typeid, *channel].to_vec();
                vec.extend(baud.to_le_bytes().to_vec());
                vec.extend([parity.get_byte(), stop.get_byte()].to_vec());
                Bytes::from(vec)
            },

            // TODO: kill this panic
            _ => panic!()
        }
    }

    fn from_bytes(bytes: &Bytes) -> Option<Self> {

        match bytes[0] {
            1 => Some(Self::Ping),
            2 => Some(Self::Pong),
            3 => Some(Self::VersionQuery),
            4 => Some(Self::VersionData {
                    major: bytes[1],
                    minor: bytes[2],
                    maintenance: bytes[3],
                    build: bytes[4]
                }),
            5 => { Some(Self::AdcQuery {
                    channel: bytes[1],
                    length: bytes[2] ,
                    increment_usec: u32::from_le_bytes((&bytes[3..7]).try_into().unwrap())
                })},
            6 => { Some(Self::AdcData {
                    channel: bytes[1],
                    data: bytes[2..]
                        .chunks(2)
                        .map(|chunk| {
                            let mut bytes = [0; 2];
                            bytes.copy_from_slice(chunk);
                            i16::from_le_bytes(bytes)
                        })
                    .collect()
                })},
            7 => { Some(Self::Status {
                    rcv_count: u16::from_le_bytes((&bytes[1..3]).try_into().unwrap()),
                    snd_count: u16::from_le_bytes((&bytes[3..5]).try_into().unwrap()),
                    rcv_fails: u16::from_le_bytes((&bytes[5..7]).try_into().unwrap())
                })},
            8 => { Some(Self::SerialParams {
                    channel: bytes[1],
                    baud: u32::from_le_bytes((&bytes[2..6]).try_into().unwrap()),
                    parity: Parity::from_byte(&bytes[6]).unwrap_or(Parity::None),
                    stop: StopBits::from_byte(&bytes[6]).unwrap_or(StopBits::One) })}
            _ => None

        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scribble() {
        let bytes = Bytes::from_static(b"\x08\x09\xA0\xA1 hello");
        let le_u32 = u32::from_le_bytes((&bytes[0..4]).try_into().unwrap());

        assert_eq!(0xa1a00908, le_u32);

        let input_data: Vec<i16> = vec![100, 200, 300];

        let result_bytes: Vec<u8> = input_data.iter().flat_map(|&value| value.to_le_bytes().to_vec()).collect();

        assert_eq!(result_bytes, [100, 0, 200, 0, 44, 1]);
    }

    #[test]
    fn check_ping_pong() {
        let ping = Message::Ping;
        assert_eq!(ping, Message::Ping);

        let tx_bytes = ping.get_bytes();
        assert_eq!([1], tx_bytes[..]);

        let rx_message = Message::from_bytes(&tx_bytes);
        assert_eq!(rx_message.unwrap(), Message::Ping);

        let pong = Message::Pong;
        assert_eq!(pong, Message::Pong);

        let tx_bytes = pong.get_bytes();
        assert_eq!([2], tx_bytes[..]);

        let rx_message = Message::from_bytes(&tx_bytes);
        assert_eq!(rx_message.unwrap(), Message::Pong);

    }

    #[test]
    fn check_version() {
        let query = Message::VersionQuery;
        assert_eq!(query, Message::VersionQuery);

        let tx_bytes = query.get_bytes();
        assert_eq!([3], tx_bytes[..]);

        let rx_message = Message::from_bytes(&tx_bytes);
        assert_eq!(rx_message.unwrap(), query);

        let response = Message::VersionData { major: 1, minor: 2, maintenance: 3, build: 4 };
        assert_eq!(response, Message::VersionData { major: 1, minor: 2, maintenance: 3, build: 4 });

        let tx_bytes = response.get_bytes();
        assert_eq!([4, 1, 2, 3, 4], tx_bytes[..]);

        let rx_message = Message::from_bytes(&tx_bytes);
        assert_eq!(rx_message.unwrap(), response);

    }

    #[test]
    fn check_adc() {
        // Create a UUT AdcQuery
        let query = Message::AdcQuery {channel: 1, length: 100, increment_usec: 5};
        assert_eq!(query, Message::AdcQuery {channel: 1, length: 100, increment_usec: 5});

        assert_eq!(5, query.get_typeid());

        let tx_bytes = query.get_bytes();
        assert_eq!([5, 1, 100, 5, 0, 0, 0], tx_bytes[..]);

        let rx_message = Message::from_bytes(&tx_bytes);
        assert_eq!(rx_message.unwrap(), query);

        let response = Message::AdcData { channel: 1, data: [1024, 1999, 0, -800, -900].to_vec() };

        let tx_bytes = response.get_bytes();
        assert_eq!([6, 1, 0, 4, 207, 7, 0, 0, 224, 252, 124, 252], tx_bytes[..]);

        let rx_message = Message::from_bytes(&tx_bytes);
        assert_eq!(rx_message.unwrap(), response);

    }

    #[test]
    fn check_status() {

        let status = Message::Status { rcv_count: 260, snd_count: 270, rcv_fails: 0 };
        assert_eq!(status, Message::Status { rcv_count: 260, snd_count: 270, rcv_fails: 0 });

        assert_eq!(7, status.get_typeid());

        let tx_bytes = status.get_bytes();
        assert_eq!([7, 4, 1, 14, 1, 0, 0], tx_bytes[..]);

        let rx_message = Message::from_bytes(&tx_bytes);
        assert_eq!(rx_message.unwrap(), status);
    }

    #[test]
    fn check_parms() {
        let params = Message::SerialParams { channel: 1, baud: 19200, parity: Parity::Even, stop: StopBits::One };
        assert_eq!(params, Message::SerialParams { channel: 1, baud: 19200, parity: Parity::Even, stop: StopBits::One });

        assert_eq!(8, params.get_typeid());

        let tx_bytes = params.get_bytes();
        assert_eq!([8, 1, 0, 75, 0, 0, 1, 1], tx_bytes[..]);

        let rx_message = Message::from_bytes(&tx_bytes);
        assert_eq!(rx_message.unwrap(), params);
    }



}
