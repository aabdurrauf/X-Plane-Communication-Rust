pub mod xpc{

    use std::net::{SocketAddr, UdpSocket};
    use std::io;

    pub struct Position {
        pub lat: f64,   // latitude
        pub lng: f64,   // longitude
        pub alt: f64,   // altitude
        pub pit: f32,   // pitch
        pub rol: f32,   // roll
        pub trh: f32,   // true heading
        pub gr: f32,    // gear
    }

    pub struct Control {
        pub pit_s: f32, // pitch stick
        pub rol_s: f32, // roll stick
        pub rud_s: f32, // rudder stick
        pub thr_s: f32, // throttle stick
        pub gr_s: i8,   // gear on/off
        pub fl_s: f32,  // flaps
        pub spd_brk: f32, // speed breaks
    }

    pub fn connect_xplane(xp_host: &str, xp_port: u16, port: u16, timeout: u64) -> Result<(UdpSocket, SocketAddr), String> {
        let xp_ip = match xp_host.parse() {
            Ok(ip) => ip,
            Err(_) => return Err("Invalid IP address!".to_string()),
        };

        let xp_dst = SocketAddr::new(xp_ip, xp_port);
        let client_addr = SocketAddr::new("0.0.0.0".parse().unwrap(), port);
        let sock = UdpSocket::bind(client_addr).map_err(|e| e.to_string())?;
        sock.set_read_timeout(Some(std::time::Duration::from_millis(timeout))).map_err(|e| e.to_string())?;

        println!("Connected to X-Plane");

        Ok((sock, xp_dst))
    }

    fn send_udp(sock: &UdpSocket, xp_dst: &std::net::SocketAddr, buffer: &[u8]) -> Result<(), io::Error> {
        if buffer.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "send_udp: buffer is empty."));
        }

        sock.send_to(buffer, *xp_dst)?;
        Ok(())
    }

    fn read_udp(sock: &UdpSocket) -> Result<Vec<u8>, String> {
        let mut buffer = vec![0; 16384];
        let (size, _) = match sock.recv_from(&mut buffer) {
            Ok(n) => n,
            Err(_e) => return Err("Did not received any data".to_string()),
        };
        buffer.truncate(size);
        Ok(buffer)
    }

    pub fn get_posi(sock: &UdpSocket, xp_dst: &SocketAddr) -> Result<Position, io::Error> {
        // function to get the position of the aircraft
        let mut pos = Position{lat:0.0, lng:0.0, alt:0.0, pit:0.0, rol:0.0, trh:0.0, gr:0.0, };

        // write the request message
        let request: &[u8] = b"GETP\x00\x00";

        send_udp(&sock, &xp_dst, request).expect("Failed to send UDP packet");

        let received_data = match read_udp(&sock) {
            Ok(x) => x,
            Err(_e) => vec![0; 46],
        };

        // println!("Received data: {:?}", received_data);
        // println!("Size of the vector: {}", received_data.len());

        // now decode the received data to a struct
        // the first 6 bytes are: "POSI 1"

        // start with the first 3 double values (64-bit)
        let mut bytes64: [u8; 8] = received_data[6..14].try_into().expect("slice has incorrect length");
        let mut float_value64 = f64::from_le_bytes(bytes64);
        pos.lat = float_value64;

        bytes64 = received_data[14..22].try_into().expect("slice has incorrect length");
        float_value64 = f64::from_le_bytes(bytes64);
        pos.lng = float_value64;

        bytes64 = received_data[22..30].try_into().expect("slice has incorrect length");
        float_value64 = f64::from_le_bytes(bytes64);
        pos.alt = float_value64;

        // the remaining 4 values are in type float (32-bit)
        let mut bytes32: [u8; 4] = received_data[30..34].try_into().expect("slice has incorrect length");
        let mut float_value32 = f32::from_le_bytes(bytes32);
        pos.pit = float_value32;

        bytes32 = received_data[34..38].try_into().expect("slice has incorrect length");
        float_value32 = f32::from_le_bytes(bytes32);
        pos.rol = float_value32;

        bytes32 = received_data[38..42].try_into().expect("slice has incorrect length");
        float_value32 = f32::from_le_bytes(bytes32);
        pos.trh = float_value32;

        bytes32 = received_data[42..46].try_into().expect("slice has incorrect length");
        float_value32 = f32::from_le_bytes(bytes32);
        pos.gr = float_value32;

        Ok(pos)
    }

    pub fn send_posi(sock: &UdpSocket, xp_dst: &SocketAddr, values: &Position) -> Result<(), io::Error> {
        // set the header of the datagram to "POSI 0"
        let header: &[u8] = b"POSI\x00\x00";

        // convert each field of the Position struct to little-endian bytes
        let lat_bytes = values.lat.to_le_bytes();
        let lng_bytes = values.lng.to_le_bytes();
        let alt_bytes = values.alt.to_le_bytes();
        let pit_bytes = values.pit.to_le_bytes();
        let rol_bytes = values.rol.to_le_bytes();
        let trh_bytes = values.trh.to_le_bytes();
        let gr_bytes = values.gr.to_le_bytes();

        let request: Vec<u8> = [header, &lat_bytes, &lng_bytes, &alt_bytes, &pit_bytes, &rol_bytes, &trh_bytes, &gr_bytes].concat();
        // println!("request: {:?}", request);

        send_udp(&sock, &xp_dst, &request).expect("Failed to send UDP packet");

        Ok(())
    }

    pub fn get_ctrl(sock: &UdpSocket, xp_dst: &SocketAddr) -> Result<Control, io::Error> {
        // function to get the position of the aircraft
        let mut ctrl = Control{pit_s:0.0, rol_s:0.0, rud_s:0.0, thr_s:0.0, gr_s:0, fl_s:0.0, spd_brk: 0.0};

        // write the request message
        let request: &[u8] = b"GETC\x00\x00";

        send_udp(&sock, &xp_dst, request).expect("Failed to send UDP packet");

        let received_data = match read_udp(&sock) {
            Ok(x) => x,
            Err(_e) => vec![0; 27],
        };


        // println!("Received data: {:?}", received_data);
        // println!("Size of the vector: {}", received_data.len());

        // now decode the received data to a struct
        // the first 6 bytes are: "CTRL 1"

        // now extract values are in type float (32-bit)
        let mut bytes32: [u8; 4] = received_data[6..10].try_into().expect("slice has incorrect length");
        let mut float_value32 = f32::from_le_bytes(bytes32);
        ctrl.pit_s = float_value32;

        bytes32 = received_data[10..14].try_into().expect("slice has incorrect length");
        float_value32 = f32::from_le_bytes(bytes32);
        ctrl.rol_s = float_value32;

        bytes32 = received_data[14..18].try_into().expect("slice has incorrect length");
        float_value32 = f32::from_le_bytes(bytes32);
        ctrl.rud_s = float_value32;

        bytes32 = received_data[18..22].try_into().expect("slice has incorrect length");
        float_value32 = f32::from_le_bytes(bytes32);
        ctrl.thr_s = float_value32;

        let bytes8 = received_data[22..23].try_into().expect("slice has incorrect length");
        let int_value8 = i8::from_le_bytes(bytes8);
        ctrl.gr_s = int_value8;

        bytes32 = received_data[23..27].try_into().expect("slice has incorrect length");
        float_value32 = f32::from_le_bytes(bytes32);
        ctrl.fl_s = float_value32;

        Ok(ctrl)
    }

    pub fn send_ctrl(sock: &UdpSocket, xp_dst: &SocketAddr, values: &Control) -> Result<(), io::Error> {
        // set the header of the datagram to "CTRL 0"
        let header: &[u8] = b"CTRL\x00";

        // bytes pattern: 4sxffffbfBf

        // convert each field of the Position struct to little-endian bytes
        let pits_bytes = values.pit_s.to_le_bytes();
        let rols_bytes = values.rol_s.to_le_bytes();
        let ruds_bytes = values.rud_s.to_le_bytes();
        let thrs_bytes = values.thr_s.to_le_bytes();
        let grs_bytes = values.gr_s.to_le_bytes();
        let fls_bytes = values.fl_s.to_le_bytes();
        let aircraft: &[u8] = b"\x00";
        let spdb_bytes = values.spd_brk.to_le_bytes();

        let request: Vec<u8> = [header, &pits_bytes, &rols_bytes, &ruds_bytes, &thrs_bytes, &grs_bytes, &fls_bytes, &aircraft, &spdb_bytes].concat();
        // println!("request: {:?}", request);

        send_udp(&sock, &xp_dst, &request).expect("Failed to send UDP packet");

        Ok(())
    }

    pub fn get_dref(sock: &UdpSocket, xp_dst: &SocketAddr, dref: &[u8]) -> Result<f32, io::Error> {
        // prepare the header datagram (only one dref at a time can be requested)
        let header: &[u8] = b"GETD\x00\x01";
        let dref_len: &[u8] = &dref.len().to_le_bytes();

        let request: Vec<u8> = [header, &[dref_len[0]], &dref].concat();
        // println!("request: {:?}", request);

        send_udp(&sock, &xp_dst, &request).expect("Failed to send UDP packet");

        // let received_data = read_udp(&sock).expect("Failed to read UDP packet");
        // let length: u8 = received_data[6..7][0];
        
        let received_data = match read_udp(&sock) {
            Ok(value) => value,
            Err(_e) => vec![0; 11],
        };

        let value = match received_data.len() {
            x if x == 11 => f32::from_le_bytes(received_data[7..].try_into().expect("slice has incorrect length")),
            _ => 0.0,
        };

        Ok(value)
    }

    pub fn clear_buffer(sock: &UdpSocket) -> Result<(), io::Error> {
        // to clear the buffer before asking a new data from the x-plane

        let mut buffer = vec![0; 16384];
        match sock.recv_from(&mut buffer) {
            Ok(_data_in_buffer) => {
                Ok(())
            },
            Err(_e) => Ok(()),
        }
    }
}


