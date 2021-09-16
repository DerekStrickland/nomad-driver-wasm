#![allow(unused_variables)]
use log;
use std::pin::Pin;
use tonic::{Request, Response, Status, Streaming};

use crate::proto::drivers::{ExecTaskStreamingResponse, TaskConfigSchemaRequest, TaskConfigSchemaResponse, CapabilitiesRequest, CapabilitiesResponse, FingerprintRequest, RecoverTaskRequest, RecoverTaskResponse, StartTaskRequest, StartTaskResponse, WaitTaskRequest, WaitTaskResponse, StopTaskRequest, StopTaskResponse, DestroyTaskRequest, DestroyTaskResponse, InspectTaskRequest, InspectTaskResponse, TaskStatsRequest, TaskEventsRequest, SignalTaskRequest, SignalTaskResponse, ExecTaskRequest, ExecTaskResponse, ExecTaskStreamingRequest, CreateNetworkRequest, CreateNetworkResponse, DestroyNetworkRequest, DestroyNetworkResponse, DriverTaskEvent, TaskStatsResponse, FingerprintResponse};
use crate::proto::drivers::driver_server::{Driver};

#[derive(Default)]
pub struct WasmtimeDriver {}

#[tonic::async_trait]
impl Driver for WasmtimeDriver {
    async fn task_config_schema(&self, request: Request<TaskConfigSchemaRequest>) -> Result<Response<TaskConfigSchemaResponse>, Status> {
        log::info!("Received TaskConfigSchemaRequest");
        Ok(tonic::Response::new(TaskConfigSchemaResponse{ spec: None }))
    }

    async fn capabilities(&self, request: Request<CapabilitiesRequest>) -> Result<Response<CapabilitiesResponse>, Status> {
        log::info!("Received CapabilitiesRequest");
        Ok(tonic::Response::new(CapabilitiesResponse{ capabilities: None }))
    }

    type FingerprintStream = Pin<Box<dyn futures_core::Stream<Item = Result<FingerprintResponse, Status>> + Send + Sync + 'static>>;

    async fn fingerprint(&self, request: Request<FingerprintRequest>) -> Result<Response<Self::FingerprintStream>, Status> {
        log::info!("Received FingerprintRequest");
        let (sender, receiver) = tokio::sync::mpsc::channel(4);

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(receiver),
        )))
    }

    async fn recover_task(&self, request: Request<RecoverTaskRequest>) -> Result<Response<RecoverTaskResponse>, Status> {
        log::info!("Received RecoverTaskRequest");
        Ok(tonic::Response::new(RecoverTaskResponse{}))
    }

    async fn start_task(&self, request: Request<StartTaskRequest>) -> Result<Response<StartTaskResponse>, Status> {
        log::info!("Received StartTaskRequest");
        Ok(tonic::Response::new(StartTaskResponse{
            result: 0,
            driver_error_msg: "".to_string(),
            handle: None,
            network_override: None
        }))
    }

    async fn wait_task(&self, request: Request<WaitTaskRequest>) -> Result<Response<WaitTaskResponse>, Status> {
        log::info!("Received WaitTaskRequest");
        Ok(tonic::Response::new(WaitTaskResponse{ result: None, err: "".to_string() }))
    }

    async fn stop_task(&self, request: Request<StopTaskRequest>) -> Result<Response<StopTaskResponse>, Status> {
        log::info!("Received StopTaskRequest");
        Ok(tonic::Response::new(StopTaskResponse{}))
    }

    async fn destroy_task(&self, request: Request<DestroyTaskRequest>) -> Result<Response<DestroyTaskResponse>, Status> {
        log::info!("Received DestroyTaskRequest");
        Ok(tonic::Response::new(DestroyTaskResponse{}))
    }

    async fn inspect_task(&self, request: Request<InspectTaskRequest>) -> Result<Response<InspectTaskResponse>, Status> {
        log::info!("Received InspectTaskRequest");
        Ok(tonic::Response::new(InspectTaskResponse{
            task: None,
            driver: None,
            network_override: None
        }))
    }

    type TaskStatsStream = Pin<Box<dyn futures_core::Stream<Item = Result<TaskStatsResponse, Status>> + Send + Sync + 'static>>;

    async fn task_stats(&self, request: Request<TaskStatsRequest>) -> Result<Response<Self::TaskStatsStream>, Status> {
        log::info!("Received TaskStatsRequest");
        let (sender, receiver) = tokio::sync::mpsc::channel(4);

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(receiver),
        )))
    }

    type TaskEventsStream = Pin<Box<dyn futures_core::Stream<Item = Result<DriverTaskEvent, Status>> + Send + Sync + 'static>>;

    async fn task_events(&self, request: Request<TaskEventsRequest>) -> Result<Response<Self::TaskEventsStream>, Status> {
        log::info!("Received TaskEventsRequest");
        let (sender, receiver) = tokio::sync::mpsc::channel(4);

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(receiver),
        )))
    }

    async fn signal_task(&self, request: Request<SignalTaskRequest>) -> Result<Response<SignalTaskResponse>, Status> {
        log::info!("Received SignalTaskRequest");
        Ok(tonic::Response::new(SignalTaskResponse{}))
    }

    async fn exec_task(&self, request: Request<ExecTaskRequest>) -> Result<Response<ExecTaskResponse>, Status> {
        log::info!("Received ExecTaskRequest");
        Ok(tonic::Response::new(ExecTaskResponse{
            stdout: vec![],
            stderr: vec![],
            result: None
        }))
    }

    type ExecTaskStreamingStream = Pin<Box<dyn futures_core::Stream<Item = Result<ExecTaskStreamingResponse, Status>> + Send + Sync + 'static>>;

    async fn exec_task_streaming(&self, request: Request<Streaming<ExecTaskStreamingRequest>>) -> Result<Response<Self::ExecTaskStreamingStream>, Status> {
        log::info!("Received ExecTaskStreamingRequest");
        let (sender, receiver) = tokio::sync::mpsc::channel(4);

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(receiver),
        )))
    }

    async fn create_network(&self, request: Request<CreateNetworkRequest>) -> Result<Response<CreateNetworkResponse>, Status> {
        log::info!("Received CreateNetworkRequest");
        Ok(tonic::Response::new(CreateNetworkResponse{
            isolation_spec: None,
            created: false
        }))
    }

    async fn destroy_network(&self, request: Request<DestroyNetworkRequest>) -> Result<Response<DestroyNetworkResponse>, Status> {
        log::info!("Received DestroyNetworkRequest");
        Ok(tonic::Response::new(DestroyNetworkResponse{}))
    }
}