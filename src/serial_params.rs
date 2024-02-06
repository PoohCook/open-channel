/// A representation of a Serial Port character length
/// (independent of parity bits)  setting
///
/// can be converted back and forth with a u8 value
///
/// # Examples
///
/// ```
///   use open_channel::serial_params::CharLength;
///
///   let cl = CharLength::Eight;
///   let v = cl.get_byte();
///   let v2 = CharLength::from_byte(&v);
///   assert_eq!(Some(cl), v2);
///
///   let cl = CharLength::Seven;
///   let v = cl.get_byte();
///   let v2 = CharLength::from_byte(&v);
///   assert_eq!(Some(cl), v2);

/// ````
///
#[derive(Debug, PartialEq, Clone, Copy, Default)]
#[repr(u8)]
pub enum CharLength {
    #[default]
    Eight,
    Seven,
}

impl CharLength {
    /// turn a u8 value to Some(CharLength).
    /// returns None if there is no mapping
    pub fn from_byte(byte: &u8) -> Option<Self> {
        match byte {
            1 => Some(Self::Seven),
            2 => Some(Self::Eight),
            _ => None,
        }
    }
    /// Returns the u8 representation of this [`CharLength`].
    pub fn get_byte(&self) -> u8 {
        match self {
            Self::Seven => 1,
            Self::Eight => 2,
        }
    }
}


/// A representation of a Serial Port Parity setting
///
/// can be converted back and forth with a u8 value
///
/// # Examples
///
/// ```
///   use open_channel::serial_params::Parity;
///   let p = Parity::None;
///   let v = p.get_byte();
///   let v2 = Parity::from_byte(&v);
///   assert_eq!(Some(p), v2);
///
///   let p = Parity::Odd;
///   let v = p.get_byte();
///   let v2 = Parity::from_byte(&v);
///   assert_eq!(Some(p), v2);
///
///   let p = Parity::Even;
///   let v = p.get_byte();
///   let v2 = Parity::from_byte(&v);
///   assert_eq!(Some(p), v2);
///
///   assert_eq!(None, Parity::from_byte(&33));
///
/// ```
///
#[derive(Debug, PartialEq, Clone, Copy, Default)]
#[repr(u8)]
pub enum Parity {
    Even,
    Odd,
    #[default]
    None,
}

impl Parity {
    /// turn a u8 value to Some(Parity).
    /// returns None if there is no mapping
    pub fn from_byte(byte: &u8) -> Option<Self> {
        match byte {
            0 => Some(Self::None),
            1 => Some(Self::Even),
            2 => Some(Self::Odd),
            _ => None,
        }
    }
    /// Returns the u8 representation of this [`Parity`].
    pub fn get_byte(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::Even => 1,
            Self::Odd => 2,
        }
    }
}


/// A representation of a Serial Port Stop Bits setting
///
/// can be converted back and forth with a u8 value
///
/// # Examples
///
/// ```
///
///   use open_channel::serial_params::StopBits;
///
///   let sb = StopBits::One;
///   let v = sb.get_byte();
///   let v2 = StopBits::from_byte(&v);
///   assert_eq!(Some(sb), v2);
///
///   let sb = StopBits::Two;
///   let v = sb.get_byte();
///   let v2 = StopBits::from_byte(&v);
///   assert_eq!(Some(sb), v2);
///
///   let sb = StopBits::OneAndHalf;
///   let v = sb.get_byte();
///   let v2 = StopBits::from_byte(&v);
///   assert_eq!(Some(sb), v2);
///
///   assert_eq!(None, StopBits::from_byte(&33));
///   
/// ```
///
#[derive(Debug, PartialEq, Clone, Copy, Default)]
#[repr(u8)]
pub enum StopBits {
    #[default]
    One,
    Two,
    OneAndHalf,
}

impl StopBits {
    /// turn a u8 value to Some(StopBits).
    /// returns None if there is no mapping
    pub fn from_byte(byte: &u8) -> Option<Self> {
        match byte {
            1 => Some(Self::One),
            2 => Some(Self::Two),
            3 => Some(Self::OneAndHalf),
            _ => None,
        }
    }
    /// Returns the u8 representation of this [`StopBits`].
    pub fn get_byte(&self) -> u8 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::OneAndHalf => 3,
        }
    }
}
