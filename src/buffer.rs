use crate::ReaderError;

pub struct Buffer {
    buf: Vec<u8>,
    finalized: bool
}

#[allow(unused)]
impl Buffer {
    pub fn new(start_byte: u8) -> Self {
        let mut vector = Vec::with_capacity(256);
        vector.push(start_byte);
        vector.push(0x00); // length byte 1
        vector.push(0x00); // length byte 2
        Self {buf: vector, finalized: false}
    }

    pub fn new_with_data(start_byte: u8, data: &[u8]) -> Self {
        let mut buf = Self::new(start_byte);
        buf.write_all(data);
        buf
    }

    pub fn write(&mut self, byte: u8) {
        self.buf.push(byte);
    }

    pub fn write_all(&mut self, data: &[u8]){
        for b in data {
            self.write(*b);
        }
    }

    pub fn finalize(&mut self) -> &[u8] {
        if self.finalized {
            return &self.buf;
        }

        let length = self.buf.len() - 1 & 0xffff;
        self.buf[2] = length as u8;
        self.buf[1] = (length >> 8) as u8;

        let checksum = Self::calc_checksum(&self.buf);
        self.buf.push((checksum >> 8) as u8);
        self.buf.push((checksum & 0xff) as u8);
        self.finalized = true;

        &self.buf
    }

    pub fn start_byte(&self) -> &u8 {
        &self.buf[0]
    }

    pub fn data(&self) -> Option<&[u8]> {
        if self.buf.len() <=3 {
            return None;
        }

        if !self.finalized {
            Some(&self.buf[3..])
        } else {
            Some(&self.buf[3..self.buf.len()-2])
        }
    }

    fn calc_checksum(buf: &[u8]) -> u16 {
        let crc_poly = 0x1021;
        let mut crc: u16 = 0xffff;
    
        for b in &buf[1..buf.len()] {
            let mut current_byte = *b;
            for _ in 0..8 {
                let crc_old = crc as u32;
                crc <<= 1;
                if crc_old >> 0xf  != current_byte as u32 >> 7  {
                    crc ^= crc_poly;
                }
                current_byte <<= 1;
            }
        }
        crc ^= 0xffff;
        crc
    }
}

impl TryFrom<[u8; 256]> for Buffer {
    type Error = ReaderError;

    fn try_from(byte_array: [u8; 256]) -> Result<Self, Self::Error> {
        let mut new_len = byte_array.len();
        for i in 0..(byte_array.len()-1) {
            if byte_array[byte_array.len()-1-i] == 0x00 {
                new_len = byte_array.len()-1-i;
            } else {
                break;
            }
        }

        let buffer = byte_array[0..new_len].to_vec();

        let read_checksum = &buffer[buffer.len()-2..];
        let true_checksum = Self::calc_checksum(&buffer[0..buffer.len()-2]);
        if true_checksum.to_be_bytes() != read_checksum {
            return Err(ReaderError::WrongChecksum);
        }

        Ok( Self {buf: buffer, finalized: true} )
    }
}
