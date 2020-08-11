extern crate rand;

use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use std::net::{TcpStream, Shutdown};
use rand::Rng;


// 运行线程, 每个线程建立一个连接，并随机向服务器发送一次请求
fn run_thread(id: u32){
    // 连接服务端
    let mut stream: TcpStream = TcpStream::connect("127.0.0.1:9991").unwrap();

    // 获取一个随机数
    let iterations = rand::thread_rng().gen_range(5, 15);

    let mut x = 0;

    // 循环发送数据
    while x < iterations {
        // 向服务器发送数据
        match stream.write(format!("Thread {}: {}\n", id, rand::thread_rng().gen_range(10000,100000)).as_bytes()){
           Ok(_)   => { stream.flush().unwrap(); }
           Err(e)  => { println!("Error! - {}", e); }
        }

        // 随机休眠一段时间，单位毫秒
        thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(150, 2000)));

        // 累计数字
        x += 1;
    }

    // 关闭管道
    stream.shutdown(Shutdown::Both).unwrap();
    println!("Client {} sent {} requests", id, iterations);   
}

fn main(){
    // 随机数生成
    let threads = rand::thread_rng().gen_range(5, 25);

    // 生成一个空的数组
    let mut t = Vec::new();

    let mut x = 0;
    while x < threads {
        let y = x;

        // 加入线程数组
        t.push(
            thread::spawn(move || {
                run_thread(y);
            })
        );

        x += 1;
    }

    // 执行线程
    for thr in t{ thr.join().unwrap(); }

    // 结束线程
    TcpStream::connect("127.0.0.1:9991").unwrap().write(String::from("quit\n").as_bytes()).unwrap();

    // 打印客户端连接数
    println!("Spawned {} clients", threads);
}