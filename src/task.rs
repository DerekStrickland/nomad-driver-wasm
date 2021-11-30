use std::collections::HashMap;
use std::sync::{Arc, Mutex};
// TODO: I'm experimenting with embedding the WASI Runtime on the task directly.
// It may prove more prudent to separate this out.
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

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
use crate::task_kernel;
use std::path::Path;


pub struct Task {
    pub config: TaskConfig,
    pub handle: Option<TaskHandle>,
    pub config_schema: Spec,
    pub status: Option<TaskStatus>,
    pub driver_status: Option<TaskDriverStatus>,
    pub timeout: Option<Duration>,
    pub network_override: Option<NetworkOverride>,
    store: Store<Task>,
    engine: Engine,
    linker: Linker<Task>,
}

impl Task {
    fn new(config: TaskConfig) -> Self {
        // Define the WASI functions globally on the `Config`.
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

        // Create a WASI context and put it in a Store; all instances in the store
        // share this context. `WasiCtxBuilder` provides a number of ways to
        // configure what the target program will have access to.
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args()?
            .build();

        // Create the store instance
        let mut store = Store::new(&engine, wasi);

        // Return the initialized runtime.
        Task {
            config,
            handle: None,
            config_schema: Spec { block: None },
            status: None,
            driver_status: None,
            timeout: None,
            network_override: None,
            store,
            engine,
            linker,
        }
    }

    fn load_wasm_module(&self, module_name: String, module_path: Path) -> Result<(), E> {
        // TODO: Retrieve from the OCI registry.
        // Instantiate our module with the imports we've created, and run it.
        let module = Module::from_file(&engine, module_path)?;

        // Link the module by name to the store instance.
        linker.module(&mut store, module_name.as_str(), &module)?;
        linker
            .get_default(&mut store, "")?
            .typed::<(), (), _>(&store)?
            .call(&mut store, ())?;

        Ok(())
    }

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
