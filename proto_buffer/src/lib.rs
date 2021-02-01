use std::convert::TryInto;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Endian {
    BigEndian,
    LittleEndian
}

#[derive(Debug)]
pub struct Buffer {
    data: Vec<u8>,
    pos: usize,
    endian: Endian,
}

pub trait ToBytes {
    fn to_bytes(self, endian: Endian) -> Box<[u8]>;
}

pub trait FromBytes<T> {
    fn from_bytes(endian: Endian, s: &[u8]) -> T;
}

impl ToBytes for u8 {
    fn to_bytes(self, _endian: Endian) -> Box<[u8]> {
        Box::new([self]) 
    }
}

impl ToBytes for &str {
    fn to_bytes(self, _endian: Endian) -> Box<[u8]> {
        self.to_owned().into_boxed_str().into_boxed_bytes()
    }
}

macro_rules! impl_ToBytes {
    (for $($t:ty), +) => {
        $(impl ToBytes for $t {
            fn to_bytes(self, endian: Endian) -> Box<[u8]> {
                if endian == Endian::BigEndian {
                    Box::new(self.to_be_bytes()) 
                } else {
                    Box::new(self.to_be_bytes()) 
                }
            }
        })*
    }
}

impl_ToBytes! (for u16, u32, u64, usize, f64);

impl FromBytes<u8> for u8 {
    fn from_bytes(_endian: Endian, s: &[u8]) -> u8 {
        s[0]
    }
}

macro_rules! impl_FromBytes {
    (for $($t:ty), +) => {
        $(impl FromBytes<Self> for $t {
            fn from_bytes(endian: Endian, s: &[u8]) -> Self {
                if endian == Endian::BigEndian {            
                    Self::from_be_bytes(s.try_into().unwrap())
                } else {
                    Self::from_le_bytes(s.try_into().unwrap())
                }
            }
        })*
    }
}

impl_FromBytes! (for u16, u32, u64, usize, f64);

impl Buffer {

    pub fn build_buffer(cap: usize, endian: Endian) -> Buffer{
        Buffer {
            data: Vec::with_capacity(cap),
            pos: 0,
            endian        
        }
    }

    pub fn new() -> Buffer {
        Buffer {
            data: Vec::new(),
            pos: 0,
            endian: Endian::BigEndian
        }
    }

    pub fn write<T:ToBytes>(&mut self, v: T) {
        let bytes = v.to_bytes(self.endian);

        self.write_slice_u8(&bytes);
    }

    pub fn read<T:FromBytes<T>>(&mut self) -> T {
       T::from_bytes(self.endian, self.read_slice_u8(std::mem::size_of::<T>())) 
    }

    fn write_slice_u8(&mut self, v:&[u8]) {        
        let mut data_len = self.data.len();

        if (data_len - self.pos) == 0 {
            self.data.reserve(data_len + v.len());
        }

        for i in v {
            if self.pos == data_len {
                self.data.push(*i);
                data_len += 1;
            } else {
                self.data[self.pos] = *i;
            }

            self.pos += 1; 
        }
    }

    fn read_slice_u8(&mut self, len: usize) -> &[u8] {
        self.pos += len;

        &self.data[(self.pos - len) .. self.pos]
    }

    pub fn write_utf8(&mut self, v:&str) {
        let str_len = v.len();

        if self.data.capacity() < self.pos + str_len + std::mem::size_of::<usize>() {
            self.data.reserve(self.pos + str_len + std::mem::size_of::<usize>() - self.data.capacity());
        }

        self.write::<usize>(str_len);

        //self.write_slice_u8(v.as_bytes());

        let mut add_len = 0;
        if self.data.len() < self.pos + str_len {
            add_len = self.pos + str_len - self.data.len();
            add_len = if add_len > 0 {add_len} else {0};
        }

        unsafe {
            v.as_ptr().copy_to(self.data.as_mut_ptr().offset(self.pos as isize), v.len());

            if add_len > 0 {
                self.data.set_len(self.pos + add_len);
            }
        }
    }


