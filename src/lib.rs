mod buffer;
mod error;

use std::time::Duration;
use libftd2xx::{Ftdi, FtdiCommon, BitsPerWord, StopBits, Parity, FtStatus};
use buffer::*;
pub use error::*;
pub struct BibliothecaRfidReader {
    handle: Ftdi,
    timeout: Duration
}

impl BibliothecaRfidReader {
    pub fn open() -> Result<Self, ReaderError> {
        let handle = Ftdi::new()?;
        Self::open_with_handle(handle)
    }

    pub fn open_with_description(desc: &str) -> Result<Self, ReaderError> {
        let handle = Ftdi::with_description(desc)?;
        Self::open_with_handle(handle)
    }

    pub fn open_with_handle(handle: Ftdi) -> Result<Self, ReaderError> {
        let mut reader = Self {handle: handle, timeout: Duration::from_millis(200)};
        reader.prepare_reader()?;
        Ok(reader)
    }

    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    pub fn timeout(&self) -> &Duration {
        &self.timeout
    }

    fn prepare_reader(&mut self) -> Result<(), ReaderError> {
        self.handle.set_latency_timer(Duration::from_millis(16))?;
        self.handle.set_baud_rate(19200)?;
        self.handle.set_data_characteristics(BitsPerWord::Bits8, StopBits::Bits1, Parity::No)?;
        self.handle.clear_dtr()?;
        self.handle.clear_rts()?;
        self.handle.set_flow_control_none()?;

        let mut command1 = Buffer::new_with_data(0xd5, &[0x04,0x00,0x11]);
        self.perform_command(command1.finalize())?;

        let mut command2 = Buffer::new_with_data(0xd6, &[0x13, 0x06, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x0b, 0x00, 0x0a, 0x00]);
        self.perform_command(command2.finalize())?;

        let mut command3 = Buffer::new_with_data(0xd6, &[0x58,0x00,0x03,0x02,0x3a,0x02,0x00]);
        self.perform_command(command3.finalize())?;

        Ok(())
    }

    fn perform_command(&mut self, buffer: &[u8]) -> Result< Vec<u8>, ReaderError > {
        self.write(buffer)?;
        std::thread::sleep(self.timeout);
        self.read()
    }

    fn write(&mut self, buffer: &[u8]) -> Result<(), FtStatus> {
        self.handle.write_all(buffer).unwrap();
        print!("Wrote data: [ ");
        for b in buffer {
            print!("{b:#X} ");
        }
        println!("]");
        Ok(())
    }
    
    fn read(&mut self) -> Result< Vec<u8>, ReaderError> {
        let mut out_buffer: [u8; 256] = [0; 256];
        let bytes = self.handle.queue_status()?;
        if bytes > 0 {
            self.handle.read(&mut out_buffer[0..bytes])?;

            let new_buf = Buffer::try_from(out_buffer).unwrap();
            
            print!("Read data: [ ");
            for b in new_buf.data().unwrap() {
                print!("{b:#X} ");
            }
            println!("]");
    
            return Ok(new_buf.data().unwrap().to_vec());
        }
        Err(ReaderError::EmptyResponse)
    }

    pub fn inventory(&mut self) -> Result< Vec<[u8;8]>, ReaderError> {
        let mut command = Buffer::new_with_data(0xd6, &[0xfe,0x00,0x07]);
        let result = self.perform_command(command.finalize())?;
        if result.len() < 5 {
            return Err(ReaderError::WrongResponse);
        }

        let num_of_cards = result[4];
        if num_of_cards == 0 {
            return Err(ReaderError::NoCard);
        }

        let mut cards = Vec::with_capacity(num_of_cards.into());
        for i in 0..num_of_cards {
            let mut card = [0; 8];
            let from = 6 + 9*i;
            let to = from + 8;

            let mut k = 0;
            for j in from..to {
                card[k] = result[j as usize];
                k += 1;
            }

            cards.push(card);
        }
        Ok(cards)
    }

    pub fn read_card(&mut self, card_id: &[u8]) -> Result<Vec<u8>, ReaderError> {
        if card_id.len() != 8 {
            return Err(ReaderError::WrongCardId);
        }

        let mut command = Buffer::new(0xd6);
        command.write(0x02);
        command.write_all(card_id);
        command.write_all(&[0x00,0x09,0x0c]);

        let result = self.perform_command(command.finalize())?;
        if result.len() < 3 {
            return Err(ReaderError::WrongResponse);
        }
        if result[1] != 0x00 {
            return Err(ReaderError::NoCard);
        }

        Ok(result[12..].to_vec())

    }

    pub fn write_card(&mut self, card_id: &[u8], data: &[u8]) -> Result<(), ReaderError> {
        if card_id.len() != 8 {
            return Err(ReaderError::WrongCardId);
        }

        let mut command = Buffer::new(0xd6);
        command.write(0x04);
        command.write_all(&card_id);
        command.write(0x00);
        command.write(0x09);
        command.write(0x00);
        command.write_all(&data);
        let zeros = [0; 8];
        command.write_all(&zeros);

        let result = self.perform_command(command.finalize())?;
        if result[1] != 0x00 { // 0x06 - no card
            return Err(ReaderError::NoCard);
        }

        Ok(())
    }

    pub fn custom_command(&mut self, buf: &[u8]) -> Result<Vec<u8>, ReaderError> {
        self.perform_command(buf)
    }
}

#[allow(unused_must_use)]
impl Drop for BibliothecaRfidReader {
    fn drop(&mut self) {
        self.handle.close();
    }
}
