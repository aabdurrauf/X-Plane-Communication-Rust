use xplane_rust::xpc::{Position, Control, connect_xplane, get_posi, send_posi, get_ctrl, send_ctrl, get_dref, clear_buffer};
use xplane_rust::pid::{PIDpitch, PIDroll, set_rocket_altitude, launch_rocket_pid, land_rocket_pid, throttle_up, throttle_down};
use std::time::Duration;

fn main() {
    // let (sock, xp_dst) = connect_xplane("192.168.1.174", 49009, 0, 1000).unwrap();
    let (sock, xp_dst) = connect_xplane("127.0.0.1", 49009, 0, 1000).unwrap();

    let pid_pitch = PIDpitch{
        p: 0.06,
        i: 0.0001,
        d: 0.0024,
    };

    let pid_roll = PIDroll{
        p: 0.04,
        i: 0.0001,
        d: 0.004,
    };

    // autonomous launch and landing
    launch_rocket_pid(&sock, &xp_dst, &pid_pitch, &pid_roll, 200.0);
    let _ = land_rocket_pid(&sock, &xp_dst, &pid_pitch, &pid_roll, 190.0);

    // autonomous landing from specified altitude
    // let _ = set_rocket_altitude(&sock, &xp_dst, 1000.0);
    // let _ = land_rocket_pid(&sock, &xp_dst, &pid_pitch, &pid_roll, 670.0);



    // throttle_up(&sock, &xp_dst);
    // std::thread::sleep(Duration::from_secs(7));
    // throttle_down(&sock, &xp_dst);

    // let mut vel: f32 = 0.0;
    // for i in 1..1000{
    //     vel = get_dref(&sock, &xp_dst, b"sim/flightmodel/position/vh_ind");
    //     println!("the vertical velocity is {}", vel);
    //     clear_buffer(&sock);
    //     // std::thread::sleep(Duration::from_secs(1));
    // }
}
