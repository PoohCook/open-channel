use std::collections::HashMap;
use open_channel::message::Package;
use open_channel::message::Cereal;


#[derive(PartialEq, Debug, Default)]
struct VersionData {
    major: u8,
    minor: u8,
    maintenance: u8,
    build: u8,
}

impl Cereal for VersionData{
    fn get_id(&self) -> u8 {
        4
    }

    fn serialize(&self, out: &mut Package) -> Result<(), String> {
        out.push_bytes(&[self.major, self.minor, self.maintenance, self.build]);
        Ok(())
    }

    fn deserialize(&mut self, source: &mut Package) -> Result<(), String> {
        self.major = source.pop_byte();
        self.minor = source.pop_byte();
        self.maintenance = source.pop_byte();
        self.build = source.pop_byte();

        println!("we are: {:?}", self);
        Ok(())
    }
}

struct Packager {
    map: HashMap<u8, Box<dyn Cereal>>,
}

impl Packager {

    fn new() -> Self {

        let mut msg_map: HashMap<u8, Box<dyn Cereal>> = HashMap::new();

        msg_map.insert(4, Box::new(VersionData::default()));

        Self {
            map: msg_map
        }

    }

    fn serialize(&self, msg: &dyn Cereal, out: &mut Package) -> Result<(), String>{
        let id = msg.get_id();
        out.push_bytes(&[id]);
        msg.serialize(out)
    }

    fn deserialize(&mut self, source: &mut Package) -> Result<(), String> {
        let id = source.pop_byte();
        self.map.entry(id).and_modify(|msg| {
            msg.deserialize(source).unwrap();
        });
        Ok(())
    }

}


fn main() {
    // let json_data = r#"{ "name": "Alice", "age": 30, "sex": "female" }"#;
    // let person: Person = serde_json::from_str(json_data).unwrap();
    // println!("{:?}", person);

    let mut map = Packager::new();

    let data = VersionData{
        major: 1,
        minor: 2,
        maintenance: 3,
        build: 4,
    };

    let mut serial = Package::new();
    map.serialize(&data, &mut serial).unwrap();

    assert_eq!([4, 1, 2, 3, 4], serial.get_vec()[..]);

    map.deserialize(&mut serial).unwrap();
    assert_eq!(0, serial.get_vec().len());


}
