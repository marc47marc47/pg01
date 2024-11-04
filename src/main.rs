extern crate libc;

use std::ffi::CString;
use std::io::Read;
use std::os::unix::io::RawFd;

extern "C" {
    fn htons(hostshort: u16) -> u16;
    fn ntohl(netlong: u32) -> u32;
    fn inet_pton(af: libc::c_int, src: *const libc::c_char, dst: *mut libc::c_void) -> libc::c_int;
    fn inet_ntop(
        af: libc::c_int,
        src: *const libc::c_void,
        dst: *mut libc::c_char,
        size: libc::socklen_t,
    ) -> *const libc::c_char;
}

// 定義 TcpServer 結構體
struct TcpServer {
    socket_fd: RawFd,
    ip: String,
    port: u16,
}

// 定義 TcpStream 結構體
struct TcpStream {
    client_fd: RawFd,
}

impl TcpServer {
    // 建立 TCP 伺服器並綁定到指定的 IP 和端口
    fn new(ip: &str, port: u16) -> Result<Self, String> {
        unsafe {
            let socket_fd = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);
            if socket_fd < 0 {
                return Err("無法建立 socket".to_string());
            }

            // 解析 IP 地址
            let ip_addr = match Self::parse_ip(ip) {
                Some(addr) => addr,
                None => return Err("無效的 IP 地址".to_string()),
            };

            let sockaddr_in = libc::sockaddr_in {
                sin_family: libc::AF_INET as u16,
                sin_port: htons(port),
                sin_addr: ip_addr,
                sin_zero: [0; 8],
            };

            let bind_res = libc::bind(
                socket_fd,
                &sockaddr_in as *const libc::sockaddr_in as *const libc::sockaddr,
                std::mem::size_of::<libc::sockaddr_in>() as u32,
            );

            if bind_res < 0 {
                libc::close(socket_fd);
                return Err("無法綁定 socket".to_string());
            }

            let listen_res = libc::listen(socket_fd, 10);
            if listen_res < 0 {
                libc::close(socket_fd);
                return Err("無法開始監聽".to_string());
            }

            Ok(TcpServer {
                socket_fd,
                ip: ip.to_string(),
                port,
            })
        }
    }

    // 等待客戶端的連接
    fn accept(&self) -> Result<TcpStream, String> {
        unsafe {
            let mut client_addr = libc::sockaddr_in {
                sin_family: 0,
                sin_port: 0,
                sin_addr: libc::in_addr { s_addr: 0 },
                sin_zero: [0; 8],
            };
            let mut addr_len = std::mem::size_of::<libc::sockaddr_in>() as u32;

            let client_fd = libc::accept(
                self.socket_fd,
                &mut client_addr as *mut libc::sockaddr_in as *mut libc::sockaddr,
                &mut addr_len,
            );

            if client_fd < 0 {
                return Err("接受連接失敗".to_string());
            }

            Ok(TcpStream { client_fd })
        }
    }

    // 關閉伺服器 socket
    fn close(&self) {
        unsafe {
            libc::close(self.socket_fd);
        }
    }

    // 私有方法，將 IP 字串解析為 libc 的 in_addr 結構體
    fn parse_ip(ip: &str) -> Option<libc::in_addr> {
        let ip_cstr = CString::new(ip).ok()?;
        let mut addr: libc::in_addr = libc::in_addr { s_addr: 0 };
        let res = unsafe {
            inet_pton(
                libc::AF_INET,
                ip_cstr.as_ptr(),
                &mut addr as *mut _ as *mut libc::c_void,
            )
        };

        if res == 1 {
            Some(addr)
        } else {
            None
        }
    }
}

impl TcpStream {
    // 從客戶端讀取資料
    fn read(&self, buffer: &mut [u8]) -> Result<usize, String> {
        unsafe {
            let bytes_read = libc::read(
                self.client_fd,
                buffer.as_mut_ptr() as *mut libc::c_void,
                buffer.len(),
            );
            if bytes_read < 0 {
                return Err("讀取失敗".to_string());
            }
            Ok(bytes_read as usize)
        }
    }

    // 傳送資料到客戶端
    fn write(&self, buffer: &[u8]) -> Result<usize, String> {
        unsafe {
            let bytes_written = libc::write(
                self.client_fd,
                buffer.as_ptr() as *const libc::c_void,
                buffer.len(),
            );
            if bytes_written < 0 {
                return Err("寫入失敗".to_string());
            }
            Ok(bytes_written as usize)
        }
    }

    // 關閉連接
    fn close(&self) {
        unsafe {
            libc::close(self.client_fd);
        }
    }

    // 解析 PostgreSQL Startup Message
    fn parse_startup_message(&self, buffer: &[u8]) {
        if buffer.len() < 8 {
            println!("無效的啟動訊息封包");
            return;
        }

        // 解析協議版本號 (4 bytes)
        let protocol_version = unsafe { ntohl(*(buffer.as_ptr().add(4) as *const u32)) };
        if protocol_version == 196608 {
            println!("協議版本: 3.0");
        } else {
            println!("未知的協議版本: {}", protocol_version);
        }
    }

    // 解析 Simple Query 封包
    fn parse_query_message(&self, buffer: &[u8]) {
        if buffer.len() < 6 {
            println!("無效的查詢封包");
            return;
        }

        // Simple Query 封包以 'Q' 開頭
        if buffer[0] == b'Q' {
            let query = &buffer[5..]; // 'Q' 開頭的訊息，第 5 位元組之後是 SQL 字串
            let sql_query = String::from_utf8_lossy(query);
            println!("收到 SQL 查詢: {}", sql_query);
        }
    }
}

fn main() {
    // 建立伺服器
    let server = TcpServer::new("0.0.0.0", 5432).expect("伺服器建立失敗");

    println!("伺服器正在監聽 0.0.0.0:5432");

    loop {
        // 等待客戶端連接
        match server.accept() {
            Ok(stream) => {
                println!("接受來自客戶端的連接");

                let mut buffer = [0; 1024];
                match stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        // 根據第一個封包的類型進行解析
                        if bytes_read > 0 {
                            // 檢查第一個封包，解析啟動訊息或查詢
                            stream.parse_startup_message(&buffer[..bytes_read]);

                            // 等待接收查詢封包
                            match stream.read(&mut buffer) {
                                Ok(query_bytes) => {
                                    if query_bytes > 0 {
                                        stream.parse_query_message(&buffer[..query_bytes]);
                                    }
                                }
                                Err(e) => eprintln!("讀取查詢封包失敗: {}", e),
                            }
                        }
                    }
                    Err(e) => eprintln!("讀取啟動封包失敗: {}", e),
                }

                // 關閉連接
                stream.close();
            }
            Err(e) => {
                eprintln!("錯誤：{}", e);
            }
        }
    }
}
