use is_even_or_odd::IsEvenOrOdd;

pub trait IsntEvenNorOdd {
    fn isnt_even_nor_odd(&self) -> bool;
}

macro_rules! prim_impl {
    ($($t:tt)*) => {
        $(
            impl IsntEvenNorOdd for $t {
                fn isnt_even_nor_odd(&self) -> bool {
                    !self.is_even_or_odd()
                }
            }
        )*
    };
}

prim_impl!(i8 u8 i16 u16 i32 u32 i64 u64);
