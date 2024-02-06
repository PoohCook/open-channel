use std::collections::HashMap;

/// A Hub for serializing and deserializing Cereal Flavors into a Package Stream
///
/// # Examples
///
/// ```
/// use open_channel::cereal::Packager;
///
/// let mut packager = Packager::new();
/// ```
pub struct CerealStream(Vec<u8>);

impl CerealStream {
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

/// A trait representing a Cereal Box.
///
/// This trait defines common methods that all Cereal Boxes should implement,
/// A cereal box is a metaphor for a Message struct that can be poured out
/// into a binary stream and tehn poured back into box from the stream and
/// then consumed
pub trait CerealBox {
    /// Get the type id fo the ceral being processed.
    fn get_id(&self) -> u8;

    /// Pour a cereal stream into a box.
    ///
    /// # Errors
    ///
    /// This function will return an error if the stream lack sufficient
    /// bytes to fill the box.
    fn pour_in(&mut self, _: &mut CerealStream) -> Result<(), String>;

    /// Pour the contents of a cereal box into a cereal stream.
    fn pour_out(&self, _: &mut CerealStream) {
    }

    /// defines how a box of cereal is consumed.
    fn consume(&self){
    }
}

/// A Hub for serializing and deserializing Cereal Flavors into a Package Stream
///
/// # Examples
///
/// ```
/// use open_channel::cereal::Packager;
///
/// let mut packager = Packager::new();
/// ```
pub struct Packager {
    map: HashMap<u8, Box<dyn CerealBox>>,
}

impl Packager {

    /// Creates a new [`Packager`].
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    /// Adds a Ceareal Flavor to the Packager.
    ///
    /// # Panics
    ///
    /// Panics if a cereal flavor has the same id key as a previously added flavor.
    pub fn add_flavor(&mut self, flavor: Box<dyn CerealBox>){
        let id = flavor.get_id();
        if self.map.contains_key(&id) {
            panic!("Error adding flavor ot Packager! Multiple flavors have id: {}", id);
        }
        self.map.insert(id, flavor);
    }

    /// unpack a ceral box into a cereal stream.
    pub fn unpack(&self, msg: &dyn CerealBox, out: &mut CerealStream){
        let id = msg.get_id();
        out.push_bytes(&[id]);
        msg.pour_out(out);
    }

    /// pack a ceral box from the cereal stream.
    ///
    /// # Errors
    ///
    /// This function will return an error if the cereal stream does not
    /// have enough bytes in it.
    pub fn pack(&mut self, source: &mut CerealStream) -> Result<(), String> {
        let id = source.pop_byte();
        let mut result: Result<(), String> = Ok(());
        self.map.entry(id).and_modify(|msg| {
            result = msg.pour_in(source);
        });
        result
    }

}
