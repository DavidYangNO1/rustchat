extern crate futures;
extern crate async_std;

use async_std::{
    io::{stdin,BufReader},
    net::{TcpStream,ToSocketAddrs},
    prelude::*,
    task,
};
use futures::{select,FutureExt};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() {
    println!("client, begin!");
    run();
    println!("client, finish");
}

fn run() -> Result<()>{
    task::block_on(try_run("127.0.0.1:8080"))
}

async fn try_run(addr: impl ToSocketAddrs) -> Result<()>{
    
    let stream =  TcpStream::connect(addr).await?;
    
    let (reader,mut write) = (&stream,&stream);
    
    let mut lines_from_server = BufReader::new(reader).lines().fuse();
    
    let mut lines_from_stdin = BufReader::new(stdin()).lines().fuse();

    loop {
        select !{
            line = lines_from_server.next().fuse() => match line {
                Some(line) => {
                   let line = line?;
                   println!("{}",line);
                },
                None => break,
            },
            line = lines_from_stdin.next().fuse() => match line {
                Some(line) => {
                    let line = line?;
                    write.write_all(line.as_bytes()).await?;
                    write.write_all(b"\n").await?;
                },
                None => break,
            }
        }
    }

    Ok(())
}
