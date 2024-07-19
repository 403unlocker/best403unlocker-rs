# Best 403 Unlocker (Rust Version)

## Key Differences and Features of the Rust Version

The Rust version of the 403 unlocker tool, available in [here]("https://github.com/403unlocker/best403unlocker-rs"), introduces several key differences and features compared to the original project:

### Rust-based Implementation

The primary difference is that the tool has been rewritten using the Rust programming language, leveraging Rust's native tools and libraries. This allows for improved performance, safety, and cross-platform compatibility compared to the original version.

### Cross-platform Compatibility

Thanks to amazing rust this project now supports both ***Windows*** and ***Linux***.

### Simplified Deployment

Just a single executable file and its `best403unlocker.conf` file to check for best DNSs.

## How to Run the Rust Version

To run the Rust-based 403 unlocker tool, there is two method:

### Method 1 - using released version ( perfered )

1. just download the latest released version base on your OS from [releases](https://github.com/403unlocker/best403unlocker-rs/releases/).
2. unzip and run the program (pay attention that .conf file and the executable file should be in the same directory)

### Method 2 - build form source ( suitable for development )

#### Prerequisites

Before running the Rust version of the 403 unlocker tool, ensure that you haver Rust programming language installed:

- You can install Rust by following the official instructions for your operating system: https://www.rust-lang.org/tools/install

---

### Build and run

1. **Clone the Repository**: Clone the repository from GitHub using the following command:

```
git clone https://github.com/403unlocker/best403unlocker-rs.git
```

2. **Navigate to the Project Directory**: Change your current working directory to the cloned repository:

```
cd best403unlocker-rs
```

3. **Build the Project**: Use Cargo, the Rust package manager and build tool, to build the project:

```
cargo build --release
```

This will compile the Rust code and create a optimized, release-ready executable.

4. **Run the Tool**: Execute the built binary to run the 403 unlocker tool:

```
./target/release/best403unlocker-rs
```

5. (optional):copy the released app to `$HOME/.cargo/bin`:

- On Linux:
  ```
  cp .\target\release\best403unlocker-rs ~/.cargo/bin
  ```
- On Windows: 
    ```
    copy .\target\release\best403unlocker-rs.exe %USERPROFILE%\.cargo\bin\
    ```

The tool should now be running and ready to use.


## credit
Many thanks to [Arman Taheri](https://github.com/ArmanTaheriGhaleTaki) for its great idea!
## Contact
Fell free to open PR and issues

I will try my best to answer them as soon as possible

[@BrPrS](https://github.com/BrPrS)