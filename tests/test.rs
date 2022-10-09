#[cfg(test)]
mod tests {
    use bibliotheca_rfid_reader::*;

    #[test]
    fn inventory_and_read() {
        let mut reader = BibliothecaRfidReader::open().unwrap();
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
}