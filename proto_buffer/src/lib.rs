use std::convert::TryInto;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Endian {
    BigEndian,
    LittleEndian
}

#[derive(Debug)]
pub struct Buffer {
    data: Vec<u8>,
    pub pos: usize,
    pub endian: Endian,
}

pub trait ToBytes {
    fn to_bytes(self, endian: Endian) -> Box<[u8]>;
}

pub trait FromBytes {
    fn from_bytes(endian: Endian, s: &[u8]) -> Self;
}

pub trait ProtoWriter {
    fn proto_write(&self, buf: &mut Buffer);
}

pub trait ProtoReader {
    fn proto_read(buf: &mut Buffer) -> Self;
}

impl ToBytes for u8 {
    fn to_bytes(self, _endian: Endian) -> Box<[u8]> {
        Box::new([self]) 
    }
}

impl ToBytes for i8 {
    fn to_bytes(self, _endian: Endian) -> Box<[u8]> {
        Box::new([self as u8]) 
    }
}

impl ToBytes for bool {
    fn to_bytes(self, _endian: Endian) -> Box<[u8]> {
        Box::new([if self {1}  else {0}]) 
    }
}

impl ToBytes for () {
    fn to_bytes(self, _endian: Endian) -> Box<[u8]> {
        Box::new([1]) 
    }
}

impl ToBytes for &str {
    fn to_bytes(self, _endian: Endian) -> Box<[u8]> {
        self.to_owned().into_boxed_str().into_boxed_bytes()
    }
}

impl ProtoWriter for u8 {
    fn proto_write(&self, buf: &mut Buffer) {
       buf.write_u8(self);
    }
} 

impl ProtoWriter for i8 {
    fn proto_write(&self, buf: &mut Buffer) {
       let u = *self as u8;
       buf.write_u8(&u);
    }
} 

impl ProtoWriter for bool {
    fn proto_write(&self, buf: &mut Buffer) {
       let u = if *self {1} else {0};
       buf.write_u8(&u);
    }
} 

impl ProtoWriter for () {
    fn proto_write(&self, buf: &mut Buffer) {
       buf.write_u8(&1);
    }
} 

impl ProtoWriter for &str {
    fn proto_write(&self, buf: &mut Buffer) {
        buf.write_utf8(self);
    }
} 

impl ProtoWriter for String {
    fn proto_write(&self, buf: &mut Buffer) {
        buf.write_utf8(self);
    }
} 

macro_rules! impl_ToBytes {
    ($($t:ty), +) => {
        $(impl ToBytes for $t {
            fn to_bytes(self, endian: Endian) -> Box<[u8]> {
                if endian == Endian::BigEndian {
                    Box::new(self.to_be_bytes()) 
                } else {
                    Box::new(self.to_le_bytes()) 
                }
            }
        })*
    }
}

macro_rules! impl_ProtoWrite {
    ($($t:ty), +) => {
        $(impl ProtoWriter for $t {
            fn proto_write(&self, buf:&mut Buffer) {
                if buf.endian == Endian::BigEndian {
                    buf.write_slice_u8(&self.to_be_bytes()) 
                } else {
                    buf.write_slice_u8(&self.to_le_bytes()) 
                }
            }
        })*
    }
}

impl FromBytes for u8 {
    fn from_bytes(_endian: Endian, s: &[u8]) -> Self {
        s[0]
    }
}

impl FromBytes for i8 {
    fn from_bytes(_endian: Endian, s: &[u8]) -> Self {
        s[0] as i8
    }
}

impl FromBytes for bool {
    fn from_bytes(_endian: Endian, s: &[u8]) -> Self {
        if s[0] == 0 {false} else {true}
    }
}

impl FromBytes for () {
    fn from_bytes(_endian: Endian, _s: &[u8]) -> Self {
        ()
    }
}

impl ProtoReader for u8 {
    fn proto_read(buf: &mut Buffer) -> Self {
        buf.read_u8()
    }
}

impl ProtoReader for i8 {
    fn proto_read(buf: &mut Buffer) -> Self {
        buf.read_u8() as i8
    }
}

impl ProtoReader for bool {
    fn proto_read(buf: &mut Buffer) -> Self {
        if buf.read_u8() == 0 {false} else {true}
    }
}

impl ProtoReader for () {
    fn proto_read(buf: &mut Buffer) -> Self {
        let _ = buf.read_u8();
        ()
    }
}


