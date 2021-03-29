//module:   tcp server example
//date:     2021/03/21
//author:   chenyouneng
//email:    bbeyondllove@gmail.com

use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str;

//限制每个消息包最大长度为512字节
const BUFFER_SIZE:usize =512; 

//客户端消息处理方法
fn handle_client(mut stream: TcpStream) {
    //定义客户端消息接收数组
    let mut msg = [0 as u8; BUFFER_SIZE];  
    //模式匹配错误处理:循环读取客户端消息并处理
    while match stream.read(&mut msg) {
        //如果接收消息正常
        Ok(size) => {
                //消息数组转换为字符串
                let buf = str::from_utf8(&msg[0..size]).unwrap();
                //打印接收到的消息
                println!("recv from client:{:?}",buf);
                //如果消息长度大于0,echo返回
                if size>0{
                    //echo返回
                    println!("send to client:{:?}",buf);
                    stream.write(&msg[0..size]).unwrap();
                    true
                }else{//如果消息长度等于0，打印错误信息，关闭连接
                    //打印错误消息
                    println!("An connection closed:{}", stream.peer_addr().unwrap());
                    //关闭连接
                    stream.shutdown(Shutdown::Both).unwrap();
                    false
                }
                             
         },
         //如果接收消息错误
        Err(_) => {
           //打印错误消息
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            //关闭连接
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    //定义要绑定的本机地址和端口
    let addr="0.0.0.0:8888";
    //在指定的addr上进行网络监听
    let listener = TcpListener::bind(addr).unwrap();
     //打印服务器监听信息
    println!("Server listening on port {}",addr);

    //接收所有客户端连接，并启动新的线程处理每个客户端连接
    for stream in listener.incoming() {
        //模式匹配错误处理
        match stream {
            //如果连接正常
            Ok(stream) => {
                //打印连接信息
                println!("New connection: {}", stream.peer_addr().unwrap());
                //启动新的线程处理新连接
                thread::spawn(move|| {
                     handle_client(stream)
                });
            }
            //如果连接出错
            Err(e) => {
                //打印错误信息
                println!("Error: {}", e);
            }
        }
    }
    //关闭服务监听
    drop(listener);
}
