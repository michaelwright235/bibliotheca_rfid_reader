#[cfg(test)]
mod tests {
    use bibliotheca_rfid_reader::*;
    use libftd2xx::*;

    fn open() -> Result<BibliothecaRfidReader, ReaderError> {
        #[cfg(not(windows))]
        set_vid_pid(0x0d2c, 0x032a).unwrap(); // bibliotheca M210 RFID reader
        BibliothecaRfidReader::open()
    }
    
    #[test]
    fn inventory_and_read() {
        let mut reader = open().unwrap();
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

    #[test]
    fn device_info() {
        let mut reader = open().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(3));
        println!("{:?}", reader.ftdi_device_info());
    }
}
