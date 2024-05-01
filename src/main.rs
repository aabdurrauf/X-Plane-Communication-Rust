use xplane_rust::xpc::{Position, Control, connect_xplane, get_posi, send_posi, get_ctrl, send_ctrl, get_dref, clear_buffer};
use xplane_rust::pid::{PIDpitch, PIDroll, control_pid, throttle_up, throttle_down};
use std::time::Duration;

fn main() {
    let (sock, xp_dst) = connect_xplane("192.168.1.174", 49009, 0, 1000).unwrap();
    // let (sock, xp_dst) = connect_xplane("127.0.0.1", 49009, 0, 1000).unwrap();

    let pid_pitch = PIDpitch{
        p: 0.042,
        i: 0.0001,
        d: 0.004,
    };

    let pid_roll = PIDroll{
        p: 0.04,
        i: 0.0001,
        d: 0.004,
    };

    let _ = control_pid(&sock, &xp_dst, &pid_pitch, &pid_roll);

    // throttle_up(&sock, &xp_dst);
    
    // let control_values = Control {
    //     pit_s: -998.0,
    //     rol_s: -998.0,
    //     rud_s: -998.0,
    //     thr_s: 1.0,
    //     gr_s: 0,
    //     fl_s: -998.0,
    //     spd_brk: -998.0,
    // };

    // let _ = send_ctrl(&sock, &xp_dst, &control_values);

    // throttle_up(&sock, &xp_dst);
    // std::thread::sleep(Duration::from_secs(7));
    // throttle_down(&sock, &xp_dst);


    // std::thread::sleep(Duration::from_secs(5));

    // let mut vel: f32 = 0.0;

    // for i in 1..1000{
    //     vel = get_dref(&sock, &xp_dst, b"sim/flightmodel/position/vh_ind");
    //     println!("the vertical velocity is {}", vel);
    //     clear_buffer(&sock);
    //     // std::thread::sleep(Duration::from_secs(1));
    // }

    // let mut i: u32 = 0;
    // while i < 1{
    // let position = get_posi(&sock, &xp_dst).expect("Failed to get position");
    //     println!("the altitude of the aircraft is {}", position.alt);

    //     i += 1;
    // }
    
    // let position_values = Position {
    //     lat: -998.0,
    //     lng: -998.0,
    //     alt:  200.0,
    //     pit:   50.0,
    //     rol: -998.0,
    //     trh: -998.0,
    //     gr: -998.0,
    // };

    // let _ = send_posi(&sock, &xp_dst, &position_values);
}


/*
so what we have done here?
1. all we need to get from and send to the xplane have been implemented
2. now we have to write the scenario how we want to control our rocket

TODO:
use PID controller

*/