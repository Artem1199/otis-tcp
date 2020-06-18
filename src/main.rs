#[macro_use]
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use byteorder::{ByteOrder, LittleEndian};
use dotenv::dotenv;
use std::env;
use std::time::Duration;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::time::Instant;

use std::f32::consts::PI;

fn main () -> std::io::Result<()>{

    let device_state = DeviceState::new();

    // Initial TCP connection with Arduino
    let mut stream = attempt_arduino_connection();

    let mut data = [0 as u8; 8];
    stream.read(&mut data);


    // Read 1st msg
    let text = from_utf8(&data).unwrap();
    println!("{}", text);

    let sent_time = Instant::now();
    loop{
        stream.read(&mut data);
        if data == [0 as u8; 8]{
            println!("Connection lost");
            stream = attempt_arduino_connection();
        }

        let keys: Vec<Keycode> = device_state.get_keys();
        
        if (sent_time.elapsed().as_millis() > 500){
            if(keys.contains(&Keycode::I)){
                stream.write(&[1]);
                let sent_time = Instant::now();
            }
            if(keys.contains(&Keycode::K)){
                stream.write(&[2]);
                let sent_time = Instant::now();
            }
            if(keys.contains(&Keycode::O)){
                stream.write(&[3]);
                let sent_time = Instant::now();
            } 
	    if(keys.contains(&Keycode::L)){
                stream.write(&[4]);
                let sent_time = Instant::now();
            } 
	    if(keys.contains(&Keycode::J)){
                stream.write(&[5]);
                let sent_time = Instant::now();
            }
	    if(keys.contains(&Keycode::P)){
                stream.write(&[6]);
                let sent_time = Instant::now();
            }
	    if(keys.contains(&Keycode::F)){
                stream.write(&[7]);
                let sent_time = Instant::now();
            }	
        }
        
        // decode data stream
       let mut p = [data[0] as u8, data[1] as u8];
       let p_float: f32 = ((LittleEndian::read_u16(&p) as f32) / 10436.381 - PI) * 180.0/(PI) ;
       let mut y = [data[2] as u8, data[3] as u8];
       let y_float: f32 = ((LittleEndian::read_u16(&y) as f32) / 10436.381 - PI) * 180.0/(PI) ;
       let mut o = [data[4] as u8, data[5] as u8];
       let o_float: f32 = (LittleEndian::read_u16(&o) as f32) / 33.0 - 1000.0 ;
       let mut g = [data[6] as u8, data[7] as u8];
       let g_float: f32 = (LittleEndian::read_u16(&g) as f32) / 33.0 - 1000.0 ;

       println!("P: {}, Y: {}, O: {}, G: {}", p_float, y_float, o_float, g_float);

        // reset data buffer
        data = [0 as u8; 8];
    }
}

pub fn attempt_arduino_connection () -> std::net::TcpStream {

    // attempt to connect to arduino on loop
    let mut stream = loop {

    /* "192.168.50.45:80" for MKR on older twip */
    /* "192.168.50.181:80" for newer robot */

        if let Ok(stream) = TcpStream::connect("192.168.50.181:80") { 
            println!("Connected to the server!");
            break stream;
        } else {
           // println!("Couldn't connect to server...");

        }
    };

    // Set timeout condition to 5 seconds
    stream.set_read_timeout(Some(Duration::new(5, 0))).expect("set_read_timeout failed");
    stream.write(&[3]).expect("Failed to send data to Arduino");
    stream

}
