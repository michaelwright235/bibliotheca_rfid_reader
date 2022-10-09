mod buffer;
mod error;

use bibliotheca_rfid_reader::BibliothecaRfidReader;
use buffer::*;
use error::*;

fn main() {
    let mut reader = BibliothecaRfidReader::open().unwrap();
    //inventory_and_read(&mut reader);
    //let mut command3 = Buffer::new_with_data(0xd6, &[0xfe]);
    //reader.custom_command(command3.finalize());
    reader.write_card(&[0xe0,0x04,0x01,0x50,0x93,0xa4,0xe9,0x0e], 
        &[0x81,0x01,0x01,0x32,0x39,0x33,0x35,0x30,0x30,0x30,0x30,0x30,0x33,0x36,0x34,0x39,0x00,0x00,0x00,0x06,0xe7,0x52,0x55,0x32,0x39,0x33,0x2d,0x30]).unwrap();

}

fn inventory_and_read(reader: &mut BibliothecaRfidReader) {
    let inv = reader.inventory();
    if inv.is_err() {
        println!("{}", inv.err().unwrap().to_string());
    }
    else {
        for c in inv.unwrap() {
         print!("Card: ");
         for b in &c {
             print!("{b:#X} ");
         }
         println!();
         let data = reader.read_card(&c);
         if data.is_err() {
            println!("{}", data.err().unwrap().to_string());
         }
         else {
            print!("Data: ");
            for b in data.unwrap() {
                print!("{b:#X} ");
            }
         }
         println!();
     }
    }
}