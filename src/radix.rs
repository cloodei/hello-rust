use std::mem;


pub trait RadixKey: Copy + Sized + Ord {
    type Unsigned: Copy + Sized + Ord + std::ops::Shr<usize, Output = Self::Unsigned> + std::ops::BitAnd<Self::Unsigned, Output = Self::Unsigned> + From<u8>;

    const BYTES: usize = mem::size_of::<Self>();
    const IS_SIGNED: bool;

    fn to_unsigned(self) -> Self::Unsigned;
    fn from_unsigned(unsigned: Self::Unsigned) -> Self;
}

macro_rules! impl_radix_key_signed {
    ($T:ty, $U:ty) => {
        impl RadixKey for $T {
            type Unsigned = $U;
            const IS_SIGNED: bool = true;

            #[inline]
            fn to_unsigned(self) -> Self::Unsigned {
                self.to_le_bytes();
                unsafe { mem::transmute::<$T, Self::Unsigned>(self) }
            }

            #[inline]
            fn from_unsigned(unsigned: Self::Unsigned) -> Self {
                 unsigned.to_le_bytes();
                 unsafe { mem::transmute::<Self::Unsigned, $T>(unsigned) }
            }
        }
    };
}

macro_rules! impl_radix_key_unsigned {
    ($T:ty) => {
        impl RadixKey for $T {
            type Unsigned = $T;
            const IS_SIGNED: bool = false;

            #[inline]
            fn to_unsigned(self) -> Self::Unsigned {
                self
            }
             #[inline]
            fn from_unsigned(unsigned: Self::Unsigned) -> Self {
                unsigned
            }
        }
    };
}

impl_radix_key_signed!(i8, u8);
impl_radix_key_signed!(i16, u16);
impl_radix_key_signed!(i32, u32);
impl_radix_key_signed!(i64, u64);
impl_radix_key_signed!(i128, u128);
impl_radix_key_signed!(isize, usize);

impl_radix_key_unsigned!(u8);
impl_radix_key_unsigned!(u16);
impl_radix_key_unsigned!(u32);
impl_radix_key_unsigned!(u64);
impl_radix_key_unsigned!(u128);
impl_radix_key_unsigned!(usize);


/// Sorts a slice of elements implementing the `RadixKey` trait using Radix Sort (LSD).
///
/// This implementation handles both signed and unsigned integer types correctly.
///
/// # Arguments
///
/// * `arr`: The mutable slice to be sorted in place.
///
/// # Type Parameters
///
/// * `T`: The type of elements in the slice. Must implement `RadixKey`.
pub fn radix_sort<T: RadixKey>(arr: &mut [T]) {
    let n = arr.len();
    if n < 2 {
        return;
    }

    let mut buffer: Vec<T> = vec![T::from_unsigned(0u8.into()); n];

    let mut src: &mut [T] = arr;
    let mut dst: &mut [T] = buffer.as_mut_slice();

    let mut histogram: [usize; 256] = [0; 256];

    for pass in 0..T::BYTES {
        histogram.fill(0);
        
        let shift = pass * 8;
        let is_last_pass = pass == T::BYTES - 1;

        for &item in src.iter() {
            let value = item.to_unsigned();
            let mut digit = (value >> shift) & T::Unsigned::from(0xFF);
            
            let mut digit_u8 = unsafe {
                mem::transmute::<T::Unsigned, u8>(digit)
            };

            if T::IS_SIGNED && is_last_pass {
                digit_u8 ^= 0x80;
            }
            histogram[digit_u8 as usize] += 1;
        }

        let mut cumulative_pos = 0;
        for count in histogram.iter_mut() {
            let tmp = *count;
            *count = cumulative_pos;
            cumulative_pos += tmp;
        }

        for &item in src.iter() {
             let value = item.to_unsigned();
             let mut digit = ((value >> shift) & T::Unsigned::from(0xFF)) ;
             let mut digit_u8 = unsafe { mem::transmute::<T::Unsigned, u8>(digit) };

             if T::IS_SIGNED && is_last_pass {
                digit_u8 ^= 0x80;
             }

             let index = digit_u8 as usize;
             dst[histogram[index]] = item;
             histogram[index] += 1;
        }

        mem::swap(&mut src, &mut dst);
    }

    if src.as_ptr() != arr.as_ptr() {
       dst.copy_from_slice(src);
    }
}
