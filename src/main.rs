use std::io::{ Result, Write };
use std::fs::OpenOptions;
use structopt::StructOpt;
use rand::Rng;

#[derive(Debug, StructOpt)]
struct Cmd {
    #[structopt(short = "a", long = "add", help = "Add a thought", conflicts_with("random"))]
    thought: Option<String>,

    #[structopt(short, long, help = "Prints a random thought")]
    random: bool
}

fn print_random(thoughts: &str) {
    let mut vec = Vec::new();
    for thought in thoughts.lines() {
        vec.push(thought);
    }
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0, vec.len());

    println!("{}", vec[n]);
}

fn main() -> Result<()> {
    let args = Cmd::from_args();

    // Load thoughts in memory (heap?)
    let thoughts = std::fs::read_to_string("thoughts.txt")?;

    // Add a new thought
    if let Some(i) = &args.thought {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("thoughts.txt")?;
        writeln!(&mut file, "{}", i).unwrap();
    }
    // Print a random thought
    else if args.random {
        print_random(&thoughts);
    }

    Ok(())
}
