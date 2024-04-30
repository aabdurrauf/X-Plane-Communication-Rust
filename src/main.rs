use xplane_rust::{Position, Control, connect_xplane, get_posi, send_posi, get_ctrl, send_ctrl, get_dref};
use std::time::Duration;

fn main() {
    let (sock, xp_dst) = connect_xplane("127.0.0.1", 49009, 0, 100).unwrap();

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

    std::thread::sleep(Duration::from_secs(5));

    let mut vel: f32 = 0.0;

    for i in 1..1000{
        vel = get_dref(&sock, &xp_dst, b"sim/flightmodel/position/vh_ind").expect("Failed to get the DREF");
        println!("the vertical velocity is {}", vel);
        
        std::thread::sleep(Duration::from_secs(1));
    }


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



    // let control = get_ctrl(&sock, &xp_dst).expect("Failed to get control");
    // println!("The pittch and throttle sticks are: {} - {} ", control.pit_s, control.thr_s);
}


/*
so what we have done here?
1. all we need to get from and send to the xplane have been implemented
2. now we have to write the scenario how we want to control our rocket

TODO:
use PID controller

*/