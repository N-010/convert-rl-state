mod common;
mod new_rl;
mod old_rl;

use crate::new_rl::NewRL;
use old_rl::OldRL;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

async fn read_contract_file<P: AsRef<Path>>(
    path: P,
) -> Result<Box<OldRL>, Box<dyn std::error::Error>> {
    println!("📂 Opening file: {:?}", path.as_ref());

    let mut file = File::open(path).await?;
    let mut buffer = Vec::new();

    println!("📖 Reading file contents...");
    file.read_to_end(&mut buffer).await?;

    println!("✓ Read {} bytes", buffer.len());
    // OldRL structure size for validation
    let expected_size = std::mem::size_of::<OldRL>();
    println!("ℹ️  Expected OldRL structure size: {} bytes", expected_size);
    println!("🔄 Deserializing OldRL structure...");

    // Output first 32 bytes for debugging
    print!("   First 32 bytes of file: ");
    for byte in buffer.iter().take(32) {
        print!("{:02X} ", byte);
    }
    println!();

    // Validate expected size
    let expected = std::mem::size_of::<OldRL>();
    if buffer.len() != expected {
        return Err(format!(
            "Expected size {} bytes does not match file size {} bytes",
            expected,
            buffer.len()
        )
        .into());
    }

    // Create uninitialized buffer for OldRL and copy bytes
    let mut boxed: Box<OldRL> = unsafe { Box::new(std::mem::MaybeUninit::zeroed().assume_init()) };
    unsafe {
        std::ptr::copy_nonoverlapping(
            buffer.as_ptr(),
            boxed.as_mut() as *mut OldRL as *mut u8,
            expected,
        );
    }

    println!("✓ Byte-by-byte structure loading successful!\n");

    Ok(boxed)
}

/// Asynchronously saves NewRL to a binary file
async fn write_new_rl_to_file<P: AsRef<Path>>(
    path: P,
    new_rl: &NewRL,
) -> Result<(), Box<dyn std::error::Error>> {
    use tokio::io::AsyncWriteExt;

    println!("\n💾 Saving NewRL to file: {:?}", path.as_ref());

    let size = std::mem::size_of::<NewRL>();
    println!("   NewRL structure size: {} bytes", size);

    // Create buffer and copy structure bytes
    let bytes = unsafe { std::slice::from_raw_parts(new_rl as *const NewRL as *const u8, size) };

    let mut file = File::create(path).await?;
    file.write_all(bytes).await?;
    file.flush().await?;

    println!("✓ File successfully written ({} bytes)", size);

    Ok(())
}

/// Prints program usage help
fn print_usage(program_name: &str) {
    eprintln!("Usage: {} <input_file> <output_file>", program_name);
    eprintln!();
    eprintln!("Arguments:");
    eprintln!("  <input_file>   Path to the OldRL state file for reading");
    eprintln!("  <output_file>  Path to the file for saving NewRL");
    eprintln!();
    eprintln!("Example:");
    eprintln!("  {} contract0016.185 contract0016_new.185", program_name);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎰 Random Lottery Contract - State Converter\n");

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("❌ Error: incorrect number of arguments\n");
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    println!("📥 Input file:  {}", input_file);
    println!("📤 Output file: {}\n", output_file);

    // Check input file existence
    if !Path::new(input_file).exists() {
        eprintln!("❌ Error: input file '{}' not found", input_file);
        std::process::exit(1);
    }

    // Read OldRL
    let rl_state = read_contract_file(input_file).await?;
    println!("{}", rl_state);

    // Convert state to NewRL
    let new_rl: NewRL = NewRL::from(rl_state.as_ref());

    // Detailed output of NewRL
    println!("{}", new_rl);

    // Save NewRL to binary file
    write_new_rl_to_file(output_file, &new_rl).await?;
    println!("\n✅ NewRL successfully saved to '{}'", output_file);
    println!("\n✅ Conversion completed successfully!");

    Ok(())
}
