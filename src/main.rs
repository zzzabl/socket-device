use std::env;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};

static mut IS_ON: bool = false;
static POWER: f32 = 123.;

fn main() {
    let args: Vec<String> = env::args().collect();
    let listener = TcpListener::bind(format!("{}:{}", args[1], args[2])).unwrap();
    println!("Слушаем: {} {}", args[1], args[2]);
    for stream in listener.incoming().flatten() {
        //if let Ok(stream) = stream_result {
        println!("Новое соединение: {}", stream.peer_addr().unwrap());
        handle_connection(stream).unwrap();
        //}
    }
}

fn handle_connection(stream: TcpStream) -> Result<(), String> {
    let mut read_buf = BufReader::new(stream.try_clone().unwrap());
    let mut write_buf = BufWriter::new(stream);
    let mut command = String::new();
    read_buf
        .read_line(&mut command)
        .map_err(|err| err.to_string())?;
    command = command.replace("\n", "");
    println!("command: {}", command);
    match command.as_ref() {
        "switch" => do_switch(&mut write_buf),
        "getValue" => get_value(&mut write_buf),
        "getState" => get_state(&mut write_buf),
        _ => Err(String::from("Неизвестная команда")),
    }
}

fn do_switch(writer: &mut BufWriter<TcpStream>) -> Result<(), String> {
    unsafe {
        IS_ON = !IS_ON;
        writer
            .write(IS_ON.to_string().as_bytes())
            .map_err(|e| e.to_string())?;
    };
    Ok(())
}

fn get_state(writer: &mut BufWriter<TcpStream>) -> Result<(), String> {
    unsafe {
        let buf = [IS_ON as u8];
        writer.write(&buf).map_err(|e| e.to_string())?;
    };
    Ok(())
}

fn get_value(writer: &mut BufWriter<TcpStream>) -> Result<(), String> {
    let buf = POWER.to_be_bytes();
    writer.write(&buf).map_err(|e| e.to_string())?;
    writer.flush().map_err(|e| e.to_string())?;
    Ok(())
}
