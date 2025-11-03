# Random Lottery Contract - State Converter

A Rust utility for converting Qubic Random Lottery contract state files from the old format (OldRL) to the new format (NewRL).

## 📋 Description

This tool reads a binary state file in the old Random Lottery format and converts it to the new format, preserving all essential data structures and state information.

## 🚀 Installation

### Option 1: Use Pre-built Binaries (Recommended for most users)

Pre-built binaries are available in the `release-binaries/` directory:

- **Windows (x64):** `release-binaries/rlconverter-windows-x64.exe`
- **Linux (x64):** `release-binaries/rlconverter-linux-x64`

See `release-binaries/README.md` for detailed instructions on using pre-built binaries.

### Option 2: Install Rust (only if you want to run from source)

If you prefer to run the program from source, install Rust first.

#### Installing Rust on Linux

1. Open a terminal and run:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Follow the on-screen instructions (default is fine).
3. Restart the terminal or run:
   ```bash
   source $HOME/.cargo/env
   ```
4. Verify:
   ```bash
   rustc --version
   cargo --version
   ```

#### Installing Rust on Windows

1. Download the Rust installer from https://rustup.rs/ (or use `rustup-init.exe`).
2. Run the installer and follow the wizard (default installation is recommended).
3. Open a new Command Prompt or PowerShell and verify:
   ```cmd
   rustc --version
   cargo --version
   ```

## 📖 Usage

### Command Syntax

```
rlconverter <input_file> <output_file>
```

### Arguments

- `<input_file>` - Path to the old format Random Lottery state file (OldRL)
- `<output_file>` - Path where the converted new format file (NewRL) will be saved

### Examples

Using pre-built binaries (recommended):

Linux:
```bash
chmod +x release-binaries/rlconverter-linux-x64
./release-binaries/rlconverter-linux-x64 contract0016.185 contract0016_new.185
```

Windows:
```cmd
release-binaries\rlconverter-windows-x64.exe contract0016.185 contract0016_new.185
```

Quick run from source (no separate build step required):

From the project root you can run the program directly with Cargo (useful for testing or quick runs):

```bash
cargo run -- contract0016.185 contract0016_new.185
```

Cargo will compile and run the binary in debug mode; for repeated or production runs consider using the pre-built release binaries in `release-binaries/` or building a release locally with `cargo build --release`.

### Example Output

```
🎰 Random Lottery Contract - State Converter

📥 Input file:  contract0016.185
📤 Output file: contract0016_new.185

📂 Opening file: "contract0016.185"
📖 Reading file contents...
✓ Read 123456 bytes
ℹ️  Expected OldRL structure size: 123456 bytes
🔄 Deserializing OldRL structure...
   First 32 bytes of file: 00 01 02 03 ...
✓ Byte-by-byte structure loading successful!

[OldRL state information displayed]

💾 Saving NewRL to file: "contract0016_new.185"
   NewRL structure size: 98765 bytes
✓ File successfully written (98765 bytes)

✅ NewRL successfully saved to 'contract0016_new.185'

✅ Conversion completed successfully!
```

## 🐛 Troubleshooting

### File Not Found Error
```
❌ Error: input file 'contract0016.185' not found
```
**Solution:** Make sure the input file exists and the path is correct.

### Size Mismatch Error
```
Expected size X bytes does not match file size Y bytes
```
**Solution:** The input file may be corrupted or not in the correct OldRL format.