use int_enum::IntEnum;
use enum_iterator::IntoEnumIterator;

#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone, IntEnum, IntoEnumIterator)]
pub enum ResistorColor {
    Black   = 0,
    Brown   = 1,
    Red     = 2,
    Orange  = 3,
    Yellow  = 4,
    Green   = 5,
    Blue    = 6,
    Violet  = 7,
    Grey    = 8,
    White   = 9
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    match ResistorColor::into_enum_iter().find(|&x| x == _color ){
        Some(color) => color.int_value() as usize,
        None => usize::MAX
    }
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value.try_into().unwrap()) {
        Ok(_color) => {
            match _color {
                ResistorColor::Black    => String::from("Black"),
                ResistorColor::Brown    => String::from("Brown"),
                ResistorColor::Red      => String::from("Red"),
                ResistorColor::Orange   => String::from("Orange"),
                ResistorColor::Yellow   => String::from("Yellow"),
                ResistorColor::Green    => String::from("Green"),
                ResistorColor::Blue     => String::from("Blue"),
                ResistorColor::Violet   => String::from("Violet"),
                ResistorColor::Grey     => String::from("Grey"),
                ResistorColor::White    => String::from("White"),
            }
        },
        Err(_) => String::from("value out of range")
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut ordered: Vec<ResistorColor> = Vec::new();
    let colors = ResistorColor::into_enum_iter();
    
    // I mean, not that efficient, but I guess is good?
    for value in 0..=9 {
        let c: ResistorColor = colors.clone().find(|x| x.int_value() == value).unwrap();
        ordered.push(c);
    }
    ordered
}
