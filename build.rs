use std::process::Command;

fn main() {
    // Path to CUDA source file
    let cuda_src = "src/elementwise_product.cu";

    // Compile CUDA code using nvcc
    let output = Command::new("nvcc")
        .arg("-o").arg("src/elementwise_product")
        .arg(cuda_src)
        .output()
        .expect("Failed to compile CUDA code");

    if !output.status.success() {
        println!("Error compiling CUDA code:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }

    println!("CUDA code compiled successfully!");
}
