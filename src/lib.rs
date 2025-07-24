use std::process::{Child, ChildStderr, ChildStdout, Command, Stdio};
use std::io::{self, BufReader, Read};

enum StreamCaptureError<'a> {
    AccessError(&'a str),
    StdoutCaptureError(&'a str),
    StderrCaptureError(&'a str),
    StderrNotEmpty(&'a str)
}

impl From<StreamCaptureError<'_>> for io::Error {
    fn from(err: StreamCaptureError) -> io::Error {
        match err {
            StreamCaptureError::AccessError(msg) => io::Error::new(io::ErrorKind::Other, msg),
            StreamCaptureError::StdoutCaptureError(msg) => io::Error::new(io::ErrorKind::Other, msg),
            StreamCaptureError::StderrCaptureError(msg) => io::Error::new(io::ErrorKind::Other, msg),
            StreamCaptureError::StderrNotEmpty(msg) => io::Error::new(io::ErrorKind::Other, msg),
        }
    }
}

pub struct Popen {
    command: String, 
    child: Option<Child> 
}

impl Popen {
    /// Initialize 
    ///
    /// # Parameters
    /// - `command`: The command to run; accepts args
    pub fn new(command: &str) -> Self {
        Self { 
            command: String::from(command),
            child: None
        }
    }

    /// Tokenize the provided command and setup the child process 
    /// handles for stdout and stderr.
    /// 
    /// # Returns
    /// - An object of the constructed `Command` 
    fn spawn_process(&self) -> Command {
        let tokenize: Vec<&str> = self.command.split_whitespace().collect();
        let file = tokenize[0];
    
        let mut child_process = Command::new(file);
    
        if tokenize.len() > 1 {
            for arg in &tokenize[1..] {
                child_process.arg(arg);
            }
        }
    
        child_process.stdout(Stdio::piped());
        child_process.stderr(Stdio::piped());

        child_process
    }

    /// Capturing stdout and stderr handles for `stream_reader` function.
    ///
    /// # Returns
    /// - `Result<(ChildStdout, ChildStderr), io::Error>` containing stdout 
    ///    and stderr handles; otherwise, `StreamCaptureError`.
    fn stream_capture(&mut self) -> Result<(ChildStdout, ChildStderr), io::Error> {
        let child = self.child
            .as_mut()
            .ok_or_else(|| StreamCaptureError::AccessError("Error accessing child process"))?;

        let stdout = child.stdout
            .take()
            .ok_or_else(|| StreamCaptureError::StdoutCaptureError("Failed to capture stdout"))?;

        let stderr = child.stderr
            .take()
            .ok_or_else(|| StreamCaptureError::StderrCaptureError("Failed to capture stderr"))?;

        Ok((stdout, stderr))
    }

    /// Reading the bytes from the specified stream handle and append them to `buffer`.
    ///
    /// # Parameters
    /// - `stream`: The stream handle of either stdout or stderr
    ///
    /// # Returns
    /// - `Result<String, io::Error>` containing the output string of either stderr or
    ///    stdout; otherwise, `io::Error`.
    fn stream_reader<T: std::io::Read>(&self, stream: T) -> Result<String, io::Error> {
        let mut reader = BufReader::new(stream);
        let mut buffer = String::new();
        
        reader.read_to_string(&mut buffer)?;

        Ok(buffer)
    }

    /// Try to get the OS specified process identifier of the child process.
    ///
    /// # Returns
    /// - `Option<u32>` containing the process identifier of the child 
    ///   process; otherwise, `None`.
    #[must_use]
    pub fn pid(&mut self) -> Option<u32> {
        Some(self.child.as_mut()?.id())
    }

    /// Spawns the constructed child process, captures the stream handles
    /// for stdout and stderr and reads the final output.
    ///
    /// # Returns
    /// - `Result<String>` containing the command output.
    #[must_use]
    pub fn spawn(&mut self) -> io::Result<String> {    
        self.child = Some(self.spawn_process().spawn()?);
    
        let (stdout, stderr) = self.stream_capture()?;
        let stdout_string = self.stream_reader(stdout)?;
        let stderr_string = self.stream_reader(stderr)?;
    
        if !stderr_string.is_empty() {
            return Err(StreamCaptureError::StderrNotEmpty(&stderr_string).into());
        }
    
        Ok(stdout_string)
    }
}