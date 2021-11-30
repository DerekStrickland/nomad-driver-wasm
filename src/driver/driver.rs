
use super::{Arc, Pin, Request, Response, Status, Streaming, WasmDriver, FINGERPRINT_PERIOD};

// Alias nomad modules
use crate::proto::hashicorp::nomad::plugins as nomad;
use nomad::drivers::proto as drivers;

// Import crate types
use crate::driver::config::task_config_schema;
use crate::fingerprint::fingerprinter::build_fingerprint_attrs;
use crate::task::ExecOptions;
use drivers::driver_server::Driver;
use drivers::fingerprint_response::HealthState;
use drivers::{
    CapabilitiesRequest, CapabilitiesResponse, CreateNetworkRequest, CreateNetworkResponse,
    DestroyNetworkRequest, DestroyNetworkResponse, DestroyTaskRequest, DestroyTaskResponse,
    DriverTaskEvent, ExecTaskRequest, ExecTaskResponse, ExecTaskStreamingRequest,
    ExecTaskStreamingResponse, ExitResult, FingerprintRequest, FingerprintResponse, InspectTaskRequest,
    InspectTaskResponse, RecoverTaskRequest, RecoverTaskResponse, SignalTaskRequest,
    SignalTaskResponse, StartTaskRequest, StartTaskResponse, StopTaskRequest, StopTaskResponse,
    TaskConfigSchemaRequest, TaskConfigSchemaResponse, TaskEventsRequest, TaskStatsRequest,
    TaskStatsResponse, WaitTaskRequest, WaitTaskResponse,
};
use nomad::shared::structs::attribute::Value;

#[tonic::async_trait]
impl Driver for WasmDriver {
    async fn task_config_schema(
        &self,
        request: Request<TaskConfigSchemaRequest>,
    ) -> Result<Response<TaskConfigSchemaResponse>, Status> {
        Ok(tonic::Response::new(task_config_schema()))
    }

    async fn capabilities(
        &self,
        request: Request<CapabilitiesRequest>,
    ) -> Result<Response<CapabilitiesResponse>, Status> {
        // log::info!("Received CapabilitiesRequest");
        Ok(tonic::Response::new(CapabilitiesResponse {
            capabilities: Some(WasmDriver::default_capabilities()),
        }))
    }

    type FingerprintStream = Pin<
        Box<
            dyn futures_core::Stream<Item = Result<FingerprintResponse, Status>>
                + Send
                + Sync
                + 'static,
        >,
    >;

