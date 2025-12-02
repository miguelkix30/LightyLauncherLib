
use crate::java::errors::{JavaRuntimeError, JavaRuntimeResult};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tracing::info;
use tokio::io::AsyncReadExt;
use tokio::process::{Child, Command};
use tokio::sync::oneshot::Receiver;
use tracing::debug;
pub struct JavaRuntime(pub PathBuf);

impl JavaRuntime {
    pub fn new(path: PathBuf) -> JavaRuntime {
        JavaRuntime(path)
    }

    pub async fn execute(&self, arguments: Vec<String>, game_dir: &Path) -> JavaRuntimeResult<Child> {
        if !self.0.exists() {
            return Err(JavaRuntimeError::NotFound {
                path: self.0.display().to_string(),
            });
        }
        let mut command = Command::new(&self.0);


        //DEBUG TEST
        debug!("Executing Java runtime: {}", self.0.display());
        info!("Executing Java runtime: {}", self.0.display());
        info!("Arguments: {:?}", &arguments);




        command.current_dir(game_dir);

        command.args(arguments);
        println!("Java runtime: {}", self.0.display());

        command.stderr(Stdio::piped()).stdout(Stdio::piped());

        let child = command.spawn()?;
        Ok(child)
    }

    pub async fn handle_io<D: Send + Sync>(
        &self,
        running_task: &mut Child,
        on_stdout: fn(&D, &[u8]) -> JavaRuntimeResult<()>,
        on_stderr: fn(&D, &[u8]) -> JavaRuntimeResult<()>,
        terminator: Receiver<()>,
        data: &D,
    ) -> JavaRuntimeResult<()> {
        let mut stdout = running_task.stdout.take()
            .ok_or(JavaRuntimeError::IoCaptureFailure)?;
        let mut stderr = running_task.stderr.take()
            .ok_or(JavaRuntimeError::IoCaptureFailure)?;

        let mut stdout_buf = vec![0; 1024];
        let mut stderr_buf = vec![0; 1024];

        tokio::pin!(terminator);

        loop {
            tokio::select! {
                read_len = stdout.read(&mut stdout_buf) => {
                    let _ = on_stdout(&data, &stdout_buf[..read_len?]);
                },
                read_len = stderr.read(&mut stderr_buf) => {
                    let _ = on_stderr(&data, &stderr_buf[..read_len?]);
                },
                _ = &mut terminator => {
                    running_task.kill().await?;
                    break;
                },
                exit_status = running_task.wait() => {
                    let code = exit_status?.code().unwrap_or(7900); // 7900 = unwrap failed error code

                    debug!("Process exited with code: {}", code);
                    if code != 0 && code != -1073740791 { // -1073740791 = happens when the process is killed forcefully, we don't want to error in this case
                        return Err(JavaRuntimeError::NonZeroExit { code });
                    }
                    break;
                },
            }
        }
        Ok(())
    }
}