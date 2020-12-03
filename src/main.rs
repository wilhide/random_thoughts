use std::io::{ Result, Write, BufReader, BufRead };
use std::fs::OpenOptions;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cmd {
    #[structopt(short = "a", long = "add", help = "Add a thought", conflicts_with("random"))]
    thought: Option<String>,

    #[structopt(short, long, help = "Prints a random thought")]
    random: bool
}

fn main() -> Result<()> {
    println!("Hello, world!");

    let args = Cmd::from_args();

    if let Some(i) = &args.thought {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("thoughts.txt")?;
        writeln!(&mut file, "{}", i).unwrap();
        // file.write_all(i.as_bytes())?;
        // file.write_all("\r\n".as_bytes())?;
    }
    else if args.random {
        let content = std::fs::read_to_string("thoughts.txt")?;
        println!("{:?}", content);
        // let input = OpenOptions::new().read(true).open("thoughts.txt")?;
        // let buffered = BufReader::new(input);
        // for line in buffered.lines() {
        //     println!("{}", line?);
        // }
    }

    println!("{:?}", args);

    Ok(())
}
