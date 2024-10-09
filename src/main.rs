use plotly::common::{Marker, Mode};
use plotly::layout::{Axis, Layout};
use plotly::{Plot, Scatter3D};
use std::{fs, io, io::Write, process};

//handles user input for f64 numbers
fn userinputf64() -> f64 {
    loop {
        let mut string: String = String::new();

        io::stdin()
            .read_line(&mut string)
            .expect("Failed to read line");

        //break out of loop of the input can be parsed into an f64
        match string.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Enter a valid number"),
        }
    }
}

//handles user input for i32 numbers
fn userinputi32() -> i32 {
    loop {
        let mut string: String = String::new();

        io::stdin()
            .read_line(&mut string)
            .expect("Failed to read line");

        //break out of loop of the input can be parsed into an i32
        match string.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Enter a valid number"),
        }
    }
}

fn main() {
    let mut cd: Option<f64> = None;
    let mut v: Option<f64> = None;
    let mut p: Option<f64> = None;
    let mut a: Option<f64> = None;
    let mut m: Option<f64> = None;
    let mut selected = false;

    loop {
        println!("1. New blaster\n2. Load blaster\n3. Test\n4. Exit\n: ");
        let mode = userinputi32();

        if mode == 1 {
            println!("1. Manually Enter muzzle velocity\n2. Calculate muzzle velocity");
            let mv_mode = userinputi32();

            if mv_mode == 1 {
                println!("Enter the mass of the dart (grams): ");
                m = Some(userinputf64() / 1000.0);
                println!("Enter the muzzle velocity (feet per second): ");
                v = Some(userinputf64() / 3.281);
            } else {
                //spring constant = absolute value of (spring force / (compressed length - relaxed length) * 0.0254 (to convert to meters))
                println!("Enter the Force of the Spring (kilograms): ");
                let f = userinputf64() * 9.81;
                println!("Enter the relaxed length of the spring (inches): ");
                let d_r = userinputf64();
                println!("Enter the compressed length of the spring (inches): ");
                let d_c = userinputf64();
                let x = (d_c - d_r) * 0.0254;
                let k = (f / x).abs();

                //potential energy = 0.5(spring constant * (relaxed length - compressed length) ^ 2))
                let pe = 0.5 * (k * (x * x));

                println!("Enter the mass of the dart (grams): ");
                m = Some(userinputf64() / 1000.0);
                println!("Enter the efficency of the blaster (0 to 1) (0.20 to 0.40 is normal)");
                let n = userinputf64();
                //kinetic energy = potential energy * efficency
                let ke = pe * n;

                //exit velocity = sqrt(2 * kinetic energy / mass)
                v = Some(((2.0 * ke) / m.unwrap()).sqrt());
            }

            println!("Enter the air density (kilograms per meter cubed) (1.225 at sea level): ");
            p = Some(userinputf64());
            println!("Enter the drag coefficient (0.67 for a typical nerf dart): ");
            cd = Some(userinputf64());

            println!("Enter the diameter of the dart (inches): ");
            let d = userinputf64() / 39.37;
            //surface area = pi * (diameter / 2)^2
            a = Some(std::f64::consts::PI * ((d / 2.0) * (d / 2.0)));

            println!("Enter name");
            let mut name: String = String::new();

            std::io::stdin()
                .read_line(&mut name)
                .expect("Failed to read line");

            name = name.trim().to_string();

            //create save file
            let mut file = match fs::File::create(format!("{}.txt", name)) {
                Ok(f) => f,
                Err(e) => {
                    eprint!("Failed to create file! Err: {}", e);
                    return;
                }
            };

            //write to file
            if let Err(_) = writeln!(file, "{}", v.unwrap()) {
                eprintln!("Failed to write to file!");
                return;
            }

            if let Err(_) = writeln!(file, "{}", p.unwrap()) {
                eprintln!("Failed to write to file!");
                return;
            }

            if let Err(_) = writeln!(file, "{}", cd.unwrap()) {
                eprintln!("Failed to write to file!");
                return;
            }

            if let Err(_) = writeln!(file, "{}", a.unwrap()) {
                eprintln!("Failed to write to file!");
                return;
            }

            if let Err(_) = writeln!(file, "{}", m.unwrap()) {
                eprintln!("Failed to write to file!");
                return;
            }
        }

        if mode == 2 {
            let mut name: String = String::new();
            println!("Enter name");

            std::io::stdin()
                .read_line(&mut name)
                .expect("Failed to read line");

            //load data
            let loaddata = fs::read_to_string(format!("{}.txt", name.trim())).unwrap_or_else(|error| {
                if error.kind() == io::ErrorKind::NotFound {
                    println!("File not found!");
                    return String::new();
                
                } else {
                    panic!("Error: {}", error);
                }
            });

            if loaddata.is_empty() {
                continue;
            }

            let data = loaddata;
            let varables: Vec<&str> = data.lines().collect();

            v = Some(varables[0].parse::<f64>().expect("Failed to parse to f64"));
            p = Some(varables[1].parse::<f64>().expect("Failed to parse to f64"));
            cd = Some(varables[2].parse::<f64>().expect("Failed to parse to f64"));
            a = Some(varables[3].parse::<f64>().expect("Failed to parse to f64"));
            m = Some(varables[4].parse::<f64>().expect("Failed to parse to f64"));

            selected = true;
        }

        if mode == 3 {
            if selected == true {
                println!("Enter the firing height (feet): ");
                let initial_height: f64 = userinputf64() / 3.281;

                println!("Enter deltatime (miliseconds, a value of 1 is reccomended, as lower values may cause graphs to not load) (lower values lead to higher accuracy but longer computational time, as well as higher RAM usage): ");
                let dt: f64 = userinputf64() / 1000.0;

                println!("Enter angle step (degrees, a value of 1 is reccomended, as lower values may cause graphs to not load) (lower values lead to higher accuracy but longer computational time, as well as higher RAM usage): ");
                let ang_step: f64 = userinputf64();

                let mut all_distances: Vec<f64> = vec![];
                let mut all_positions: Vec<Vec<f64>> = vec![];
                let mut all_velocities: Vec<Vec<f64>> = vec![];
                let mut all_heights: Vec<Vec<f64>> = vec![];
                let mut angles: Vec<f64> = vec![];

                let mut ang: f64 = 0.0;

                while ang <= 90.0 {
                    let ang_radians = ang * (std::f64::consts::PI / 180.0);
                    let mut h = initial_height;
                    print!("\x1B[2J\x1B[1;1H"); //clear screen
                    println!("Progress: {}%", ((ang / 90.0) * 100.0).round());

                    let mut distance: f64 = 0.0;
                    let mut positions: Vec<f64> = vec![];
                    let mut velocities: Vec<f64> = vec![];
                    let mut heights: Vec<f64> = vec![];

                    let mut v_y: f64 = v.unwrap_or_else(|| {
                        println!("Velocity was not initialized, defaulting to 21");
                        21.0
                    });
                    v_y = v_y * (ang_radians as f64).sin();

                    let mut v_x: f64 = v.unwrap_or_else(|| {
                        println!("Velocity was not initialized, defaulting to 21");
                        21.0
                    });
                    v_x = v_x * (ang_radians as f64).cos();

                    let p: f64 = p.unwrap_or_else(|| {
                        println!("Air density was not initialized, defaulting to 1.225");
                        1.225
                    });

                    let cd: f64 = cd.unwrap_or_else(|| {
                        println!("Drag Coefficient was not initialized, defaulting to 0.67");
                        0.67
                    });

                    let a: f64 = a.unwrap_or_else(|| {
                        println!("Dart diameter was not initialized, defaulting to 0.0127");
                        0.0127
                    });

                    let m: f64 = m.unwrap_or_else(|| {
                        println!("Mass was not initialized, defaulting to 0.001");
                        0.001
                    });

                    while h >= 0.0 {
                        let vtotal: f64 = (v_x * v_x + v_y * v_y).sqrt();
                        let fd: f64 = 0.5 * p * (vtotal * vtotal) * cd * a;

                        v_x -= (fd / m) * (v_x / vtotal) * dt;
                        v_y -= (9.81 + (fd / m) * (v_y / vtotal)) * dt;

                        distance += v_x * dt;
                        h += v_y * dt;

                        if h >= 0.0 {
                            heights.push(h * 3.281);
                            positions.push(distance * 3.281);
                            velocities.push(vtotal * 3.281);
                        }
                    }

                    all_positions.push(positions.clone());
                    all_distances.push(positions[positions.len() - 1]);

                    all_heights.push(heights);

                    all_velocities.push(velocities);

                    angles.push(ang);

                    ang += ang_step;
                }

                print!("\x1B[2J\x1B[1;1H"); //clear screen
                println!("Progress: 100%");
                println!(
                    "Exit velocity (feet per second): {}",
                    (v.unwrap_or_else(|| { 21.0 })) * 3.281
                );

                let index_max = all_distances //find max distance
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

                if let Some((index, _)) = index_max {
                    //find angle at max distance
                    println!(
                        "Optimal Launch angle (degrees): {}",
                        index as f64 * ang_step
                    );
                }
                println!(
                    "Max range (feet): {}",
                    all_distances.iter().cloned().fold(f64::MIN, f64::max)
                );

                for x in 0..all_distances.len() {
                    println!(
                        "Max range for angle {} (feet): {}", (x as f64) * ang_step, 
                        all_distances[x]
                    );
                }

                //set plot axis
                println!("X-Axis of Graph:\n1. Distance (Default)\n2. Height\n3. Velocity\n");
                let x_axis_setting = userinputi32();
                println!("Y-Axis of Graph:\n1. Distance\n2. Height (Default)\n3. Velocity\n");
                let y_axis_setting = userinputi32();

                let x_axis_title: &str;
                let y_axis_title: &str;

                match x_axis_setting {
                    2 => x_axis_title = "Height",
                    3 => x_axis_title = "Velocity",
                    _ => x_axis_title = "Distance",
                }

                match y_axis_setting {
                    2 => y_axis_title = "Height",
                    3 => y_axis_title = "Velocity",
                    _ => y_axis_title = "Distance",
                }

                let mut x: Vec<f64> = vec![];
                let mut y: Vec<f64> = vec![];
                let mut z: Vec<f64> = vec![];

                for i in 0..angles.len() {
                    for j in 0..all_positions[i].len() {
                        match x_axis_setting {
                            2 => {
                                x.push(all_heights[i][j]);
                            }
                            3 => {
                                x.push(all_velocities[i][j]);
                            }
                            _ => {
                                x.push(all_positions[i][j]);
                            }
                        }

                        match y_axis_setting {
                            2 => {
                                y.push(all_heights[i][j]);
                            }
                            3 => {
                                y.push(all_velocities[i][j]);
                            }
                            _ => {
                                y.push(all_positions[i][j]);
                            }
                        }
                        z.push(angles[i]);
                    }

                    //prevent line connecting 2 different angles
                    x.push(f64::NAN);
                    y.push(f64::NAN);
                    z.push(f64::NAN);
                }

                let graph_name = format!("{} vs {} vs angle", x_axis_title, y_axis_title);
                let graph_file = format!("{}.html", graph_name);
                
                let layout = Layout::new()
                    //TODO fix axis titles
                    .x_axis(Axis::new().title(x_axis_title))
                    .y_axis(Axis::new().title(y_axis_title))
                    .z_axis(Axis::new().title("Angle"))
                    .width(1920)
                    .height(1080);

                let scatter = Scatter3D::new(x, y, z)
                    .mode(Mode::Markers)
                    .marker(Marker::new().size(1))
                    .name(graph_name);

                let mut plot = Plot::new();
                plot.set_layout(layout);
                plot.add_trace(scatter);

                plot.write_html(graph_file);
                plot.show();

                println!("X-axis: {}", x_axis_title);
                println!("Y-axis: {}", y_axis_title);
                println!("Z-axis: Angle");
            } else {
                println!("Select a blaster");
            }
        }

        if mode == 4 {
            process::exit(0);
        }
    }
}
