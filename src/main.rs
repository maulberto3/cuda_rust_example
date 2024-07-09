use std::process::Command;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Run the compiled CUDA executable
    let output = Command::new("./src/elementwise_product")
        .output()
        .expect("Failed to execute CUDA code");

    // Print CUDA program output
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    Ok(())
}
