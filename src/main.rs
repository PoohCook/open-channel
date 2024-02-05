mod flavors;
use flavors::{*};
use open_channel::cereal::{Package, Packager};
use open_channel::serial_params::{Parity, StopBits};

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

    let mut serial = Package::new();
    packager.serialize(&Ping::default(), &mut serial).unwrap();
    packager.serialize(&Pong::default(), &mut serial).unwrap();
    packager.serialize(&VersionQuery::default(), &mut serial).unwrap();
    packager.serialize(&VersionData{
        major: 1,
        minor: 2,
        maintenance: 3,
        build: 4,
    }, &mut serial).unwrap();
    packager.serialize(&VersionData{
        major: 4,
        minor: 1,
        maintenance: 9,
        build: 3,
    }, &mut serial).unwrap();
    packager.serialize(&SerialParams{
        channel: 1,
        baud: 19200,
        parity: Parity::Even,
        stop: StopBits::One
    }, &mut serial).unwrap();
    packager.serialize(&AdcQuery{
        channel: 3,
        length:4,
        increment_usec: 1500
    }, &mut serial).unwrap();
    packager.serialize(&AdcData{
        channel: 7,
        data: [1024, 1999, 0, -800, -900].to_vec()
    }, &mut serial).unwrap();
    packager.serialize(&SerialParams{
        channel: 2,
        baud: 9600,
        parity: Parity::Odd,
        stop: StopBits::Two
    }, &mut serial).unwrap();
    packager.serialize(&SerialParams{
        channel: 3,
        baud: 4800,
        parity: Parity::None,
        stop: StopBits::One
    }, &mut serial).unwrap();

    while !serial.is_empty(){
        packager.deserialize(&mut serial).unwrap();
    }

    // assert_eq!(0, serial.get_vec().len());


}
