# popen_rs - Simple Process Spawning Library

### Description

```
This small Rust library simplifies the process of running system commands by 
spawning a new child process and reading the content of either the stdout 
or stderr stream handle.
```

### Usage

- Add the crate to your project
```bash
cargo add popen_rs
```

- Import the popen_rs crate to your source file

```rust
use popen_rs::Popen;
```

### Methods

#### `spawn`

##### Spawns a new child process and captures the handles of stdout and stderr. This function will return the content of either stdout or stderr.


```rust
pub fn spawn(&mut self) -> io::Result<String>
```

#### `pid`
##### Requests the OS specified process identifier of the current child process.

```rust
pub fn pid(&mut self) -> Option<u32>
```


### Example
```rust
use popen_rs::Popen;

fn main() {
    let command = "ls -l";
    let mut process = Popen::new(command);

    match process.spawn() {
        Ok(output) => {
            let pid = process.pid().unwrap();

            println!("Output of `{}` [PID:{}]:", command, pid);
            println!("\n{}", output);
        },
        Err(err) => eprintln!("Error: {}", err),
    }
}
```

## License
This project is published under the [MIT](https://github.com/f42h/popen_rs/blob/master/LICENSE) License. 