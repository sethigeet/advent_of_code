use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./data/inputs/13.txt")?;
    let arrival_time: u128 = input
        .lines()
        .nth(0)
        .unwrap()
        .to_string()
        .parse()
        .expect("the arrival time is not a number");
    let buses: Vec<u128> = input
        .lines()
        .nth(1)
        .unwrap()
        .to_string()
        .split(",")
        .map(|bus| if bus == "x" { "0" } else { bus })
        .map(|bus| bus.parse().expect("bus no is not a number"))
        .collect();

    let mut times: HashMap<u128, u128> = HashMap::new();
    for bus in buses.iter() {
        if *bus != 0 {
            times.insert(*bus, ((arrival_time / *bus) + 1) * *bus);
        }
    }

    let (bus, departure_time) = times.iter().min_by_key(|(_, time)| *time).unwrap();
    println!("Part 1 -> {}", (*departure_time - arrival_time) * *bus);

    // let smallest_bus = *buses.iter().filter(|bus| *bus != &0).min().unwrap();
    // let mut i: u128 = 1;
    // loop {
    //     let time = smallest_bus * i;

    //     let mut is_ans = true;
    //     for (j, bus) in buses.iter().enumerate() {
    //         if *bus == 0 || *bus == smallest_bus {
    //             continue;
    //         }

    //         let req_j: u128 = j.try_into().unwrap();
    //         if (time + req_j) % *bus != 0 {
    //             is_ans = false;
    //             break;
    //         }
    //     }

    //     if is_ans {
    //         println!("Part 2 -> {}", time);
    //         break;
    //     }

    //     i += 1;
    // }

    let mut time: u128 = 0;
    let mut common_factor: u128 = 1;
    for (id, bus) in buses.iter().enumerate() {
        if *bus == 0 {
            continue;
        }

        let req_id: u128 = id.try_into().unwrap();
        while (time + req_id) % *bus != 0 {
            time += common_factor;
        }

        common_factor *= *bus;
    }
    println!("Part 2 -> {}", time);

    Ok(())
}
