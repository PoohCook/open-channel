use open_channel::cereal::{CerealBox, CerealStream};
use open_channel::serial_params::{CharLength, Parity, StopBits};

#[derive(PartialEq, Debug, Default)]
pub struct Ping {}

impl CerealBox for Ping{
    fn get_id(&self) -> u8 {
        1
    }

    fn pour_in(&mut self, _: &mut CerealStream) -> Result<(), String> {
        self.consume();
        Ok(())
    }

}

#[derive(PartialEq, Debug, Default)]
pub struct Pong {}

impl CerealBox for Pong{
    fn get_id(&self) -> u8 {
        2
    }
    fn pour_in(&mut self, _: &mut CerealStream) -> Result<(), String> {
        self.consume();
        Ok(())
    }

}

#[derive(PartialEq, Debug, Default)]
pub struct VersionQuery {}

impl CerealBox for VersionQuery{
    fn get_id(&self) -> u8 {
        3
    }
    fn pour_in(&mut self, _: &mut CerealStream) -> Result<(), String> {
        self.consume();
        Ok(())
    }

}


#[derive(PartialEq, Debug, Default)]
pub struct VersionData {
    pub major: u8,
    pub minor: u8,
    pub maintenance: u8,
    pub build: u8,
}

impl CerealBox for VersionData{
    fn get_id(&self) -> u8 {
        4
    }

    fn pour_out(&self, package: &mut CerealStream) {
        package.push_bytes(&[self.major, self.minor, self.maintenance, self.build]);
    }

    fn pour_in(&mut self, package: &mut CerealStream) -> Result<(), String> {
        self.major = package.pop_byte();
        self.minor = package.pop_byte();
        self.maintenance = package.pop_byte();
        self.build = package.pop_byte();

        self.consume();
        Ok(())
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct AdcQuery {
    pub channel: u8,
    pub length: u8,
    pub increment_usec: u32,
}

impl CerealBox for AdcQuery{
    fn get_id(&self) -> u8 {
        5
    }

    fn pour_out(&self, package: &mut CerealStream) {
        package.push_bytes(&[self.channel, self.length].to_vec());
        package.push_bytes(&self.increment_usec.to_le_bytes());
    }

    fn pour_in(&mut self, package: &mut CerealStream) -> Result<(), String> {
        self.channel = package.pop_byte();
        self.length = package.pop_byte();
        self.increment_usec = u32::from_le_bytes(package.pop_bytes(4).try_into().unwrap());

        self.consume();
        Ok(())
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct AdcData {
    pub channel: u8,
    pub data: Vec<i16>
}

impl CerealBox for AdcData{
    fn get_id(&self) -> u8 {
        6
    }

    fn pour_out(&self, package: &mut CerealStream) {
        package.push_bytes(&[self.channel]);
        let data: Vec<u8> = self.data.iter()
            .flat_map(|&value| value.to_le_bytes().to_vec())
            .collect();
        let length: u16 = data.len() as u16;
        package.push_bytes(&length.to_le_bytes().to_vec());
        package.push_bytes(&data);
    }

    fn pour_in(&mut self, package: &mut CerealStream) -> Result<(), String> {
        self.channel = package.pop_byte();
        let length = u16::from_le_bytes(package.pop_bytes(2).try_into().unwrap());
        self.data = package
            .pop_bytes(length as usize)
            .chunks(2)
            .map(|chunk| {
                let mut bytes = [0; 2];
                bytes.copy_from_slice(chunk);
                i16::from_le_bytes(bytes)
            })
            .collect();

        self.consume();
        Ok(())
    }
}

/// A CerealBox for Serial Port Parameters
///
/// # Examples
/// ```
///
///   use open_channel::boxes::SerialParams
///
///   let pstr = "9600:8E1";
///   let sp = SerialParams::from_str(1, pstr);
///
///   assert_eq!(sp, Ok(SerialParams{
///       channel: 1,
///       baud: 9600,
///       char_len: CharLength::Eight,
///       parity: Parity::Even,
///       stop: StopBits::One
///   }));
///
/// ```
///
#[derive(PartialEq, Debug, Default)]
pub struct SerialParams {
    pub channel: u8,
    pub baud: u32,
    pub char_len: CharLength,
    pub parity: Parity,
    pub stop: StopBits,
}

impl CerealBox for SerialParams {
    fn get_id(&self) -> u8 {
        8
    }

    fn pour_out(&self, package: &mut CerealStream) {
        package.push_bytes(&[self.channel]);
        package.push_bytes(&self.baud.to_le_bytes());
        package.push_bytes(&[self.char_len.get_byte(), self.parity.get_byte(), self.stop.get_byte()]);

    }

    fn pour_in(&mut self, package: &mut CerealStream) -> Result<(), String> {

        self.channel = package.pop_byte();
        self.baud = u32::from_le_bytes(package.pop_bytes(4).try_into().unwrap());
        self.char_len = CharLength::from_byte(&package.pop_byte()).unwrap_or(CharLength::Eight);
        self.parity = Parity::from_byte(&package.pop_byte()).unwrap_or(Parity::None);
        self.stop = StopBits::from_byte(&package.pop_byte()).unwrap_or(StopBits::One);

        self.consume();
        Ok(())

    }
}

impl SerialParams{

    /// create a Serial Parameters from setting string.
    pub fn from_str(channel: u8, settings: &str) -> Result<Self, &str> {
        let parts: Vec<&str> = settings.split(':').collect();
        if (parts.len() != 2) | !([3, 5].contains(&parts[1].len())) {
            return Err("invalid Params format")
        }

        let mut params = SerialParams::default();
        params.channel = channel;

        let b = parts[0].parse();
        match b {
            Ok(b) => {params.baud = b},
            Err(_) => { return Err("invalid baud")}
        }

        let l = parts[1][0..1].to_string();
        match l.as_str() {
            "8" => {params.char_len = CharLength::Eight},
            "7" => {params.char_len = CharLength::Seven},
            _ => {return Err("invalid char length")}
        }

        let p = parts[1][1..2].to_uppercase();
        match p.as_str() {
            "E" => {params.parity = Parity::Even},
            "O" => {params.parity = Parity::Odd},
            "N" => {params.parity = Parity::None},
            _ => {return Err("invalid parity")}
        }

        let s = parts[1][2..].to_uppercase();
        match s.as_str() {
            "1" => {params.stop = StopBits::One},
            "2" => {params.stop = StopBits::Two},
            "1.5" => {params.stop = StopBits::OneAndHalf},
            _ => {return Err("invalid parity")}
        }

        Ok(params)

    }
}


#[test]
fn my_ser(){

    let pstr = "9600:8E1";
    let sp = SerialParams::from_str(1, pstr);

    assert_eq!(sp, Ok(SerialParams{
        channel: 1,
        baud: 9600,
        char_len: CharLength::Eight,
        parity: Parity::Even,
        stop: StopBits::One
    }));

    let pstr = "19200:7o2";
    let sp = SerialParams::from_str(1, pstr);

    assert_eq!(sp, Ok(SerialParams{
        channel: 1,
        baud: 19200,
        char_len: CharLength::Seven,
        parity: Parity::Odd,
        stop: StopBits::Two
    }));

    let pstr = "4800:7n1.5";
    let sp = SerialParams::from_str(1, pstr);

    assert_eq!(sp, Ok(SerialParams{
        channel: 1,
        baud: 4800,
        char_len: CharLength::Seven,
        parity: Parity::None,
        stop: StopBits::OneAndHalf
    }));


}
