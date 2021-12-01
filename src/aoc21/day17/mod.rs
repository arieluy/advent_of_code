use std::cmp::Ordering;

struct Probe {
    x: i32,
    y: i32,
    vel_x: i32,
    vel_y: i32,
    highest_y: i32
}

impl Probe {
    fn new(_vel_x: i32, _vel_y: i32) -> Probe {
        Probe {
            x: 0,
            y: 0,
            vel_x: _vel_x,
            vel_y: _vel_y,
            highest_y: 0
        }
    }

    fn step(&mut self) {
        self.x += self.vel_x;
        self.y += self.vel_y;
        if self.y > self.highest_y {
            self.highest_y = self.y;
        }
        match self.vel_x.cmp(&0) {
            Ordering::Less => { self.vel_x += 1; },
            Ordering::Greater => { self.vel_x -= 1; },
            _ => ()
        };
        self.vel_y -= 1;
    }

    fn intersects_target(&self, x0: i32, x1: i32, y0: i32, y1: i32) -> bool {
        return x0 <= self.x && self.x <= x1 && y0 <= self.y && self.y <= y1;
    }

    fn intersects_target_num_steps(&mut self, x0: i32, x1: i32, y0: i32, y1: i32, num_steps: i32) -> bool {
        for _ in 0..num_steps {
            if self.intersects_target(x0, x1, y0, y1) {
                return true;
            }
            if self.x > x1 {
                return false;
            }
            self.step();
        }
        return false;
    }
}

pub fn day17_part1() {
    //let (x0, x1, y0, y1) = (20, 30, -10, -5);
    let (x0, x1, y0, y1) = (195, 238, -93, -67);
    const NUM_STEPS: i32 = 1000;
    const MAX_X_VEL: i32 = 1000;
    const MAX_Y_VEL: i32 = 1000;
    let mut best_y = 0;

    for x_vel in 0..MAX_X_VEL {
        for y_vel in 0..MAX_Y_VEL {
            let mut probe = Probe::new(x_vel, y_vel);
            if probe.intersects_target_num_steps(x0, x1, y0, y1, NUM_STEPS) {
                if probe.highest_y > best_y {
                    best_y = probe.highest_y;
                }
            }
        }
    }

    println!("{}", best_y);
}


pub fn day17_part2() {
    //let (x0, x1, y0, y1) = (20, 30, -10, -5);
    let (x0, x1, y0, y1) = (195, 238, -93, -67);
    const NUM_STEPS: i32 = 1000;
    const MAX_X_VEL: i32 = 1000;
    const MAX_Y_VEL: i32 = 1000;
    let mut count = 0;

    for x_vel in 0..MAX_X_VEL {
        for y_vel in -MAX_Y_VEL..MAX_Y_VEL {
            let mut probe = Probe::new(x_vel, y_vel);
            if probe.intersects_target_num_steps(x0, x1, y0, y1, NUM_STEPS) {
                count += 1;
            }
        }
    }

    println!("{}", count);
}