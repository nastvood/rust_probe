
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

        let fin = buf[offset].wrapping_shr(7) != 0;

        if !fin {
            panic!{"Only short messages"}
        }

        let opcode = buf[offset] & 0x0F;

        offset += 1;
        let mask = buf[offset].wrapping_shr(7) != 0;

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

        let mask_val = [buf[offset], buf[offset + 1], buf[offset + 2], buf[offset + 3]];
        offset += 4;        

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
