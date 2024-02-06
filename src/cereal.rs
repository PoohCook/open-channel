use std::collections::HashMap;

/// A Hub for packing and unpacking Cereal Boxes into a Cereal Stream
///
/// # Examples
///
/// ```
///   use open_channel::cereal::CerealStream;
///
///   let mut stream = CerealStream::new();
///   assert_eq!(stream.is_empty(), true);
///
///   stream.push_bytes(&[1,2,3]);
///   assert_eq!(stream.is_empty(), false);
///
///   let out = stream.pop_bytes(2);
///   assert_eq!(out, [1,2].to_vec());
///   assert_eq!(stream.is_empty(), false);
///
///   let out = stream.pop_byte();
///   assert_eq!(out, 3);
///   assert_eq!(stream.is_empty(), true);
/// ```
pub struct CerealStream(Vec<u8>);

impl CerealStream {
    /// Creates a new [`CerealStream`].
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Returns a reference to the get vec of this [`CerealStream`].
    pub fn get_vec(&self) -> &Vec<u8> {
        &self.0
    }

    /// Returns true if the stream is empty [`CerealStream`].
    pub fn is_empty(&self) -> bool {
        self.get_vec().len() == 0
    }

    /// Returns the a singel byte from the stream.
    ///
    /// # Panics
    ///
    /// Panics if the stream is empty.
    pub fn pop_byte(&mut self) -> u8 {
        self.0.drain(0..1).next().unwrap()
    }

    /// Returns the specified number of bytes from the stream.
    ///
    /// # Panics
    ///
    /// Panics if the stream does not conatin enough bytes.
    pub fn pop_bytes(&mut self, num_bytes: usize) -> Vec<u8> {
        self.0.drain(0..num_bytes).collect()
    }

    /// pushes the specified bytes into the stream.
    pub fn push_bytes(&mut self, bytes: &[u8]) {
        self.0.extend(bytes)
    }
}

impl Default for CerealStream {
    fn default() -> Self {
        Self::new()
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
    stream: CerealStream,

}

impl Packager {

    /// Creates a new [`Packager`].
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            stream: CerealStream::new()
        }
    }

    /// Returns the true if the contained stream is empty.
    pub fn is_empty(&self) -> bool {
        self.stream.is_empty()
    }

    /// Adds a Cereal Box to the Packager.
    ///
    /// # Panics
    ///
    /// Panics if a cereal cereal_box has the same id key as a previously added cereal_box.
    pub fn add_flavor(&mut self, cereal_box: Box<dyn CerealBox>){
        let id = cereal_box.get_id();
        if self.map.contains_key(&id) {
            panic!("Error adding cereal_box ot Packager! Multiple flavors have id: {}", id);
        }
        self.map.insert(id, cereal_box);
    }

    /// unpack a ceral box into a cereal stream.
    pub fn unpack(&mut self, msg: &dyn CerealBox){
        let id = msg.get_id();
        self.stream.push_bytes(&[id]);
        msg.pour_out(&mut self.stream);
    }

    /// pack a ceral box from the cereal stream.
    ///
    /// # Errors
    ///
    /// This function will return an error if the cereal stream does not
    /// have enough bytes in it.
    pub fn pack(&mut self) -> Result<(), String> {
        let id = self.stream.pop_byte();
        let mut result: Result<(), String> = Ok(());
        self.map.entry(id).and_modify(|msg| {
            result = msg.pour_in(&mut self.stream);
        });
        result
    }

}
