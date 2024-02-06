mod boxes;
use boxes::{*};
use open_channel::cereal::Packager;
use open_channel::serial_params::{CharLength, Parity, StopBits};

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

    packager.unpack(&Ping::default());
    packager.unpack(&Pong::default());
    packager.unpack(&VersionQuery::default());
    packager.unpack(&VersionData{
        major: 1,
        minor: 2,
        maintenance: 3,
        build: 4,
    });
    packager.unpack(&VersionData{
        major: 4,
        minor: 1,
        maintenance: 9,
        build: 3,
    });
    packager.unpack(&SerialParams{
        channel: 1,
        baud: 19200,
        char_len: CharLength::Eight,
        parity: Parity::Even,
        stop: StopBits::One
    });
    packager.unpack(&AdcQuery{
        channel: 3,
        length:4,
        increment_usec: 1500
    });
    packager.unpack(&AdcData{
        channel: 7,
        data: [1024, 1999, 0, -800, -900].to_vec()
    });
    packager.unpack(&SerialParams{
        channel: 2,
        baud: 9600,
        char_len: CharLength::Eight,
        parity: Parity::Odd,
        stop: StopBits::Two
    });
    packager.unpack(&SerialParams{
        channel: 3,
        baud: 4800,
        char_len: CharLength::Seven,
        parity: Parity::None,
        stop: StopBits::One
    });

    while !packager.is_empty(){
        packager.pack().unwrap();
    }

}