pub mod pid{
    
    use std::net::{SocketAddr, UdpSocket};
    use std::io;
    use std::time::Duration;
    use crate::xpc::{Position, Control, get_posi, send_posi, send_ctrl, get_dref, clear_buffer};

    pub struct PIDpitch{
        pub p: f32,
        pub i: f32,
        pub d: f32,
    }

    pub struct PIDroll{
        pub p: f32,
        pub i: f32,
        pub d: f32,
    }

    pub fn set_rocket_altitude(sock: &UdpSocket, xp_dst: &SocketAddr, altitude: f64) -> Result<(), io::Error> {
        let pos = Position{
            lat: -998.0,
            lng: -998.0,
            alt: altitude,
            pit: -998.0,
            rol: -998.0,
            trh: -998.0,
            gr: -998.0,
        };

        let _ = send_posi(&sock, &xp_dst, &pos);

        Ok(())
    }

    pub fn launch_rocket_pid(sock: &UdpSocket, xp_dst: &SocketAddr, pid_pitch: &PIDpitch, pid_roll: &PIDroll, max_altitude: f64) -> Result<(), io::Error>  {
        let mut elevator: f32;
        let mut aileron: f32;
        let mut velocity: f32;
        
        throttle_up(&sock, &xp_dst);

        let mut pitch = match get_dref(&sock, &xp_dst, b"sim/flightmodel/position/theta") {
            Ok(value) => value,
            Err(_e) => 0.0,
        };
        let mut pitch_rate = match get_dref(&sock, &xp_dst, b"sim/flightmodel/position/Q") {
            Ok(value) => value,
            Err(_e) => 0.0,
        };
        let mut roll = match get_dref(&sock, &xp_dst, b"sim/flightmodel/position/phi") {
            Ok(value) => value,
            Err(_e) => 0.0,
        };
        let mut roll_rate = match get_dref(&sock, &xp_dst, b"sim/flightmodel/position/P") {
            Ok(value) => value,
            Err(_e) => 0.0,
        };
        let _ = clear_buffer(&sock);
        let mut position: Position = get_posi(&sock, &xp_dst).expect("Failed to get position");
        
        let mut pitch_integral: f32 = pitch;
        let mut roll_integral: f32 = roll;

        // rocket launching
        while position.alt < max_altitude {
            position = get_posi(&sock, &xp_dst).expect("Failed to get position");
            // std::thread::sleep(Duration::from_secs(1));
            // println!("altitude: {}", position.alt);

            elevator = match -pitch * pid_pitch.p + pitch_rate * pid_pitch.d + pitch_integral * pid_pitch.i {
                x if x > 1.0 => 1.0,
                x if x < -1.0 => -1.0,
                x => x
            };

            aileron = match -roll * pid_roll.p + roll_rate * pid_roll.d + roll_integral * pid_roll.i {
                x if x > 1.0 => 1.0,
                x if x < -1.0 => -1.0,
                x => x
            };

            pitch = match get_dref(&sock, &xp_dst, b"sim/flightmodel/position/theta") {
                Ok(value) => value,
                Err(_e) => pitch,
            };
            pitch_rate = match get_dref(&sock, &xp_dst, b"sim/flightmodel/position/Q") {
                Ok(value) => value,
                Err(_e) => pitch_rate,
            };
            roll = match get_dref(&sock, &xp_dst, b"sim/flightmodel/position/phi") {
                Ok(value) => value,
                Err(_e) => roll,
            };
            roll_rate = match get_dref(&sock, &xp_dst, b"sim/flightmodel/position/P") {
                Ok(value) => value,
                Err(_e) => roll_rate,
            };
            

            pitch_integral -= pitch;
            roll_integral -= roll;

            // println!("elevator: {}", elevator);
            // println!("aileron: {}", aileron);

            let control_values = Control {
                pit_s: elevator,
                rol_s: aileron,
                rud_s: -998.0,
                thr_s: -998.0,
                gr_s: 0,
                fl_s: -998.0,
                spd_brk: -998.0,
            };
            let _ = send_ctrl(&sock, &xp_dst, &control_values);
        }

        throttle_down(&sock, &xp_dst);

        Ok(())
    }

