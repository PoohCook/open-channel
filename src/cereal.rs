use std::collections::HashMap;

pub struct Package(Vec<u8>);

impl Package {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn get_vec(&self) -> &Vec<u8> {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.get_vec().len() == 0
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
    fn pour_out(&self, _: &mut Package) -> Result<(), String> {
        Ok(())
    }

    fn pour_in(&mut self, _: &mut Package) -> Result<(), String> {
        self.process()
    }

    fn process(&self) -> Result<(), String>;
}

pub struct Packager {
    map: HashMap<u8, Box<dyn Cereal>>,
}

impl Packager {

    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    pub fn add_flavor(&mut self, flavor: Box<dyn Cereal>){
        let id = flavor.get_id();
        if self.map.contains_key(&id) {
            panic!("Error adding flavor ot Packager! Multiple flavors have id: {}", id);
        }
        self.map.insert(id, flavor);
    }

    pub fn serialize(&self, msg: &dyn Cereal, out: &mut Package) -> Result<(), String>{
        let id = msg.get_id();
        out.push_bytes(&[id]);
        msg.pour_out(out)
    }

    pub fn deserialize(&mut self, source: &mut Package) -> Result<(), String> {
        let id = source.pop_byte();
        self.map.entry(id).and_modify(|msg| {
            msg.pour_in(source).unwrap();
        });
        Ok(())
    }

}
