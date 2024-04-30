use xplane_rust::{Position, Control, connect_xplane, get_posi, send_posi, get_ctrl, send_ctrl};
use std::time::Duration;

fn main() {
    let (sock, xp_dst) = connect_xplane("127.0.0.1", 49009, 0, 100).unwrap();

    let mut i: u32 = 0;
    while i < 1{
    let position = get_posi(&sock, &xp_dst).expect("Failed to get position");
        println!("the altitude of the aircraft is {}", position.alt);

        i += 1;
    }
    
    let position_values = Position {
        lat: -998.0,
        lng: -998.0,
        alt:  200.0,
        pit:   50.0,
        rol: -998.0,
        trh: -998.0,
        gr: -998.0,
    };

    let _ = send_posi(&sock, &xp_dst, &position_values);


    let control_values = Control {
        pit_s: -998.0,
        rol_s: 0.5,
        rud_s: -998.0,
        thr_s: 1.0,
        gr_s: 1,
        fl_s: -998.0,
        spd_brk: -998.0,
    };

    let _ = send_ctrl(&sock, &xp_dst, &control_values);

    let control = get_ctrl(&sock, &xp_dst).expect("Failed to get control");
    println!("The pittch and throttle sticks are: {} - {} ", control.pit_s, control.thr_s);
}


// so what we have done here?
// 1. dont use tokio first, use the standart library from RUST
// 2. we have successfully sent command to X-Plane using: b"GETP\x00\x00";
//    so now we have to make the function get_posi
// 3. we also receive data as a type vector containing the position
//    but we cannot unpack that data automaticallt so we have to do it manually.
//    we can mannually extract it  using the f64::from_le_bytes(bytes);
// 4. remember that  the data received from the X-Plane has this following format:
//    an array of size: 46 which the first 6 are POSI {number of airplane} and then
//    following the values we are interested in [8, 8, 8, 4, 4, 4, 4] or 
//    [d, d, d, f, f, f, f] where d is double and f is float

//    DONE

/* 
so next implement the getCNTRL and sendCNTRL.
if we have time implement the get and set DREF.
then build the system.
*/
