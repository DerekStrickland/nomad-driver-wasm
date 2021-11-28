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

pub struct TaskController {
    pub registry_url: String,
    pub tasks: Option<TaskSet<Task>>,
}

impl TaskController {
    pub fn default(registry_url: String) -> TaskController {
        TaskController {
            registry_url,
            tasks: Some(TaskSet::new(Task {
                task_id: "".to_string(),
                handle: None,
                config_schema: Spec { block: None },
                config: None,
                status: None,
                driver_status: None,
                timeout: None,
                network_override: None,
                start_result: None,
                exit_result: None,
                error_msg: "".to_string(),
            })),
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

    pub fn exec(request: ExecTaskRequest) -> ExecTaskResponse {
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

struct Task {
    pub task_id: String,
    pub handle: Option<TaskHandle>,
    pub config_schema: Spec,
    pub config: Option<TaskConfig>,
    pub status: Option<TaskStatus>,
    pub driver_status: Option<TaskDriverStatus>,
    pub timeout: Option<Duration>,
    pub network_override: Option<NetworkOverride>,
    pub start_result: Option<start_task::Result>,
    pub exit_result: Option<ExitResult>,
    pub error_msg: String,
}

impl Task {
    pub fn get_stats(&self) -> TaskStatsResponse {
        TaskStatsResponse { stats: None }
    }

    pub fn signal(&self, request: SignalTaskRequest) -> SignalTaskResponse {
        SignalTaskResponse {}
    }
}

struct TaskSet<Iter> {
    iter: Iter,
    i: usize,
}

impl<Iter> TaskSet<Iter> {
    pub fn new(iter: Iter) -> Self {
        TaskSet { iter, i: 0 }
    }
}

impl<Iter> Iterator for TaskSet<Iter>
where
    Iter: Iterator,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
