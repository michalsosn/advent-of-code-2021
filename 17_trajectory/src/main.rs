use std::io;
use std::cmp;
use regex::Regex;


fn main() {
    let Input { area } = Input::read_stdin();
    println!("{:?}", area);

    let (max_vyi, max_vyi_t) = find_max_vyi(&area);
    let max_y = find_max_y(max_vyi, max_vyi_t as i32);
    println!("max_vyi = {}, max_vyi_t = {}, max_y = {}", max_vyi, max_vyi_t, max_y);

    let all_count = count_all(&area);
    println!("all_count = {}", all_count);
}

#[derive(Debug)]
struct Input {
    area: Rectangle
}

impl Input {
    fn read_stdin() -> Self {
        let mut line: String = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let re = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
        let captures = re.captures(&line).unwrap();

        let area = Rectangle {
            xa: captures.get(1).unwrap().as_str().parse().unwrap(),
            xb: captures.get(2).unwrap().as_str().parse().unwrap(),
            ya: captures.get(3).unwrap().as_str().parse().unwrap(),
            yb: captures.get(4).unwrap().as_str().parse().unwrap(),
        };
        Input {
            area
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    xa: i32, xb: i32, ya: i32, yb: i32,
}


fn find_max_vyi(area: &Rectangle) -> (i32, u32) {
    let inf_t_vxi = find_inf_t_vxi(area.xa, area.xb).unwrap();
    println!("{:?}", inf_t_vxi);

    let yaf = area.ya as f64;
    let ybf = area.yb as f64;
    let yspan = (area.yb - area.ya) as u32;

    let mut max_vyi: Option<i32> = None;
    let mut max_vyi_t: Option<u32> = None;
    let mut t = inf_t_vxi.abs() as u32;
    let mut fail_count: u32 = 0;
    while fail_count < yspan {
        let tf = t as f64;
        let vyi_min = 0.5 * tf - 0.5 + yaf / tf;
        let vyi_max = 0.5 * tf - 0.5 + ybf / tf;

        match find_y_in_range(vyi_min, vyi_max) {
            Some(n) => {
                max_vyi = Some(n);
                max_vyi_t = Some(t);
                fail_count = 0;
            }
            None => {
                fail_count += 1;
            }
        }
        t += 1;
    }

    (max_vyi.unwrap(), max_vyi_t.unwrap())
}

fn find_y_in_range(a: f64, b: f64) -> Option<i32> {
    let mut c = a as i32;
    loop {
        let cf = c as f64;
        if cf > b {
            return None;
        }
        if cf >= a {
            return Some(c);
        }
        c += 1;
    }
}

fn find_inf_t_vxi(xa: i32, xb: i32) -> Option<i32> {
    if xa <= 0 && 0 <= xb {
        return Some(0);
    } else if xb <= 0 {
        return find_inf_t_vxi(-xb, -xa).map(|n| -n);
    } else {
        let xa2 = 2 * xa;
        let xb2 = 2 * xb;

        let mut c: i32 = (xa2 as f64).sqrt() as i32;
        loop {
            let csq = c * (c + 1);
            if csq > xb2 {
                return None;
            }
            if csq >= xa2 {
                return Some(c);
            }
            c += 1;
        }
    }
}

fn find_max_y(vyi: i32, t: i32) -> i32 {
    if vyi <= 0 {
        0
    } else if vyi > t {
        t * (2 * vyi - t + 1) / 2
    } else {
        vyi * (vyi + 1) / 2
    }
}


fn count_all(area: &Rectangle) -> u64 {
    let ya_abs = area.ya.abs();
    let yb_abs = area.yb.abs();
    let max_y_abs = cmp::max(ya_abs, yb_abs);

    let (xa_abs, xb_abs) = if area.xb <= 0 {
        (-area.xb, -area.xa)
    } else {
        (area.xa, area.xb)
    };

    let mut count: u64 = 0;
    for vyi in area.ya..=max_y_abs {
        // println!("vyi {} out", vyi);
        for vxi in 0..=xb_abs {
            // println!("vxi {} out", vxi);
            let mut t: i32 = 0;
            let mut x: i32 = 0;
            let mut y: i32 = 0;
            loop {
                if area.ya <= y && y <= area.yb && xa_abs <= x && x <= xb_abs {
                    print!("({} {}) ", vyi, vxi);
                    count += 1;
                    break;
                }
                if (y < area.ya && t > vyi) || x > xb_abs {
                    // println!("{} {} out after {} at {} {}", vyi, vxi, t, y, x);
                    break;
                }
                y += vyi - t;
                x += cmp::max(0, vxi - t);
                t += 1;
            }
        }
    }
    count
}