    async fn fingerprint(
        &self,
        request: Request<FingerprintRequest>,
    ) -> Result<Response<Self::FingerprintStream>, Status> {
        let (tx, rx) = tokio::sync::mpsc::channel(4);

        let attrs = build_fingerprint_attrs();

        let fingerprint_response = FingerprintResponse {
            attributes: attrs.clone(),
            health: HealthState::Healthy as i32,
            health_description: String::from("healthy"),
        };

        for (k, v) in attrs {
            match v.value {
                Some(Value::StringVal(val)) => {
                    log::info!("attribute {}: {}", k, val)
                }
                _ => log::info!("attribute {} is not a string", k),
            }
        }

        tokio::spawn(async move {
            loop {
                tx.send(Ok(fingerprint_response.clone())).await.unwrap();
                tokio::time::sleep(FINGERPRINT_PERIOD).await;
            }
        });

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        )))
    }

    async fn recover_task(
        &self,
        request: Request<RecoverTaskRequest>,
    ) -> Result<Response<RecoverTaskResponse>, Status> {
        // log::info!("Received RecoverTaskRequest");
        Ok(tonic::Response::new(RecoverTaskResponse {}))
    }

    async fn start_task(
        &self,
        request: Request<StartTaskRequest>,
    ) -> Result<Response<StartTaskResponse>, Status> {
        // Get the task from the request
        let task = request.get_ref().task.unwrap();
        // Guard against already running task.
        if self.is_running(task.id) {
            Err(format!("task with ID {} already started", task.id))
        }

        // Deserialize the Task config from the MessagePack format.
        var driverConfig TaskConfig
        if err := cfg.DecodeDriverConfig(&driverConfig); err != nil {
            return nil,
            nil,
            fmt.Errorf("failed to decode driver config: %v",
            err)
        }

        // Create the module config
        containerConfig := ContainerConfig{}

        if driverConfig.HostNetwork && cfg.NetworkIsolation != nil {
            return nil, nil, fmt.Errorf("host_network and bridge network mode are mutually exclusive, and only one of them should be set")
        }

        if err := driverConfig.setVolumeMounts(cfg); err != nil {
            return nil, nil, err
        }

        d.logger.Info("starting task", "driver_cfg", hclog.Fmt("%+v", driverConfig))
        handle := drivers.NewTaskHandle(taskHandleVersion)
        handle.Config = cfg

        // https://www.nomadproject.io/docs/drivers/docker#container-name
        containerName := cfg.Name + "-" + cfg.AllocID
        containerConfig.ContainerName = containerName

        var err error
        containerConfig.Image, err = d.pullImage(driverConfig.Image, driverConfig.ImagePullTimeout, &driverConfig.Auth)
        if err != nil {
            return nil, nil, fmt.Errorf("Error in pulling image %s: %v", driverConfig.Image, err)
        }

        d.logger.Info(fmt.Sprintf("Successfully pulled %s image\n", containerConfig.Image.Name()))

        // Setup environment variables.
        for key, val := range cfg.Env {
            if skipOverride(key) {
                continue
            }
            containerConfig.Env = append(containerConfig.Env, fmt.Sprintf("%s=%s", key, val))
        }

        // Setup source paths for secrets, task and alloc directories.
        containerConfig.SecretsDirSrc = cfg.TaskDir().SecretsDir
        containerConfig.TaskDirSrc = cfg.TaskDir().LocalDir
        containerConfig.AllocDirSrc = cfg.TaskDir().SharedAllocDir

        // Setup destination paths for secrets, task and alloc directories.
        containerConfig.SecretsDirDest = cfg.Env[taskenv.SecretsDir]
        containerConfig.TaskDirDest = cfg.Env[taskenv.TaskLocalDir]
        containerConfig.AllocDirDest = cfg.Env[taskenv.AllocDir]

        containerConfig.ContainerSnapshotName = fmt.Sprintf("%s-snapshot", containerName)
        if cfg.NetworkIsolation != nil && cfg.NetworkIsolation.Path != "" {
            containerConfig.NetworkNamespacePath = cfg.NetworkIsolation.Path
        }

        // memory and cpu are coming from the resources stanza of the nomad job.
        // https://www.nomadproject.io/docs/job-specification/resources
        containerConfig.MemoryLimit = cfg.Resources.NomadResources.Memory.MemoryMB * 1024 * 1024
        containerConfig.MemoryHardLimit = cfg.Resources.NomadResources.Memory.MemoryMaxMB * 1024 * 1024
        containerConfig.CPUShares = cfg.Resources.LinuxResources.CPUShares

        container, err := d.createContainer(&containerConfig, &driverConfig)
        if err != nil {
            return nil, nil, fmt.Errorf("Error in creating container: %v", err)
        }

        d.logger.Info(fmt.Sprintf("Successfully created container with name: %s\n", containerName))
        task, err := d.createTask(container, cfg.StdoutPath, cfg.StderrPath)
        if err != nil {
            return nil, nil, fmt.Errorf("Error in creating task: %v", err)
        }

        d.logger.Info(fmt.Sprintf("Successfully created task with ID: %s\n", task.ID()))

        h := &taskHandle{
            taskConfig:     cfg,
            procState:      drivers.TaskStateRunning,
            startedAt:      time.Now().Round(time.Millisecond),
            logger:         d.logger,
            totalCpuStats:  stats.NewCpuStats(),
            userCpuStats:   stats.NewCpuStats(),
            systemCpuStats: stats.NewCpuStats(),
            container:      container,
            containerName:  containerName,
            task:           task,
        }

        driverState := TaskState{
            StartedAt:     h.startedAt,
            ContainerName: containerName,
            StdoutPath:    cfg.StdoutPath,
            StderrPath:    cfg.StderrPath,
        }

        if err := handle.SetDriverState(&driverState); err != nil {
            return nil, nil, fmt.Errorf("failed to set driver state: %v", err)
        }

        d.tasks.Set(cfg.ID, h)

        go h.run(d.ctxContainerd)
        return handle, nil, nil

        Ok(tonic::Response::new(StartTaskResponse {
            result: 0,
            driver_error_msg: "".to_string(),
            handle: None,
            network_override: None,
        }))
    }

    async fn wait_task(
        &self,
        request: Request<WaitTaskRequest>,
    ) -> Result<Response<WaitTaskResponse>, Status> {
        // log::info!("Received WaitTaskRequest");
        Ok(tonic::Response::new(WaitTaskResponse {
            result: None,
            err: "".to_string(),
        }))
    }

    async fn stop_task(
        &self,
        request: Request<StopTaskRequest>,
    ) -> Result<Response<StopTaskResponse>, Status> {
        // log::info!("Received StopTaskRequest");
        Ok(tonic::Response::new(StopTaskResponse {}))
    }

    async fn destroy_task(
        &self,
        request: Request<DestroyTaskRequest>,
    ) -> Result<Response<DestroyTaskResponse>, Status> {
        // log::info!("Received DestroyTaskRequest");
        Ok(tonic::Response::new(DestroyTaskResponse {}))
    }

    async fn inspect_task(
        &self,
        request: Request<InspectTaskRequest>,
    ) -> Result<Response<InspectTaskResponse>, Status> {
        // log::info!("Received InspectTaskRequest");
        Ok(tonic::Response::new(InspectTaskResponse {
            task: None,
            driver: None,
            network_override: None,
        }))
    }

    type TaskStatsStream = Pin<
        Box<
            dyn futures_core::Stream<Item = Result<TaskStatsResponse, Status>>
                + Send
                + Sync
                + 'static,
        >,
    >;

    async fn task_stats(
        &self,
        request: Request<TaskStatsRequest>,
    ) -> Result<Response<Self::TaskStatsStream>, Status> {
        // log::info!("Received TaskStatsRequest");
        let (sender, receiver) = tokio::sync::mpsc::channel(4);

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(receiver),
        )))
    }

    type TaskEventsStream = Pin<
        Box<
            dyn futures_core::Stream<Item = Result<DriverTaskEvent, Status>>
                + Send
                + Sync
                + 'static,
        >,
    >;

    async fn task_events(
        &self,
        request: Request<TaskEventsRequest>,
    ) -> Result<Response<Self::TaskEventsStream>, Status> {
        // log::info!("Received TaskEventsRequest");
        let (sender, receiver) = tokio::sync::mpsc::channel(4);

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(receiver),
        )))
    }

    async fn signal_task(
        &self,
        request: Request<SignalTaskRequest>,
    ) -> Result<Response<SignalTaskResponse>, Status> {
        // log::info!("Received SignalTaskRequest");
        Ok(tonic::Response::new(SignalTaskResponse {}))
    }

    async fn exec_task(
        &self,
        request: Request<ExecTaskRequest>,
    ) -> Result<Response<ExecTaskResponse>, Status> {
        // TODO: This needs to handle replacing a task vs. launching a new one.
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

        // TODO: This isn't right. Just here to compile.
        Ok(Response::new(task_handle.exec(&ExecOptions {
            command: vec![],
            tty: false,
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
            stderr: std::io::stderr(),
        })))
    }

    type ExecTaskStreamingStream = Pin<
        Box<
            dyn futures_core::Stream<Item = Result<ExecTaskStreamingResponse, Status>>
                + Send
                + Sync
                + 'static,
        >,
    >;

    async fn exec_task_streaming(
        &self,
        request: Request<Streaming<ExecTaskStreamingRequest>>,
    ) -> Result<Response<Self::ExecTaskStreamingStream>, Status> {
        // log::info!("Received ExecTaskStreamingRequest");
        let (sender, receiver) = tokio::sync::mpsc::channel(4);

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(receiver),
        )))
    }

    async fn create_network(
        &self,
        request: Request<CreateNetworkRequest>,
    ) -> Result<Response<CreateNetworkResponse>, Status> {
        // log::info!("Received CreateNetworkRequest");
        Ok(tonic::Response::new(CreateNetworkResponse {
            isolation_spec: None,
            created: false,
        }))
    }

    async fn destroy_network(
        &self,
        request: Request<DestroyNetworkRequest>,
    ) -> Result<Response<DestroyNetworkResponse>, Status> {
        // log::info!("Received DestroyNetworkRequest");
        Ok(tonic::Response::new(DestroyNetworkResponse {}))
    }
}

// Adapted from https://tldp.org/LDP/abs/html/exitcodes.html
pub enum ExitCodes {
    Error = 1,             // Catchall for general errors
    BuiltinMisuse = 2,     // Misuse of shell builtins (according to Bash documentation)
    CannotExecute = 126,   // Command invoked cannot execute
    CommandNotFound = 127, // “command not found”
    InvalidArgument = 128, // Invalid argument to exit
    Exited = 130,          // Terminated by Control-C
    OutOfRange = 255,      // Exit status out of range
}

fn exit_result(exit_code: ExitCodes) -> Option<ExitResult> {
    Some(ExitResult {
        exit_code: exit_code as i32,
        signal: 0,
        oom_killed: false,
    })
}
