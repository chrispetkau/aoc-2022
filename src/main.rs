use std::time::{Duration, Instant};

mod artifact {
    use std::{
        fs::{create_dir, File},
        io::Write,
    };

    const ARTIFACTS_FOLDER: &str = "artifacts";

    #[cfg(debug_assertions)]
    fn build_config() -> &'static str {
        "Debug"
    }

    #[cfg(not(debug_assertions))]
    fn build_config() -> &'static str {
        "Release"
    }

    pub fn make_artifact(folder: Option<&str>, description: &str, row_headers: &str, row: &str) {
        let _ = create_dir(ARTIFACTS_FOLDER);
        let path = if let Some(folder) = folder {
            format!("{}/{}", ARTIFACTS_FOLDER, folder)
        } else {
            String::from(ARTIFACTS_FOLDER)
        };
        let _ = create_dir(&path);
        let filename = format!(
            "{}/{}_{}.csv",
            path,
            description,
            chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
        );
        match File::create(filename) {
            Ok(mut file) => {
                let _ = writeln!(&mut file, "build_config,{}", row_headers);
                let _ = writeln!(&mut file, "{},{}", build_config(), row,);
            }
            Err(error) => {
                println!("Failed to create artifact file: {}", error);
            }
        }
    }
}

macro_rules! solve {
    ($day:ident) => {{
        let timer = Instant::now();
        let (part1, part2, parse_duration) = $day::solve();
        let duration = timer.elapsed() - parse_duration;
        println!("{} = {}, {}", stringify!($day), part1, part2);
        println!("time = {} ms", duration.as_secs_f64() * 1000.0);
        println!();

        artifact::make_artifact(
            Some(stringify!($day)),
            "run",
            "part1,part2,duration,parse_duration",
            &format!(
                "{},{},{},{}",
                part1,
                part2,
                duration.as_secs_f64() * 1000.0,
                parse_duration.as_secs_f64() * 1000.0
            ),
        );

        duration
    }};
}

#[cfg(feature = "day01")]
mod day01;
#[cfg(feature = "day02")]
mod day02;
#[cfg(feature = "day03")]
mod day03;
#[cfg(feature = "day04")]
mod day04;
#[cfg(feature = "day05")]
mod day05;
#[cfg(feature = "day06")]
mod day06;
#[cfg(feature = "day07")]
mod day07;
#[cfg(feature = "day08")]
mod day08;
#[cfg(feature = "day09")]
mod day09;
#[cfg(feature = "day10")]
mod day10;
#[cfg(feature = "day11")]
mod day11;
#[cfg(feature = "day12")]
mod day12;
#[cfg(feature = "day13")]
mod day13;
#[cfg(feature = "day14")]
mod day14;
#[cfg(feature = "day15")]
mod day15;
#[cfg(feature = "day16")]
mod day16;

fn main() {
    let mut total_duration = Duration::new(0, 0);
    #[cfg(feature = "day01")]
    {
        total_duration += solve!(day01);
    }
    #[cfg(feature = "day02")]
    {
        total_duration += solve!(day02);
    }
    #[cfg(feature = "day03")]
    {
        total_duration += solve!(day03);
    }
    #[cfg(feature = "day04")]
    {
        total_duration += solve!(day04);
    }
    #[cfg(feature = "day05")]
    {
        total_duration += solve!(day05);
    }
    #[cfg(feature = "day06")]
    {
        total_duration += solve!(day06);
    }
    #[cfg(feature = "day07")]
    {
        total_duration += solve!(day07);
    }
    #[cfg(feature = "day08")]
    {
        total_duration += solve!(day08);
    }
    #[cfg(feature = "day09")]
    {
        total_duration += solve!(day09);
    }
    #[cfg(feature = "day10")]
    {
        total_duration += solve!(day10);
    }
    #[cfg(feature = "day11")]
    {
        total_duration += solve!(day11);
    }
    #[cfg(feature = "day12")]
    {
        total_duration += solve!(day12);
    }
    #[cfg(feature = "day13")]
    {
        total_duration += solve!(day13);
    }
    #[cfg(feature = "day14")]
    {
        total_duration += solve!(day14);
    }
    #[cfg(feature = "day15")]
    {
        total_duration += solve!(day15);
    }
    #[cfg(feature = "day16")]
    {
        total_duration += solve!(day16);
    }
    println!(
        "Total time for 2022: {} ms",
        total_duration.as_secs_f64() * 1000.0
    );
}
