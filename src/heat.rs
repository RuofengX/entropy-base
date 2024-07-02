use ordered_float::NotNan;

pub type Temperature = i8;
pub type Standard = i8;
pub type Absolute = u8;
pub type Efficiency = f32;

pub fn carnot_efficiency(one: Standard, other: Standard) -> Efficiency {
    let one = to_absolute(one);
    let other = to_absolute(other);
    let one = unsafe { NotNan::new_unchecked(one as f32) };
    let other = unsafe { NotNan::new_unchecked(other as f32) };
    let (h, c) = if one > other {
        (*one, *other)
    } else {
        (*other, *one)
    };
    1f32 - c / h
}

pub fn to_absolute(value: Standard) -> Absolute {
    if value < 0 {
        (value - i8::MIN) as u8
    } else {
        value as u8 + 128u8
    }
}
