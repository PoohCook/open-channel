use open_channel::cereal::{Cereal, Package};
use open_channel::serial_params::{Parity, StopBits};

#[derive(PartialEq, Debug, Default)]
pub struct Ping {}

impl Cereal for Ping{
    fn get_id(&self) -> u8 {
        1
    }

    fn pour_in(&mut self, _: &mut Package) -> Result<(), String> {
        self.process()
    }

}

#[derive(PartialEq, Debug, Default)]
pub struct Pong {}

impl Cereal for Pong{
    fn get_id(&self) -> u8 {
        2
    }
    fn pour_in(&mut self, _: &mut Package) -> Result<(), String> {
        self.process()
    }

}

#[derive(PartialEq, Debug, Default)]
pub struct VersionQuery {}

impl Cereal for VersionQuery{
    fn get_id(&self) -> u8 {
        3
    }
    fn pour_in(&mut self, _: &mut Package) -> Result<(), String> {
        self.process()
    }

}


#[derive(PartialEq, Debug, Default)]
pub struct VersionData {
    pub major: u8,
    pub minor: u8,
    pub maintenance: u8,
    pub build: u8,
}

impl Cereal for VersionData{
    fn get_id(&self) -> u8 {
        4
    }

    fn pour_out(&self, package: &mut Package) -> Result<(), String> {
        package.push_bytes(&[self.major, self.minor, self.maintenance, self.build]);
        Ok(())
    }

    fn pour_in(&mut self, package: &mut Package) -> Result<(), String> {
        self.major = package.pop_byte();
        self.minor = package.pop_byte();
        self.maintenance = package.pop_byte();
        self.build = package.pop_byte();

        self.process()
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct AdcQuery {
    pub channel: u8,
    pub length: u8,
    pub increment_usec: u32,
}

impl Cereal for AdcQuery{
    fn get_id(&self) -> u8 {
        5
    }

    fn pour_out(&self, package: &mut Package) -> Result<(), String> {
        package.push_bytes(&[self.channel, self.length].to_vec());
        package.push_bytes(&self.increment_usec.to_le_bytes());
        Ok(())
    }

    fn pour_in(&mut self, package: &mut Package) -> Result<(), String> {
        self.channel = package.pop_byte();
        self.length = package.pop_byte();
        self.increment_usec = u32::from_le_bytes(package.pop_bytes(4).try_into().unwrap());

        self.process()
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct AdcData {
    pub channel: u8,
    pub data: Vec<i16>
}

impl Cereal for AdcData{
    fn get_id(&self) -> u8 {
        6
    }

    fn pour_out(&self, package: &mut Package) -> Result<(), String> {
        package.push_bytes(&[self.channel]);
        let data: Vec<u8> = self.data.iter()
            .flat_map(|&value| value.to_le_bytes().to_vec())
            .collect();
        let length: u16 = data.len() as u16;
        package.push_bytes(&length.to_le_bytes().to_vec());
        package.push_bytes(&data);
        Ok(())
    }

    fn pour_in(&mut self, package: &mut Package) -> Result<(), String> {
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

        self.process()
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct SerialParams {
    pub channel: u8,
    pub baud: u32,
    pub parity: Parity,
    pub stop: StopBits,
}

impl Cereal for SerialParams {
  fn get_id(&self) -> u8 {
    8
  }

  fn pour_out(&self, package: &mut Package) -> Result<(), String> {
    package.push_bytes(&[self.channel]);
    package.push_bytes(&self.baud.to_le_bytes());
    package.push_bytes(&[self.parity.get_byte(), self.stop.get_byte()]);
    Ok(())

  }

  fn pour_in(&mut self, package: &mut Package) -> Result<(), String> {

    self.channel = package.pop_byte();
    self.baud = u32::from_le_bytes(package.pop_bytes(4).try_into().unwrap());
    self.parity = Parity::from_byte(&package.pop_byte()).unwrap_or(Parity::None);
    self.stop = StopBits::from_byte(&package.pop_byte()).unwrap_or(StopBits::One);

    self.process()

  }

}
