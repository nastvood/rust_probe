
#[derive(Debug)]
pub enum Opcode {
    Continuation, 
    Text(String), 
    Binary, 
    Close, 
    Ping, 
    Pong, 
    Other
}

#[derive(Debug)]
pub struct Frame {
    fin: bool,
    pub opcode: Opcode,
    mask: bool,
    size: usize,
    mask_val: [u8; 4]
}

impl Frame {
    pub fn new(buf:&Vec<u8>) -> Frame {
        let mut offset = 0;

        let fin = buf[offset] >> 7 != 0;

        if !fin {
            panic!{"Only short messages"}
        }

        let opcode = buf[offset] & 0x0F;

        offset += 1;
        let mask = buf[offset] >> 7 != 0;

        let size = match buf[offset] & 0x7F {
            v if v < 126 => {
                offset += 1;
                v as usize
            },
            126 => {
                offset += 3;
                u16::from_be_bytes([buf[2], buf[3]]) as usize //from_ne_bytes?
            },
            127 => {
                offset += 9;
                u64::from_be_bytes([buf[2], buf[3], buf[4], buf[5], buf[6], buf[7], buf[8], buf[9]]) as usize //?
            },
            v => panic!{"Reading ws frame size from number {}", v}, 
        };

        let mask_val = if mask {
            offset += 4;        
            [buf[offset - 4], buf[offset - 3], buf[offset - 2], buf[offset - 1]]
        } else {
            [0u8; 4]
        };

        let opcode = match opcode {
            0x0 => Opcode::Continuation,
            0x1 => {
                let data:&Vec<u8> = &buf[offset..].iter().enumerate().map(|(i, x)| {
                    x ^ mask_val[i % 4]
                }).collect();

                match String::from_utf8(data.to_vec()) {
                    Ok(text) => Opcode::Text(text),
                    _ => Opcode::Text(String::from(""))
                }
            }
            0x2 => Opcode::Binary,
            0x8 => Opcode::Close, //2 bytes data
            0x9 => Opcode::Ping,
            0xa => Opcode::Pong,
            _ => Opcode::Other,
        };

        Frame {
            fin,
            opcode,
            mask,
            size,
            mask_val
        }
    }
}

impl From<&str> for Frame {
    fn from(s: &str) -> Self {
        Frame {
            fin: true,
            opcode:Opcode::Text(s.to_owned()),
            mask: false,
            size:s.len(),
            mask_val:[0u8; 4],
        }
    }
}

impl From<&Frame> for Vec<u8> {
    fn from(frame: &Frame) -> Self {
        let data_len = match &frame.opcode {
            Opcode::Text(s) => s.len(),
            _ => 0
        };

        let mut buf = Vec::with_capacity(12 + data_len);      

        //let mut offset = 0;

        let mut byte:u8 = if frame.fin { 1 << 7 } else { 0 };
        let opcode = match &frame.opcode {
            Opcode::Continuation => 0x0,
            Opcode::Text(_) => 0x1,
            Opcode::Binary => 0x2,
            Opcode::Close => 0x8,
            Opcode::Ping => 0x9,
            Opcode::Pong => 0xa,
            Opcode::Other => panic!{"Number from opcode"}
        };
        byte += opcode;
        buf.push(byte);    

        byte = if frame.mask { 1 << 7 } else { 0 };
        match data_len {
            len if len < 126 => {
                byte += len as u8;
                buf.push(byte)
            },

            len if len < 65536 => {                
                byte += 126;
                buf.push(byte);

                let bytes = (len as u16).to_be_bytes();
                buf.push(bytes[0]);
                buf.push(bytes[1])                
            },

            len => {
                byte += 128;
                buf.push(byte);

                let bytes = (len as u64).to_be_bytes();
                buf.push(bytes[0]);
                buf.push(bytes[1]);
                buf.push(bytes[2]);               
                buf.push(bytes[3]);                
                buf.push(bytes[4]);                
                buf.push(bytes[5]);                
                buf.push(bytes[6]);                
                buf.push(bytes[7]);                
            }
        }

        if let Opcode::Text(s) = &frame.opcode {
            unsafe {
                let len = s.len();
                if len > 0 {
                    let new_len = len + buf.len();
                    s.as_ptr().copy_to(buf.as_mut_ptr().offset(buf.len() as isize), s.len());
                    buf.set_len(new_len)
                }
            }
        }

        buf
    }
}
