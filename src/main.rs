#[macro_use]
extern crate diesel;

use std::io::{self, Read, Write};
use std::io::prelude::*;
use std::net::TcpStream;
use std::str::from_utf8;
use byteorder::{ByteOrder, LittleEndian, BigEndian};
use diesel::prelude::*;
//use diesel::pg::PgConnection;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;
use std::env::args;

use self::models::*;
use self::diesel::prelude::*;

pub mod schema;
pub mod models;

#[derive(Queryable)]
pub struct Otis_AK{
    pub id: i32,
    pub PITCH: f32,
    pub YAW: f32,
    pub OUTPUT1: f32,
    pub OUTPUT2: f32,
}

pub fn save_data<'a>(conn: &MysqlConnection, 
    PITCH: &'a f32,
    YAW: &'a f32,
    OUTPUT1: &'a f32,
    OUTPUT2: &'a f32,) /*-> Otis_AK*/{  //TRY WITHOUT RETURNING values

     use schema::OtisData;

     let new_data = NewData{
         PITCH:PITCH,
         YAW:YAW,
         OUTPUT1:OUTPUT1,
         OUTPUT2:OUTPUT2,
     };

     diesel::insert_into(OtisData::table)
     .values(&new_data)
     .execute(conn); //alt use get_result()
     //expect("Error saving new post");

    }



pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main () -> std::io::Result<()>{

    // Initial connection with database
    let connection = establish_connection();

    connection.execute("TRUNCATE TABLE OtisData").unwrap();

    // Initial TCP connection with Arduino
    let mut stream = TcpStream::connect("192.168.50.45:80")?;
    stream.write(&[1])?;

    let mut data = [0 as u8; 8];
    stream.read(&mut data);
    let mut text = from_utf8(&data).unwrap();
    println!("{}", text);



    loop{
        stream.read(&mut data);
       let mut p = [data[0] as u8, data[1] as u8];
       let p_float: f32 = (LittleEndian::read_u16(&p) as f32) / 10436.0 - 3.14 ;
       let mut y = [data[2] as u8, data[3] as u8];
       let y_float: f32 = (LittleEndian::read_u16(&y) as f32) / 10436.0 - 3.14 ;
       let mut o = [data[4] as u8, data[5] as u8];
       let o_float: f32 = (LittleEndian::read_u16(&o) as f32) / 33.0 - 1000.0 ;
       let mut g = [data[6] as u8, data[7] as u8];
       let g_float: f32 = (LittleEndian::read_u16(&g) as f32) / 33.0 - 1000.0 ;

       println!("P: {}, Y: {}, O: {}, G: {}", p_float, y_float, o_float, g_float);
       //println!("{:?}", &data);
       let post = save_data(&connection, &p_float, &y_float, &o_float, &g_float);


        data = [0 as u8; 8];
    }


}