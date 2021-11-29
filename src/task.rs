use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::proto::google::protobuf::Duration;
use crate::proto::hashicorp::nomad::plugins::drivers::proto::start_task_response as start_task;
use crate::proto::hashicorp::nomad::plugins::drivers::proto::{
    CreateNetworkRequest, CreateNetworkResponse, DestroyNetworkRequest, DestroyNetworkResponse,
    DestroyTaskRequest, DestroyTaskResponse, ExecTaskRequest, ExecTaskResponse,
    ExecTaskStreamingIoOperation, ExecTaskStreamingRequest, ExecTaskStreamingResponse, ExitResult,
    InspectTaskRequest, InspectTaskResponse, NetworkOverride, RecoverTaskRequest,
    RecoverTaskResponse, SignalTaskRequest, SignalTaskResponse, TaskConfig, TaskDriverStatus,
    TaskEventsRequest, TaskHandle, TaskStatsResponse, TaskStatus,
};
use crate::proto::hashicorp::nomad::plugins::shared::hclspec::Spec;
use crate::wasi;

pub struct TaskController {
    pub registry_url: String,
    pub tasks: Arc<Mutex<HashMap<String, Task>>>,
}

impl TaskController {
    pub fn default() -> TaskController {
        TaskController {
            registry_url: String::from("0.0.0.0:5000"),
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn new(registry_url: String) -> TaskController {
        TaskController {
            registry_url,
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_network(request: CreateNetworkRequest) -> CreateNetworkResponse {
        CreateNetworkResponse {
            isolation_spec: None,
            created: false,
        }
    }

    pub fn destroy(request: DestroyTaskRequest) -> DestroyTaskResponse {
        DestroyTaskResponse {}
    }

    pub fn destroy_network(request: DestroyNetworkRequest) -> DestroyNetworkResponse {
        DestroyNetworkResponse {}
    }

    pub fn exec(&self, request: ExecTaskRequest) -> ExecTaskResponse {
        // TODO: Refactor all this to template function to reduce boilerplate.
        let task_id = request.task_id.clone().as_str();
        if task_id.is_empty() {
            ExecTaskResponse {
                stdout: vec![],
                stderr: vec![],
                result: exit_result(ExitCodes::InvalidArgument),
            }
        }

        let tasks = Arc::clone(&self.tasks);
        let handles = tasks.lock().unwrap();
        if !handles.contains_key(task_id) {
            ExecTaskResponse {
                stdout: vec![],
                stderr: vec![],
                result: exit_result(ExitCodes::CommandNotFound),
            }
        }

        let task_handle = handles.get(request.task_id.as_str())?;

        task_handle.exec();

        ExecTaskResponse {
            stdout: vec![],
            stderr: vec![],
            result: None,
        }
    }

    pub fn exec_streaming(request: ExecTaskStreamingRequest) -> ExecTaskStreamingResponse {
        ExecTaskStreamingResponse {
            stdout: None,
            stderr: None,
            exited: false,
            result: None,
        }
    }

    pub fn inspect(request: InspectTaskRequest) -> InspectTaskResponse {
        InspectTaskResponse {
            task: None,
            driver: None,
            network_override: None,
        }
    }

    pub fn recover(request: RecoverTaskRequest) -> RecoverTaskResponse {
        RecoverTaskResponse {}
    }
}

fn exit_result(exit_code: ExitCodes) -> Option<ExitResult> {
    Some(ExitResult {
        exit_code:  exit_code as i32,
        signal: 0,
        oom_killed: false,
    })
}

// Adapted from https://tldp.org/LDP/abs/html/exitcodes.html
pub enum ExitCodes {
    Error = 1, // Catchall for general errors
    BuiltinMisuse = 2, // Misuse of shell builtins (according to Bash documentation)
    CannotExecute = 126, // Command invoked cannot execute
    CommandNotFound = 127, // “command not found”
    InvalidArgument = 128, // Invalid argument to exit
    Exited = 130,  // Terminated by Control-C
    OutOfRange = 255 // Exit status out of range
}

pub struct Task {
    pub handle: Option<TaskHandle>,
    pub config_schema: Spec,
    pub status: Option<TaskStatus>,
    pub driver_status: Option<TaskDriverStatus>,
    pub timeout: Option<Duration>,
    pub network_override: Option<NetworkOverride>,
}

impl Task {
    // exec launches a new wasmtime task.
    pub fn exec(&self, opts: &ExecOptions) -> ExecTaskResponse {
        spec, err := h.container.Spec(ctxContainerd)
        if err != nil {
            return nil, err
        }

        pspec := spec.Process
        pspec.Terminal = opts.tty
        pspec.Args = opts.Command
        execID, err := uuid.GenerateUUID()
        if err != nil {
            return nil, err
        }

        cioOpts := []cio.Opt{cio.WithStreams(opts.Stdin, opts.Stdout, opts.Stderr)}
        if opts.Tty {
            cioOpts = append(cioOpts, cio.WithTerminal)
        }
        ioCreator := cio.NewCreator(cioOpts...)

        process, err := h.task.Exec(ctxContainerd, execID[:8], pspec, ioCreator)
        if err != nil {
            return nil, err
        }
        go func() {
            for {
                select {
                    case s, ok := <-opts.ResizeCh:
                    if !ok {
                        return
                    }
                    if err = h.task.Resize(ctxContainerd, uint32(s.Width), uint32(s.Height)); err != nil {
                    h.logger.Error("Failed to resize terminal", "error", err)
                    return
                }
            }
        }
    }()

        defer process.Delete(ctxContainerd)

        statusC, err := process.Wait(ctxContainerd)
        if err != nil {
        return nil, err
        }

        if err := process.Start(ctxContainerd); err != nil {
        return nil, err
        }

        var code uint32
        status := <-statusC
        code, _, err = status.Result()
        if err != nil {
        return nil, err
        }

        go func() {
        for {
        select {
        case <-ctx.Done():
        return
        }
        }
        }()

        return &drivers.ExitResult{
        ExitCode: int(code),
        }, nil

        opts.stdout::Close()
        opts.stderr::Close()

        ExecTaskResponse{
            stdout: vec![],
            stderr: vec![],
            result: exit_result(ExitCodes::CannotExecute)
        }
    }

    pub fn get_stats(&self) -> TaskStatsResponse {
        TaskStatsResponse { stats: None }
    }

    pub fn signal(&self, request: SignalTaskRequest) -> SignalTaskResponse {
        SignalTaskResponse {}
    }
}

pub struct ExecOptions {
    // Command is command to run
    pub command: Vec<String>,

    // Tty indicates whether pseudo-terminal is to be allocated
    pub tty: bool,

    // streams
    pub stdin:  std::io::Stdin,
    pub stdout: std::io::Stdout,
    pub stderr: std::io::Stderr
}
