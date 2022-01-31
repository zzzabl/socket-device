use std::env;
//use std::io::{BufRead, Write};
use tokio::net::{TcpStream, TcpListener};
use tokio::io::{BufWriter, BufReader, WriteHalf, AsyncBufReadExt, AsyncWriteExt};


static mut IS_ON: bool = false;
static POWER: f32 = 123.;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let listener = TcpListener::bind(format!("{}:{}", args[1], args[2])).await.unwrap();
    println!("Слушаем: {} {}", args[1], args[2]);
    loop {
        let stream_result = listener.accept().await;
        if let Ok((stream,addr)) = stream_result {
            println!("Новое соединение: {}", addr);
            handle_connection(stream).await.unwrap();
        }
    }
}

async fn handle_connection(stream: TcpStream) -> Result<(), String> {

    //let mut (read_buf = BufReader::new(stream.into_split());
    let (mut read, mut write) = tokio::io::split(stream);

    let mut write_buf = BufWriter::new(write);
    let mut read_buf = BufReader::new(read);


    let mut command = String::new();
    read_buf.read_line(&mut command).await
        .map_err(|err| err.to_string())?;
    command = command.replace("\n", "");
    println!("command: {}", command);
    match command.as_ref() {
        "switch" => do_switch(&mut write_buf).await,
        "getValue" => get_value(&mut write_buf).await,
        "getState" => get_state(&mut write_buf).await,
        _ => Err(String::from("Неизвестная команда")),
    }
}

async fn do_switch(writer: &mut BufWriter<WriteHalf<TcpStream>>) -> Result<(), String> {
    unsafe {
        IS_ON = !IS_ON;
        writer.write(IS_ON.to_string().as_bytes()).await
            .map_err(|e| e.to_string())?;
        println!("do_switch: return {}", IS_ON);
    };
    Ok(())
}

async fn get_state(writer: &mut BufWriter<WriteHalf<TcpStream>>) -> Result<(), String> {
    unsafe {
        let buf = [IS_ON as u8];
        writer.write(&buf).await.map_err(|e| e.to_string())?;
        println!("get_state: return {}", IS_ON);
    };
    Ok(())
}

async fn get_value(writer: &mut BufWriter<WriteHalf<TcpStream>>) -> Result<(), String> {
    let buf = POWER.to_be_bytes();
    writer.write(&buf).await.map_err(|e| e.to_string())?;
    writer.flush().await.map_err(|e| e.to_string())?;
    println!("get_value: return {}", POWER);
    Ok(())
}
