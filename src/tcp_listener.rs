use std::{
    error::Error, fs::File, io::{BufReader, Read, Seek, SeekFrom, Write}, net::{TcpListener, TcpStream}
};
use clap::Parser;

const TCP_FRAME_SIZE: usize = 4096;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Address of the Xspress card
    #[arg(short, long, default_value_t="tcp://127.0.0.1".to_owned())]
    addr: String,

    /// Port to connect to
    #[arg(short, long,default_value_t=":15510".to_owned())]
    port: String,
}


fn write_to_file(file: &mut File, f_lenght: &mut usize, buffer: &mut [u8; TCP_FRAME_SIZE]) -> Result<(), Box<dyn Error>>{
    file.seek(SeekFrom::Start(*f_lenght as u64))?;
    file.write(buffer)?;
    *f_lenght += TCP_FRAME_SIZE;
    Ok(())
}


fn read_into_buffer(
    stream: &mut TcpStream,
    buffer: &mut [u8; TCP_FRAME_SIZE],
) -> Result<(), Box<dyn Error>> {
    let mut filled: usize = 0;
    // Trying out using BufReader to buffer the TCP reads from the sockets
    // Apparently the limit for the buffer is 8KiB so I'm not sure how much it will help
    let mut reader = BufReader::new(stream);
    while filled < buffer.len() {
        match reader.read(&mut buffer[filled..]) {
            Ok(0) => {
                println!("Connection closed!");
                return Ok(());
            }
            Ok(n) => {
                println!("Read {n} bytes");
                filled += n;
            }
            Err(e) => {
                println!("Error! {e}");
                return Err(Box::new(e));
            }
        }
    }
    return Ok(());
}

fn listen_to_stream(addr: String) -> Result<(), Box<dyn Error>> {
    let listener = match TcpListener::bind(&addr) {
        Ok(listener) => {
            println!("Connected to {addr}");
            listener
        }
        Err(e) => {
            println!("Could not connect to {addr}");
            return Err(Box::new(e));
        }
    };

    let mut file: File = match File::create("/workspaces/data/test_write.txt") {
        Ok(file) => {
            println!("Opened the file!");
            file
        }
        Err(e) => {
            println!("Could not open the file! {e}");
            return Err(Box::new(e));
        }
    };
    let mut f_lenght: usize = 0;


    let mut buffer = [0u8; TCP_FRAME_SIZE];
    for value in listener.incoming() {
        println!("waiting...");
        match value {
            Ok(mut stream) => {
                match read_into_buffer(&mut stream, &mut buffer) {
                    Ok(_) => {
                        write_to_file(&mut file, &mut f_lenght, &mut buffer)?;
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Err(e) => {
                println!("Error listening to tcp stream.\n{e}");
                return Err(Box::new(e));
            }
        }
        if f_lenght >= 100 * TCP_FRAME_SIZE {
            println!("Closing the file and returning");
            drop(file);
            break;
        }
        
        // // An i16 has 2 bytes (16 bits). I then need to chunk
        // // my data so that I transform every 2 bytes into a single
        // // number.
        // // Map gives me the rule to create an iterator that does the conversion
        // // and collect transforms the iterator into an array.
        // let i32_values: Vec<i32> = buffer
        //     .chunks_exact(4)
        //     .map(|chunk| i32::from_be_bytes(chunk.try_into().unwrap()))
        //     .collect();
        // println!("i32_values.len() = {0}", i32_values.len());
        // println!("i32_values = {:?}", i32_values);
    }
    Ok(())
}

fn main() {
    let args: Args = Args::parse();
    let _ = listen_to_stream(args.addr + ":" + &args.port);
    println!("Hello world!");
}
