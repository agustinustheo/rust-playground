# Retry Loop

This Rust project demonstrates a retry loop mechanism using Tokio for asynchronous operations.

## Description

This program implements a retry loop that repeatedly calls a potentially failing function for a maximum duration of 10 seconds. It uses Tokio for asynchronous execution and handling of concurrent tasks.

Key features:
- Asynchronous execution using Tokio
- Simulated function that may succeed or fail randomly
- Retry loop with a maximum duration
- Consistent 1-second sleep between retries

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## Dependencies

- tokio = { version = "1", features = ["full"] }
- rand = "0.8"

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/retry_loop.git
   cd retry_loop
   ```

2. Build the project:
   ```
   cargo build
   ```

## Usage

Run the program using:

```
cargo run
```

The program will execute for approximately 11 seconds, attempting the potentially failing function multiple times.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.