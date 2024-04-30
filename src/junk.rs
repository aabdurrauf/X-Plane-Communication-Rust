
// use tokio::net::UdpSocket;
// use std::net::SocketAddr;
// use std::error::Error;
// use std::mem;
// use std::convert::TryInto;
// use std::io::Write;

// use byte::ctx::Bytes;
// use byte::LE;

// async fn connect_xplane(xp_host: &str, xp_port: u16, port: u16, timeout: u64) -> Result<(UdpSocket, SocketAddr), String> {
//     // Validate parameters
//     let xp_ip = match tokio::net::lookup_host((xp_host, xp_port)).await {
//         // 
//         Ok(mut addresses) => match addresses.next() {
//             // checking if teh addr is not null
//             Some(addr) => addr,
//             None => return Err(String::from("Unable to resolve xpHost.")),
//         },
//         Err(_) => return Err(String::from("Unable to resolve xpHost.")),
//     };

//     if timeout < 0 {
//         return Err(String::from("timeout must be non-negative."));
//     }

//     // Setup XPlane IP and port to be returned and used in other functions
//     let xp_dst = SocketAddr::new(xp_ip.ip(), xp_port);

//     // Create and bind socket
//     let client_addr = SocketAddr::new("0.0.0.0".parse().unwrap(), port);
//     let mut sock = match UdpSocket::bind(&client_addr).await {
//         Ok(socket) => socket,
//         Err(_) => return Err(String::from("Failed to bind socket.")),
//     };

//     // Set socket timeout
//     // sock.set_read_timeout(Some(std::time::Duration::from_millis(timeout))).unwrap();

//     println!("Connection established.");

//     Ok((sock, xp_dst))
// }

// async fn send_udp(sock: &UdpSocket, xp_dst: &SocketAddr, buffer: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
//     if buffer.is_empty() {
//         return Err(String::from("send_udp: buffer is empty.").into());
//     }

//     sock.send_to(buffer, xp_dst).await?;
//     Ok(())
// }

// async fn read_udp(sock: &UdpSocket) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
//     let mut buffer = vec![0; 16384];
//     let (len, _) = sock.recv_from(&mut buffer).await?;
//     buffer.truncate(len);
//     Ok(buffer)
// }



// //______________________________________________________________________________________//

// // last time we were here, this code has not been tested and directly copied from chatGPT
// // this needs more analysis and controls, as well as simplification.
// async fn get_posi(sock: &UdpSocket, xp_dst: &SocketAddr, ac: u8) -> Result<Vec<f32>, Box<dyn Error>> {
//     // Send request
//     let mut buffer: [u8; 6] = [0; 6];
//     // send the command to retrieve the position using command that is understood by the plugin in the X-Plane
//     // buffer[0] = 71;
//     // buffer[1] = 69;
//     // buffer[2] = 84;
//     // buffer[3] = 80;
//     // buffer[4] = 0;

//     // let bytes: &[u8] = &[0x47, 0x45, 0x54, 0x50, 0x00, 0x00];

//     // buffer[..4].copy_from_slice(b"GETP");
//     // buffer[5] = ac;

//     // let offset = &mut 0;
//     // let buffer = bytes.read_with::<u32>(offset, BE).unwrap();

//     // let mut buffer = Bytes::new();
//     // buffer.write("GETP".as_bytes())?;
//     // buffer.write_u8(0)?;

//     let mut buffer = Vec::new();
//     buffer.write(b"GETP").unwrap();
//     buffer.write(&[0]).unwrap();
//     buffer.write(&[0]).unwrap();
//     println!("buffer: {:?}", buffer);

//     send_udp(sock, xp_dst, &buffer);

//     // Read response
//     let result_buf = read_udp(sock).await?;

//     let result: Vec<f32> = Vec::new();

//     // println!("result: {:?}", result);
//     println!("result_buf: {}", result_buf[0]);

