#[macro_export]
macro_rules! bit_field {
    (get: $get:ident @ $bit:expr) => {
        #[inline]
        pub fn $get(&self) -> bool {
            (self.0 >> $bit) & 1 != 0
        }
    };

    (set: $set:ident @ $bit:expr) => {
        #[inline]
        pub fn $set(&mut self, value: bool) {
            if value {
                self.0 |= 1 << $bit;
            } else {
                self.0 &= !(1 << $bit);
            }
        }
    };

    ($get:ident, $set:ident @ $bit:expr) => {
        bit_field!(get: $get @ $bit);
        bit_field!(set: $set @ $bit);
    };
}

#[macro_export]
macro_rules! bit_range {
    (get: $get:ident @ $start:expr, $end:expr => $ty:ty) => {
        #[inline]
        pub fn $get(&self) -> $ty {
            ((self.0 >> $start) & ((1 << $end - $start) - 1)) as $ty
        }
    };

    (set: $set:ident @ $start:expr, $size:expr => $ty:ty) => {
        #[inline]
        pub fn $set(&mut self, value: $ty) {
            const MASK: $ty = ((1 << $size) - 1) << $start;
            self.0 = (self.0 & !MASK) | (((value as $ty) << $start) & MASK);
        }
    };

    ($get:ident, $set:ident @ $start:expr, $size:expr => $ty:ty) => {
        bit_range!(get: $get @ $start, $size => $ty);
        bit_range!(set: $set @ $start, $size => $ty);
    };
}

pub fn flip_byte(value: u8) -> u8 {
    let mut result = 0;
    for i in 0u8..8 {
        result |= ((value >> i) & 1) << (7 - i);
    }
    result
}
