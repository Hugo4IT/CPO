use std::{borrow::Borrow, fs::File, io::Read, path::Path, str, usize};

use clap::{App, Arg};

extern crate clap;

fn main() {
    let app = App::new("cpo")
        .arg(Arg::with_name("INPUT_FILE")
            .index(1)
            .required(true)
            .help("This is the file that will be copied"))
        .arg(Arg::with_name("START_OFFSET")
            .index(2)
            .required(true)
            .help("Hexadecimal offset, does not start with 0x"))
        .arg(Arg::with_name("END_OFFSET")
            .index(3)
            .required(true)
            .help("Hexadecimal offset, does not start with 0x"))
        .help("Example: `cpo hello.exe 0500C32 32000AB > output.txt`\n This will result in a text file called `output.txt` with the extracted data")
        .get_matches();
    
    let input_file = app.value_of("INPUT_FILE").unwrap();
    let start_offset: i32 = i32::from_str_radix(app.value_of("START_OFFSET").unwrap(), 16).unwrap();
    let end_offset: i32 = i32::from_str_radix(app.value_of("END_OFFSET").unwrap(), 16).unwrap();

    let path = Path::new(input_file);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut contents = Vec::new();
    match file.read_to_end(&mut contents) {
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(_) => {}
    }

    let part = contents[(start_offset as usize)..(end_offset as usize)].to_vec();
    let result = str::from_utf8(part.borrow()).unwrap().replace("\\n", "\n");

    println!("{:?}", result);
}