    pub fn land_rocket_pid(sock: &UdpSocket, xp_dst: &SocketAddr, pid_pitch: &PIDpitch, pid_roll: &PIDroll, alt_ignition: f64) -> Result<(), io::Error> {

        let mut elevator: f32;
        let mut aileron: f32;
        // let mut velocity: f32 = 0.0;

        let mut pitch = match get_dref(&sock, &xp_dst, b"sim/flightmodel/position/theta") {
            Ok(value) => value,
            Err(_e) => 0.0,
        };
        let mut pitch_rate = match get_dref(&sock, &xp_dst, b"sim/flightmodel/position/Q") {
            Ok(value) => value,
            Err(_e) => 0.0,
        };
        let mut roll = match get_dref(&sock, &xp_dst, b"sim/flightmodel/position/phi") {
            Ok(value) => value,
            Err(_e) => 0.0,
        };
        let mut roll_rate = match get_dref(&sock, &xp_dst, b"sim/flightmodel/position/P") {
            Ok(value) => value,
            Err(_e) => 0.0,
        };
        let _ = clear_buffer(&sock);

        let mut position: Position = get_posi(&sock, &xp_dst).expect("Failed to get position");
        let mut pitch_integral: f32 = pitch;
        let mut roll_integral: f32 = roll;
        let mut is_on_ground = match get_dref(&sock, &xp_dst, b"sim/flightmodel/failures/onground_any") {
            Ok(value) => value,
            Err(_e) => 0.0,
        };
        let mut crash = match get_dref(&sock, &xp_dst, b"sim/flightmodel2/misc/has_crashed") {
            Ok(value) => value,
            Err(_e) => 0.0,
        };

        // let mut i: i16 = 0;
        loop {
            // i += 1;

            elevator = match -pitch * pid_pitch.p + pitch_rate * pid_pitch.d + pitch_integral * pid_pitch.i {
                x if x > 1.0 => 1.0,
                x if x < -1.0 => -1.0,
                x => x
            };

            aileron = match -roll * pid_roll.p + roll_rate * pid_roll.d + roll_integral * pid_roll.i {
                x if x > 1.0 => 1.0,
                x if x < -1.0 => -1.0,
                x => x
            };

            pitch = match get_dref(&sock, &xp_dst, b"sim/flightmodel/position/theta") {
                Ok(value) => value,
                Err(_e) => pitch,
            };
            pitch_rate = match get_dref(&sock, &xp_dst, b"sim/flightmodel/position/Q") {
                Ok(value) => value,
                Err(_e) => pitch_rate,
            };
            roll = match get_dref(&sock, &xp_dst, b"sim/flightmodel/position/phi") {
                Ok(value) => value,
                Err(_e) => roll,
            };
            roll_rate = match get_dref(&sock, &xp_dst, b"sim/flightmodel/position/P") {
                Ok(value) => value,
                Err(_e) => roll_rate,
            };


            pitch_integral -= pitch;
            roll_integral -= roll;

            // println!("{} elevator: {}", i, elevator);
            // println!("{} aileron: {}", i, aileron);

            let control_values = Control {
                pit_s: elevator,
                rol_s: aileron,
                rud_s: -998.0,
                thr_s: -998.0,
                gr_s: 1,
                fl_s: -998.0,
                spd_brk: -998.0,
            };
            let _ = send_ctrl(&sock, &xp_dst, &control_values);

            // velocity = match get_dref(&sock, &xp_dst, b"sim/flightmodel/position/vh_ind") {
            //     Ok(value) => value,
            //     Err(e) => velocity,
            // };

            if position.alt <= alt_ignition && position.alt > 20.0 {
                throttle_up(&sock, &xp_dst);
            }

            // else if position.alt <= 20.0 && velocity >= -10.0 {
            //     throttle_down(&sock, &xp_dst);
            // }

            position = get_posi(&sock, &xp_dst).expect("Failed to get position");

            // open the gear (landing legs)
            // if position.alt <= 50.0 {

            //     let control_values = Control {
            //         pit_s: -998.0,
            //         rol_s: -998.0,
            //         rud_s: -998.0,
            //         thr_s: -998.0,
            //         gr_s: 0,
            //         fl_s: -998.0,
            //         spd_brk: -998.0,
            //     };
            //     let _ = send_ctrl(&sock, &xp_dst, &control_values);
            // }

            is_on_ground = match get_dref(&sock, &xp_dst, b"sim/flightmodel/failures/onground_any") {
                Ok(value) => value,
                Err(_e) => is_on_ground,
            };

            crash = match get_dref(&sock, &xp_dst, b"sim/flightmodel2/misc/has_crashed") {
                Ok(value) => value,
                Err(_e) => crash,
            };
            // println!("crash: {}", crash);

            if is_on_ground > 0.0 || crash > 0.0 {
                throttle_down(&sock, &xp_dst);
                break;
            }
        }
        Ok(())
    }

    pub fn throttle_up(sock: &UdpSocket, xp_dst: &SocketAddr) {
        let control_values = Control {
            pit_s: -998.0,
            rol_s: -998.0,
            rud_s: -998.0,
            thr_s: 1.0,
            gr_s: 0,
            fl_s: -998.0,
            spd_brk: -998.0,
        };
        let _ = send_ctrl(&sock, &xp_dst, &control_values);
    }

    pub fn throttle_down(sock: &UdpSocket, xp_dst: &SocketAddr) {
        let control_values = Control {
            pit_s: -998.0,
            rol_s: -998.0,
            rud_s: -998.0,
            thr_s: 0.0,
            gr_s: 0,
            fl_s: -998.0,
            spd_brk: -998.0,
        };
        let _ = send_ctrl(&sock, &xp_dst, &control_values);
    }

    // pub gather_data() -> Result<(Vec<f32>, Vec<f32>, ... ), io::Error>  
}