    pub fn read_utf8(&mut self) -> &str {
        let len = self.read::<usize>();

        std::str::from_utf8(self.read_slice_u8(len)).unwrap()
    }

    pub fn write_vec<T:ToBytes + Copy>(&mut self, xs: &[T]) {
        self.write::<usize>(xs.len());
        
        for x in xs.iter() {
            self.write::<T>(*x);
        }
    }


    pub fn read_vec<T:FromBytes<T> + Copy>(&mut self) -> Vec<T> {
        let len = self.read::<usize>();
        let mut v = Vec::with_capacity(len);
        
        for _ in 0..len {
            v.push(self.read::<T>())
        }

        v
    }

    /*pub fn write_elems<T>(&mut self, xs: &[T], writer: &dyn Fn(&T) -> ()) {
        self.write::<usize>(xs.len());
        
        for x in xs.iter() {
            writer(x);
        }
    }*/

}

//cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn measure() {
        {
            let mut b = Buffer::new();
            let start = Instant::now();
            b.write::<&str>("123456789");
            b.write::<&str>("123456789");
            b.write::<&str>("123456789");
            b.write::<&str>("123456789");
            b.write::<&str>("123456789");
            let duration = start.elapsed();

            println!("Time elapsed in b.write::<&str> is: {:?}", duration);        
        }

        {
            let mut b = Buffer::new();
            let start = Instant::now();
            b.write_utf8("123456789");
            b.write_utf8("123456789");
            b.write_utf8("123456789");
            b.write_utf8("123456789");
            b.write_utf8("123456789");
            let duration = start.elapsed();

            println!("Time elapsed in b.write_utf8 is: {:?}", duration);        
        }

    }

    #[test]
    fn vec() {
        let mut b = Buffer::new();
        
        let v = vec![1, 2, 4, 6];
        b.write_vec::<u8>(v.as_slice());                  

        b.pos = 0;
        
        assert_eq!(v, b.read_vec::<u8>());
    }

    #[test]
    fn simple() {
        let mut b = Buffer::build_buffer(0, Endian::BigEndian);
        b.write::<u32>(3201695);
        b.write::<u64>(125863201695);

        //println!("{:?}", b);

        b.pos = 0;

        assert_eq!(3201695, b.read::<u32>());
        assert_eq!(125863201695, b.read::<u64>());

        //println!("{:?}", b);

        //let s = String::from("Деннис");
        //println!("{} {}", s, s.len());
    }

    #[test]
    fn f64() {
        let mut b = Buffer::new();        
        b.write::<f64>(-5682.5263);
        b.pos = 0;
        assert_eq!(-5682.5263, b.read::<f64>());
    }

    #[test]
    fn u8() {
        let mut b = Buffer::new();

        b.write::<u8>(1);
        b.write::<u8>(4);
        b.write::<u8>(3);

        assert_eq!(vec![1, 4, 3], b.data);

        b.pos = 1;

        b.write::<u8>(2);

        assert_eq!(vec![1, 2, 3], b.data);

        b.write::<u8>(5);

        assert_eq!(vec![1, 2, 5], b.data);

        b.pos = 2;
        assert_eq!(5, b.read::<u8>());
    }

    #[test]
    fn u16() {
        let mut b = Buffer::new();

        b.write::<u16>(623);
        b.write::<u16>(10362);
        b.write::<u16>(7596);

        b.pos = 2;

        assert_eq!(10362, b.read::<u16>());

        b.pos = 4;
        b.write::<u8>(10);
        b.pos = 4;
        assert_eq!(2732, b.read::<u16>());

    }

    #[test]
    fn str() {
        let mut b = Buffer::new();

        b.write_utf8("[DIY家具] 収納椅子をつくる");
        b.pos = 0;

        //println!("{:?}", b);
        assert_eq!("[DIY家具] 収納椅子をつくる", b.read_utf8());

        b.pos = 11 + std::mem::size_of::<usize>();
        b.write::<u8>(0x5f);

        b.pos = 0;

        assert_eq!("[DIY家具]_収納椅子をつくる", b.read_utf8());
    }
}

