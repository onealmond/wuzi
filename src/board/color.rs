#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Color {
    pub value: u8,
}

//impl Hash for Color {
//    fn hash<H: Hasher>(&self, state: &mut H) {
//        self.value.hash(state);
//    }
//}

//impl PartialEq for Color {
//    fn eq(&self, other: &Self) -> bool {
//        return self.value == other.value;
//    }
//}
//
//impl Clone for Color {
//    fn clone(&self) -> Self {
//        *self;
//    }
//}
//impl Copy for Color {}

//impl fmt::Display for Color {
//    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//        return formatter.write_str(self.value);
//    }
//}
//

pub const COLOR_GREEN: Color = Color{value: 0};
pub const COLOR_RED: Color = Color{value: 1};

impl Color {
    pub fn from_u8(color: u8) -> Option<Color> {
        match color {
            0 => return Some(COLOR_GREEN),
            1 => return Some(COLOR_RED),
            _ => None, 
        }
    }

    pub fn to_string(&self) -> String {
        match self.value {
            0 => return "G".to_string(),
            1 => return "R".to_string(),
            _ => "".to_string(),
        }
    }

    pub fn name(&self) -> String {
        match self.value {
            0 => return "Green".to_string(),
            1 => return "Red".to_string(),
            _ => "".to_string(),
        }
    }
}