//     // let result = if result_buf.len() == 34 {
//     //     let (header, ac_id, lat, lon, alt, pitch, roll, yaw, vx, vy, vz) =
//     //         unsafe { mem::transmute::<[u8; 34], (u32, u8, f32, f32, f32, f32, f32, f32, f32, f32, f32)>(result_buf.try_into().unwrap()) };
//     //     (header, ac_id, lat, lon, alt, pitch, roll, yaw, vx, vy, vz)
//     // } else if result_buf.len() == 46 {
//     //     let (header, ac_id, lat, lon, alt, pitch, roll, yaw, vx, vy, vz) =
//     //         unsafe { mem::transmute::<[u8; 46], (u32, u8, f64, f64, f64, f32, f32, f32, f32, f32, f32)>(result_buf.try_into().unwrap()) };
//     //     (header, ac_id, lat as f32, lon as f32, alt as f32, pitch, roll, yaw, vx, vy, vz)
//     // } else {
//     //     return Err("Unexpected response length.".into());
//     // };

//     // if result.0 != 0x494F5350 { // ASCII for "POSI"
//     //     return Err(format!("Unexpected header: {}", result.0).into());
//     // }

//     // // Drop the header & ac from the return value
//     // Ok(vec![result.2, result.3, result.4, result.5, result.6, result.7, result.8, result.9, result.10])

//     Ok(result)
// }

// // fn test() {
// //     // Create a buffer to store the packed data
// //     let mut buffer = Vec::new();

// //     // Pack the data into the buffer
// //     buffer.write_all(b"GETP").unwrap(); // Writes the string "GETP"
// //     buffer.write_i8(0).unwrap();        // Writes a single byte with value 0

// //     // Print the packed data (for demonstration)
// //     println!("{:?}", buffer);
// // }

// #[tokio::main]
// async fn main() {

//     // test();

//     let (sock, xp_dst) = match connect_xplane("localhost", 49009, 0, 100).await {
//         Ok((sock, xp_dst)) => {
//             (sock, xp_dst)
//         },
//         Err(e) => {
//             println!("Error: {}", e);
//             return;
//         },
//     };

//     println!("the value of sock is: {:?} \nand xp_dst is: {}", sock, xp_dst);

//     get_posi(&sock, &xp_dst, 0);
//     println!("end of program");

//     // let mut buffer = Vec::new();

//     // // Pack the data into the buffer
//     // write!(buffer, "GETP 0").unwrap();

//     // // Print the packed data (for demonstration)
//     // println!("{:?}", buffer);


//     // let data = format!("GETP {}", 0);

//     // // Convert the string to bytes
//     // let bytes = data.into_bytes();

//     // // Print the bytes (for demonstration)
//     // println!("{:?}", bytes);

// }

// -----------------------------------------------------------
// fn connect_xplane(xp_host: &str, xp_port: u16, port: u16, timeout: u64) -> Result<(UdpSocket, SocketAddr), String> {
//     // Validate parameters
//     let xp_ip = match xp_host.parse() {
//         Ok(ip) => ip,
//         Err(_) => return Err("Unable to resolve xp_host.".to_string()),
//     };

//     if xp_port > 65535 {
//         return Err("The specified X-Plane port is not a valid port number.".to_string());
//     }
//     if port > 65535 {
//         return Err("The specified port is not a valid port number.".to_string());
//     }

//     // Setup XPlane IP and port
//     let xp_dst = SocketAddr::new(xp_ip, xp_port);

//     // Create and bind socket
//     let client_addr = SocketAddr::new("0.0.0.0".parse().unwrap(), port);
//     let sock = UdpSocket::bind(client_addr).map_err(|e| e.to_string())?;
    
//     sock.set_read_timeout(Some(std::time::Duration::from_millis(timeout)))
//         .map_err(|e| e.to_string())?;

//     println!("connection established.");

//     Ok((sock, xp_dst))
// }

// fn main() {
//     let (sock, xp_dst) = connect_xplane("127.0.0.1", 49009, 0, 100).unwrap();
//     // let data_to_send: &[u8] = b"GETP\x00\x00";
//     // println!("data_to_send: {:?}", data_to_send);
//     // send_udp(&sock, &xp_dst, data_to_send).expect("Failed to send UDP packet");

//     // let received_data = read_udp(&sock).expect("Failed to read UDP packet");
//     // println!("Received data: {:?}", received_data);
//     // println!("Size of the vector: {}", received_data.len());

//     // let bytes: [u8; 8] = [65, 236, 123, 132, 69, 255, 57, 64]; // 64-bit floating-point number bytes
//     // let float_value = f64::from_le_bytes(bytes);
//     // println!("Float value: {}", float_value);
// }