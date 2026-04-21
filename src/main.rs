use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        return Err("Usage: [Usage]".into());
    }

    let file_path = &args[1];

    println!("{}", file_path);
    Ok(())
}
