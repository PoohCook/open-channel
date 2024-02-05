#[derive(Debug, PartialEq, Clone, Copy, Default)]
#[repr(u8)]
pub enum Parity {
    Even,
    Odd,
    #[default]
    None,
}

impl Parity {
    pub fn from_byte(byte: &u8) -> Option<Self> {
        match byte {
            0 => Some(Self::None),
            1 => Some(Self::Even),
            2 => Some(Self::Odd),
            _ => None,
        }
    }
    pub fn get_byte(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::Even => 1,
            Self::Odd => 2,
        }
    }
}


#[derive(Debug, PartialEq, Clone, Copy, Default)]
#[repr(u8)]
pub enum StopBits {
    #[default]
    One,
    Two,
    OneAndHalf,
}

impl StopBits {
    pub fn from_byte(byte: &u8) -> Option<Self> {
        match byte {
            1 => Some(Self::One),
            2 => Some(Self::Two),
            3 => Some(Self::OneAndHalf),
            _ => None,
        }
    }
    pub fn get_byte(&self) -> u8 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::OneAndHalf => 3,
        }
    }
}