macro_rules! impl_FromBytes {
    ($($t:ty), +) => {
        $(impl FromBytes for $t {
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

macro_rules! impl_ProtoReader {
    ($($t:ty), +) => {
        $(impl ProtoReader for $t {
            fn proto_read(buf:&mut Buffer) -> Self {
                if buf.endian == Endian::BigEndian {            
                    Self::from_be_bytes(buf.read_slice_u8(std::mem::size_of::<Self>()).try_into().unwrap())
                } else {
                    Self::from_le_bytes(buf.read_slice_u8(std::mem::size_of::<Self>()).try_into().unwrap())
                }
            }
        })*
    }
}

impl_ProtoWrite! (u16, u32, u64, usize, f32, f64, i16, i32, i64);
impl_ProtoReader! (u16, u32, u64, usize, f32, f64, i16, i32, i64);

impl_ToBytes! (u16, u32, u64, usize, f32, f64, i16, i32, i64);
impl_FromBytes! (u16, u32, u64, usize, f32, f64, i16, i32, i64);

impl ProtoReader for String {
    fn proto_read(buf: &mut Buffer) -> Self {
        String::from(buf.read_utf8())
    }
}

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

    pub fn read<T:FromBytes>(&mut self) -> T {
       T::from_bytes(self.endian, self.read_slice_u8(std::mem::size_of::<T>())) 
    }

    fn write_u8(&mut self, v:&u8) {        
        if self.pos == self.data.len() {
            self.data.push(*v);
        } else {
            self.data[self.pos] = *v;
        }

        self.pos += 1; 
    }

    fn read_u8(&mut self) -> u8 {
        self.pos += 1;

        self.data[self.pos - 1]
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
            self.data.reserve(self.pos + str_len + std::mem::size_of::<usize>());
        }

        self.write::<usize>(str_len);

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

            self.pos += str_len; 
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


    pub fn read_vec<T:FromBytes + Copy>(&mut self) -> Vec<T> {
        let len = self.read::<usize>();
        let mut v = Vec::with_capacity(len);
        
        for _ in 0..len {
            v.push(self.read::<T>())
        }

        v
    }
}

//cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[derive(Debug, PartialEq)]
    struct User {
        name: String,
        email: String, 
        age: u8    
    }

    impl ProtoWriter for User {
        fn proto_write(&self, buf:&mut Buffer) {
            self.name.proto_write(buf);
            self.email.proto_write(buf);
            self.age.proto_write(buf);
        }
    }

    impl ProtoReader for User {
        fn proto_read(buf:&mut Buffer) -> Self {
            return User {
                name: String::proto_read(buf),
                email: String::proto_read(buf),
                age: u8::proto_read(buf)
            }
        }
    }

    #[test]
    fn user() {
        let mut b = Buffer::new();
        let user = User {
            name: String::from("Den"), 
            email: String::from("nastvood@gmail.com"), 
            age: 37
        };

        user.proto_write(&mut b);

        b.pos = 0;
        let readed_user = User::proto_read(&mut b);
        
        assert_eq!(user, readed_user);
    }

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

        {
            let mut b = Buffer::new();
            let start = Instant::now();
            "123456789".proto_write(&mut b);
            "123456789".proto_write(&mut b);
            "123456789".proto_write(&mut b);
            "123456789".proto_write(&mut b);
            "123456789".proto_write(&mut b);
            let duration = start.elapsed();

            println!("Time elapsed in proto_write is: {:?}", duration);        
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

    macro_rules! test_simple_type {
        ($func_name:ident, $t:ty, data $v0:expr, $v1:expr, $v2:expr, mdata $offset:expr, $v:expr, $tm:ty, $last:expr) => (
            #[test]
            fn $func_name() {
                let mut b = Buffer::new();
                $v0.proto_write(&mut b);
                $v1.proto_write(&mut b);
                $v2.proto_write(&mut b);

                b.pos = std::mem::size_of::<$t>();

                assert_eq!($v1, b.read::<$t>());

                b.pos = $offset;
                $v.proto_write(&mut b);

                //println!("{:?}", b);

                b.pos = $offset;
                assert_eq!($last, <$tm>::proto_read(&mut b));
            }
        )
    }

    test_simple_type! (bool, bool, data true, false, false, mdata 1, true, u16, 256);
    test_simple_type! (unit, (), data (), (), (), mdata 1, (), u16, 257);

    test_simple_type! (f64, f64, data 1.4242, -2.424, -3444., mdata 1, 10.23424, u16, 16420);
    test_simple_type! (f32, f32, data 1.4242f32, -2.424f32, -3444.0f32, mdata 1, 10.23424f32, u16, 16675);

    test_simple_type! (u8, u8, data 1u8, 2u8, 3u8, mdata 1, 10u8, u16, 2563);
    test_simple_type! (u16, u16, data 623u16, 10326u16, 7596u16, mdata 2, 10u16, u32, 662956u32);
    test_simple_type! (u32, u32, data 1462387564u32, 2423423u32, 344u32, mdata 10, 104324u32, u16, 1u16);
    test_simple_type! (u64, u64, data 146327563875423464u64, 234214234423423u64, 14234242423u64, mdata 2, 142342424242424u64, u32, 33141u32);

    test_simple_type! (i8, i8, data 1i8, -2i8, -3i8, mdata 1, 10i8, i16, 2813i16);
    test_simple_type! (i16, i16, data -1625i16, 2221i16, -25632i16, mdata 1, -10i16, u16, 65526u16);
    test_simple_type! (i32, i32, data -1625i32, 2221i32, -25632i32, mdata 3, 10i32, i64, 45868908443i64);
    test_simple_type! (i64, i64, data -1625i64, 2221i64, -25632i64, mdata 1, 4236876876786423424i64, i32, 986474770i32);


    #[test]
    fn str() {
        let mut b = Buffer::new();

        b.write_utf8("[DIY家具] 収納椅子をつくる");
        b.pos = 0;

        assert_eq!("[DIY家具] 収納椅子をつくる", b.read_utf8());

        b.pos = 11 + std::mem::size_of::<usize>();
        b.write::<u8>(0x5f);

        b.pos = 0;

        assert_eq!("[DIY家具]_収納椅子をつくる", b.read_utf8());
    }

    #[test]
    fn proto_string() {
        let mut b = Buffer::new();

        "[DIY家具] 収納椅子をつくる".proto_write(&mut b);
        b.pos = 0;

        assert_eq!("[DIY家具] 収納椅子をつくる", String::proto_read(&mut b));

        b.pos = 11 + std::mem::size_of::<usize>();
        0x5fu8.proto_write(&mut b);

        b.pos = 0;

        assert_eq!("[DIY家具]_収納椅子をつくる", String::proto_read(&mut b));
    }
}

