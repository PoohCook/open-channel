mod boxes;
use boxes::{*};
use open_channel::cereal::{CerealStream, Packager};
use open_channel::serial_params::{Parity, StopBits};

impl Ping{
    fn consume(&self) {
        println!("Ping  Consuming: {:?}", self);
    }
}
impl Pong{
    fn consume(&self) {
        println!("Pong  Consuming: {:?}", self);
    }
}
impl VersionQuery{
    fn consume(&self) {
        println!("VersionQuery  Consuming: {:?}", self);
    }
}
impl VersionData{
    fn consume(&self) {
        println!("VersionData  Consuming: {:?}", self);
    }
}
impl AdcQuery{
    fn consume(&self) {
        println!("AdcQuery  Consuming: {:?}", self);
    }
}
impl AdcData{
    fn consume(&self) {
        println!("AdcData  Consuming: {:?}", self);
    }
}
impl SerialParams{
    fn consume(&self) {
        println!("SerialParams  Consuming: {:?}", self);
    }
}



fn create_packger() -> Packager {
    let mut packager = Packager::new();
    packager.add_flavor(Box::new(Ping::default()));
    packager.add_flavor(Box::new(Pong::default()));
    packager.add_flavor(Box::new(VersionQuery::default()));
    packager.add_flavor(Box::new(VersionData::default()));
    packager.add_flavor(Box::new(AdcQuery::default()));
    packager.add_flavor(Box::new(AdcData::default()));
    packager.add_flavor(Box::new(SerialParams::default()));
    packager
}


fn main() {

    let mut packager = create_packger();

    let mut serial = CerealStream::new();
    packager.unpack(&Ping::default(), &mut serial);
    packager.unpack(&Pong::default(), &mut serial);
    packager.unpack(&VersionQuery::default(), &mut serial);
    packager.unpack(&VersionData{
        major: 1,
        minor: 2,
        maintenance: 3,
        build: 4,
    }, &mut serial);
    packager.unpack(&VersionData{
        major: 4,
        minor: 1,
        maintenance: 9,
        build: 3,
    }, &mut serial);
    packager.unpack(&SerialParams{
        channel: 1,
        baud: 19200,
        parity: Parity::Even,
        stop: StopBits::One
    }, &mut serial);
    packager.unpack(&AdcQuery{
        channel: 3,
        length:4,
        increment_usec: 1500
    }, &mut serial);
    packager.unpack(&AdcData{
        channel: 7,
        data: [1024, 1999, 0, -800, -900].to_vec()
    }, &mut serial);
    packager.unpack(&SerialParams{
        channel: 2,
        baud: 9600,
        parity: Parity::Odd,
        stop: StopBits::Two
    }, &mut serial);
    packager.unpack(&SerialParams{
        channel: 3,
        baud: 4800,
        parity: Parity::None,
        stop: StopBits::One
    }, &mut serial);

    while !serial.is_empty(){
        packager.pack(&mut serial).unwrap();
    }

    // assert_eq!(0, serial.get_vec().len());


}
