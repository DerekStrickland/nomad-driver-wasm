#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TaskConfigSchemaRequest {}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TaskConfigSchemaResponse {
    /// Spec is the configuration schema for the job driver config stanza
    #[prost(message, optional, tag = "1")]
    pub spec: ::core::option::Option<crate::proto::hclspec::Spec>,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CapabilitiesRequest {}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CapabilitiesResponse {
    /// Capabilities provides a way for the driver to denote if it implements
    /// non-core RPCs. Some Driver service RPCs expose additional information
    /// or functionality outside of the core task management functions. These
    /// RPCs are only implemented if the driver sets the corresponding capability.
    #[prost(message, optional, tag = "1")]
    pub capabilities: ::core::option::Option<DriverCapabilities>,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FingerprintRequest {}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FingerprintResponse {
    /// Attributes are key/value pairs that annotate the nomad client and can be
    /// used in scheduling constraints and affinities.
    #[prost(map = "string, message", tag = "1")]
    pub attributes: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        crate::proto::structs::Attribute,
    >,
    /// Health is used to determine the state of the health the driver is in.
    /// Health can be one of the following states:
    ///  * UNDETECTED: driver dependencies are not met and the driver can not start
    ///  * UNHEALTHY: driver dependencies are met but the driver is unable to
    ///      perform operations due to some other problem
    ///  * HEALTHY: driver is able to perform all operations
    #[prost(enumeration = "fingerprint_response::HealthState", tag = "2")]
    pub health: i32,
    /// HealthDescription is a human readable message describing the current
    /// state of driver health
    #[prost(string, tag = "3")]
    pub health_description: ::prost::alloc::string::String,
}
/// Nested message and enum types in `FingerprintResponse`.
pub mod fingerprint_response {
    #[derive(
        serde::Deserialize,
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration,
    )]
    #[repr(i32)]
    pub enum HealthState {
        Undetected = 0,
        Unhealthy = 1,
        Healthy = 2,
    }
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RecoverTaskRequest {
    /// TaskId is the ID of the target task
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    /// Handle is the TaskHandle returned from StartTask
    #[prost(message, optional, tag = "2")]
    pub handle: ::core::option::Option<TaskHandle>,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RecoverTaskResponse {}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct StartTaskRequest {
    /// Task configuration to launch
    #[prost(message, optional, tag = "1")]
    pub task: ::core::option::Option<TaskConfig>,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct StartTaskResponse {
    /// Result is set depending on the type of error that occurred while starting
    /// a task:
    ///
    ///   * SUCCESS: No error occurred, handle is set
    ///   * RETRY: An error occurred, but is recoverable and the RPC should be retried
    ///   * FATAL: A fatal error occurred and is not likely to succeed if retried
    ///
    /// If Result is not successful, the DriverErrorMsg will be set.
    #[prost(enumeration = "start_task_response::Result", tag = "1")]
    pub result: i32,
    /// DriverErrorMsg is set if an error occurred
    #[prost(string, tag = "2")]
    pub driver_error_msg: ::prost::alloc::string::String,
    /// Handle is opaque to the client, but must be stored in order to recover
    /// the task.
    #[prost(message, optional, tag = "3")]
    pub handle: ::core::option::Option<TaskHandle>,
    /// NetworkOverride is set if the driver sets network settings and the service ip/port
    /// needs to be set differently.
    #[prost(message, optional, tag = "4")]
    pub network_override: ::core::option::Option<NetworkOverride>,
}
/// Nested message and enum types in `StartTaskResponse`.
pub mod start_task_response {
    #[derive(
        serde::Deserialize,
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration,
    )]
    #[repr(i32)]
    pub enum Result {
        Success = 0,
        Retry = 1,
        Fatal = 2,
    }
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct WaitTaskRequest {
    /// TaskId is the ID of the target task
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct WaitTaskResponse {
    /// Result is the exit status of the task
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<ExitResult>,
    /// Err is set if any driver error occurred while waiting for the task
    #[prost(string, tag = "2")]
    pub err: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct StopTaskRequest {
    /// TaskId is the ID of the target task
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    /// Timeout defines the amount of time to wait before forcefully killing
    /// the task. For example, on Unix clients, this means sending a SIGKILL to
    /// the process.
    #[prost(message, optional, tag = "2")]
    pub timeout:
        ::core::option::Option<super::super::super::super::super::google::protobuf::Duration>,
    /// Signal can be set to override the Task's configured shutdown signal
    #[prost(string, tag = "3")]
    pub signal: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct StopTaskResponse {}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DestroyTaskRequest {
    /// TaskId is the ID of the target task
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    /// Force destroys the task even if it is still in a running state
    #[prost(bool, tag = "2")]
    pub force: bool,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DestroyTaskResponse {}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct InspectTaskRequest {
    /// TaskId is the ID of the target task
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct InspectTaskResponse {
    /// Task details
    #[prost(message, optional, tag = "1")]
    pub task: ::core::option::Option<TaskStatus>,
    /// Driver details for task
    #[prost(message, optional, tag = "2")]
    pub driver: ::core::option::Option<TaskDriverStatus>,
    /// NetworkOverride info if set
    #[prost(message, optional, tag = "3")]
    pub network_override: ::core::option::Option<NetworkOverride>,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TaskStatsRequest {
    /// TaskId is the ID of the target task
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    /// CollectionInterval is the interval at which to stream stats to the caller
    #[prost(message, optional, tag = "2")]
    pub collection_interval:
        ::core::option::Option<super::super::super::super::super::google::protobuf::Duration>,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TaskStatsResponse {
    /// Stats for the task
    #[prost(message, optional, tag = "1")]
    pub stats: ::core::option::Option<TaskStats>,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TaskEventsRequest {}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SignalTaskRequest {
    /// TaskId is the ID of the target task
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    /// Signal is the operating system signal to send to the task. Ex: SIGHUP
    #[prost(string, tag = "2")]
    pub signal: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SignalTaskResponse {}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ExecTaskRequest {
    /// TaskId is the ID of the target task
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    /// Command is the command to execute in the task environment
    #[prost(string, repeated, tag = "2")]
    pub command: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Timeout is the amount of time to wait for the command to stop.
    /// Defaults to 0 (run forever)
    #[prost(message, optional, tag = "3")]
    pub timeout:
        ::core::option::Option<super::super::super::super::super::google::protobuf::Duration>,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ExecTaskResponse {
    /// Stdout from the exec
    #[prost(bytes = "vec", tag = "1")]
    pub stdout: ::prost::alloc::vec::Vec<u8>,
    /// Stderr from the exec
    #[prost(bytes = "vec", tag = "2")]
    pub stderr: ::prost::alloc::vec::Vec<u8>,
    /// Result from the exec
    #[prost(message, optional, tag = "3")]
    pub result: ::core::option::Option<ExitResult>,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ExecTaskStreamingIoOperation {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "2")]
    pub close: bool,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ExecTaskStreamingRequest {
    #[prost(message, optional, tag = "1")]
    pub setup: ::core::option::Option<exec_task_streaming_request::Setup>,
    #[prost(message, optional, tag = "2")]
    pub tty_size: ::core::option::Option<exec_task_streaming_request::TerminalSize>,
    #[prost(message, optional, tag = "3")]
    pub stdin: ::core::option::Option<ExecTaskStreamingIoOperation>,
}
/// Nested message and enum types in `ExecTaskStreamingRequest`.
pub mod exec_task_streaming_request {
    #[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct Setup {
        #[prost(string, tag = "1")]
        pub task_id: ::prost::alloc::string::String,
        #[prost(string, repeated, tag = "2")]
        pub command: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(bool, tag = "3")]
        pub tty: bool,
    }
    #[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct TerminalSize {
        #[prost(int32, tag = "1")]
        pub height: i32,
        #[prost(int32, tag = "2")]
        pub width: i32,
    }
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ExecTaskStreamingResponse {
    #[prost(message, optional, tag = "1")]
    pub stdout: ::core::option::Option<ExecTaskStreamingIoOperation>,
    #[prost(message, optional, tag = "2")]
    pub stderr: ::core::option::Option<ExecTaskStreamingIoOperation>,
    #[prost(bool, tag = "3")]
    pub exited: bool,
    #[prost(message, optional, tag = "4")]
    pub result: ::core::option::Option<ExitResult>,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CreateNetworkRequest {
    /// AllocID of the allocation the network is associated with
    #[prost(string, tag = "1")]
    pub alloc_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CreateNetworkResponse {
    #[prost(message, optional, tag = "1")]
    pub isolation_spec: ::core::option::Option<NetworkIsolationSpec>,
    /// created indicates that the network namespace is newly created
    /// as a result of this request. if false, the NetworkIsolationSpec
    /// value returned is an existing spec.
    #[prost(bool, tag = "2")]
    pub created: bool,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DestroyNetworkRequest {
    /// AllocID of the allocation the network is associated with
    #[prost(string, tag = "1")]
    pub alloc_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub isolation_spec: ::core::option::Option<NetworkIsolationSpec>,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DestroyNetworkResponse {}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DriverCapabilities {
    /// SendSignals indicates that the driver can send process signals (ex. SIGUSR1)
    /// to the task.
    #[prost(bool, tag = "1")]
    pub send_signals: bool,
    /// Exec indicates that the driver supports executing arbitrary commands
    /// in the task's execution environment.
    #[prost(bool, tag = "2")]
    pub exec: bool,
    /// FsIsolation indicates what kind of filesystem isolation a driver supports.
    #[prost(enumeration = "driver_capabilities::FsIsolation", tag = "3")]
    pub fs_isolation: i32,
    #[prost(
        enumeration = "network_isolation_spec::NetworkIsolationMode",
        repeated,
        tag = "4"
    )]
    pub network_isolation_modes: ::prost::alloc::vec::Vec<i32>,
    #[prost(bool, tag = "5")]
    pub must_create_network: bool,
    /// MountConfigs indicates whether the driver supports mount configurations.
    #[prost(enumeration = "driver_capabilities::MountConfigs", tag = "6")]
    pub mount_configs: i32,
    /// remote_tasks indicates whether the driver executes tasks remotely such
    /// on cloud runtimes like AWS ECS.
    #[prost(bool, tag = "7")]
    pub remote_tasks: bool,
}
/// Nested message and enum types in `DriverCapabilities`.
pub mod driver_capabilities {
    #[derive(
        serde::Deserialize,
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration,
    )]
    #[repr(i32)]
    pub enum FsIsolation {
        None = 0,
        Chroot = 1,
        Image = 2,
    }
    #[derive(
        serde::Deserialize,
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration,
    )]
    #[repr(i32)]
    pub enum MountConfigs {
        /// treated as ANY_MOUNTS for backwards compatibility
        UnknownMounts = 0,
        NoMounts = 1,
    }
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct NetworkIsolationSpec {
    #[prost(
        enumeration = "network_isolation_spec::NetworkIsolationMode",
        tag = "1"
    )]
    pub mode: i32,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "3")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub hosts_config: ::core::option::Option<HostsConfig>,
}
/// Nested message and enum types in `NetworkIsolationSpec`.
pub mod network_isolation_spec {
    #[derive(
        serde::Deserialize,
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration,
    )]
    #[repr(i32)]
    pub enum NetworkIsolationMode {
        Host = 0,
        Group = 1,
        Task = 2,
        None = 3,
    }
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct HostsConfig {
    #[prost(string, tag = "1")]
    pub hostname: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DnsConfig {
    #[prost(string, repeated, tag = "1")]
    pub servers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub searches: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub options: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TaskConfig {
    /// Id of the task, recommended to the globally unique, must be unique to the driver.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Name of the task
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// MsgpackDriverConfig is the encoded driver configuation of the task
    #[prost(bytes = "vec", tag = "3")]
    pub msgpack_driver_config: ::prost::alloc::vec::Vec<u8>,
    /// Env is the a set of key/value pairs to be set as environment variables
    #[prost(map = "string, string", tag = "4")]
    pub env:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// DeviceEnv is the set of environment variables that are defined by device
    /// plugins. This allows the driver to differentiate environment variables
    /// set by the device plugins and those by the user. When populating the
    /// task's environment env should be used.
    #[prost(map = "string, string", tag = "5")]
    pub device_env:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Resources defines the resources to isolate
    #[prost(message, optional, tag = "6")]
    pub resources: ::core::option::Option<Resources>,
    /// Mounts is a list of targets to bind mount into the task directory
    #[prost(message, repeated, tag = "7")]
    pub mounts: ::prost::alloc::vec::Vec<Mount>,
    /// Devices is a list of system devices to mount into the task's execution
    /// environment.
    #[prost(message, repeated, tag = "8")]
    pub devices: ::prost::alloc::vec::Vec<Device>,
    /// User defines the operating system user the tasks should run as
    #[prost(string, tag = "9")]
    pub user: ::prost::alloc::string::String,
    /// AllocDir is the directory on the host where the allocation directory
    /// exists.
    #[prost(string, tag = "10")]
    pub alloc_dir: ::prost::alloc::string::String,
    /// StdoutPath is the path to the file to open and write task stdout to
    #[prost(string, tag = "11")]
    pub stdout_path: ::prost::alloc::string::String,
    /// StderrPath is the path to the file to open and write task stderr to
    #[prost(string, tag = "12")]
    pub stderr_path: ::prost::alloc::string::String,
    /// TaskGroupName is the name of the task group which this task is a member of
    #[prost(string, tag = "13")]
    pub task_group_name: ::prost::alloc::string::String,
    /// JobName is the name of the job of which this task is part of
    #[prost(string, tag = "14")]
    pub job_name: ::prost::alloc::string::String,
    /// AllocId is the ID of the associated allocation
    #[prost(string, tag = "15")]
    pub alloc_id: ::prost::alloc::string::String,
    /// NetworkIsolationSpec specifies the configuration for the network namespace
    /// to use for the task. *Only supported on Linux
    #[prost(message, optional, tag = "16")]
    pub network_isolation_spec: ::core::option::Option<NetworkIsolationSpec>,
    /// DNSConfig is the configuration for task DNS resolvers and other options
    #[prost(message, optional, tag = "17")]
    pub dns: ::core::option::Option<DnsConfig>,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Resources {
    /// AllocatedResources are the resources set for the task
    #[prost(message, optional, tag = "1")]
    pub allocated_resources: ::core::option::Option<AllocatedTaskResources>,
    /// LinuxResources are the computed values to set for specific Linux features
    #[prost(message, optional, tag = "2")]
    pub linux_resources: ::core::option::Option<LinuxResources>,
    /// Ports are the allocated port mappings for the allocation.
    /// A task may use these to manually configure port mapping if shared network namespaces aren't being used.
    #[prost(message, repeated, tag = "3")]
    pub ports: ::prost::alloc::vec::Vec<PortMapping>,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AllocatedTaskResources {
    #[prost(message, optional, tag = "1")]
    pub cpu: ::core::option::Option<AllocatedCpuResources>,
    #[prost(message, optional, tag = "2")]
    pub memory: ::core::option::Option<AllocatedMemoryResources>,
    #[prost(message, repeated, tag = "5")]
    pub networks: ::prost::alloc::vec::Vec<NetworkResource>,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AllocatedCpuResources {
    #[prost(int64, tag = "1")]
    pub cpu_shares: i64,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AllocatedMemoryResources {
    #[prost(int64, tag = "2")]
    pub memory_mb: i64,
    #[prost(int64, tag = "3")]
    pub memory_max_mb: i64,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct NetworkResource {
    #[prost(string, tag = "1")]
    pub device: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub cidr: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub ip: ::prost::alloc::string::String,
    #[prost(int32, tag = "4")]
    pub mbits: i32,
    #[prost(message, repeated, tag = "5")]
    pub reserved_ports: ::prost::alloc::vec::Vec<NetworkPort>,
    #[prost(message, repeated, tag = "6")]
    pub dynamic_ports: ::prost::alloc::vec::Vec<NetworkPort>,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct NetworkPort {
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub value: i32,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PortMapping {
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub value: i32,
    #[prost(int32, tag = "3")]
    pub to: i32,
    #[prost(string, tag = "4")]
    pub host_ip: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LinuxResources {
    /// CPU CFS (Completely Fair Scheduler) period. Default: 0 (not specified)
    #[prost(int64, tag = "1")]
    pub cpu_period: i64,
    /// CPU CFS (Completely Fair Scheduler) quota. Default: 0 (not specified)
    #[prost(int64, tag = "2")]
    pub cpu_quota: i64,
    /// CPU shares (relative weight vs. other containers). Default: 0 (not specified)
    #[prost(int64, tag = "3")]
    pub cpu_shares: i64,
    /// Memory limit in bytes. Default: 0 (not specified)
    #[prost(int64, tag = "4")]
    pub memory_limit_bytes: i64,
    /// OOMScoreAdj adjusts the oom-killer score. Default: 0 (not specified)
    #[prost(int64, tag = "5")]
    pub oom_score_adj: i64,
    /// CpusetCpus constrains the allowed set of logical CPUs. Default: "" (not specified)
    /// This field exists to support drivers which can't set a cgroup path.
    #[prost(string, tag = "6")]
    pub cpuset_cpus: ::prost::alloc::string::String,
    /// CpusetCgroup is the path to the cpuset cgroup managed by the client
    #[prost(string, tag = "9")]
    pub cpuset_cgroup: ::prost::alloc::string::String,
    /// PercentTicks is a compatibility option for docker and should not be used
    /// buf:lint:ignore FIELD_LOWER_SNAKE_CASE
    #[prost(double, tag = "8")]
    pub percent_ticks: f64,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Mount {
    /// TaskPath is the file path within the task directory to mount to
    #[prost(string, tag = "1")]
    pub task_path: ::prost::alloc::string::String,
    /// HostPath is the file path on the host to mount from
    #[prost(string, tag = "2")]
    pub host_path: ::prost::alloc::string::String,
    /// Readonly if set true, mounts the path in readonly mode
    #[prost(bool, tag = "3")]
    pub readonly: bool,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Device {
    /// TaskPath is the file path within the task to mount the device to
    #[prost(string, tag = "1")]
    pub task_path: ::prost::alloc::string::String,
    /// HostPath is the path on the host to the source device
    #[prost(string, tag = "2")]
    pub host_path: ::prost::alloc::string::String,
    /// CgroupPermissions defines the Cgroup permissions of the device.
    /// One or more of the following options can be set:
    ///  * r - allows the task to read from the specified device.
    ///  * w - allows the task to write to the specified device.
    ///  * m - allows the task to create device files that do not yet exist.
    ///
    /// Example: "rw"
    #[prost(string, tag = "3")]
    pub cgroup_permissions: ::prost::alloc::string::String,
}
/// TaskHandle is created when starting a task and is used to recover task
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TaskHandle {
    /// Version is used by the driver to version the DriverState schema.
    /// Version 0 is reserved by Nomad and should not be used.
    #[prost(int32, tag = "1")]
    pub version: i32,
    /// Config is the TaskConfig for the task
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<TaskConfig>,
    /// State is the state of the task's execution
    #[prost(enumeration = "TaskState", tag = "3")]
    pub state: i32,
    /// DriverState is the encoded state for the specific driver
    #[prost(bytes = "vec", tag = "4")]
    pub driver_state: ::prost::alloc::vec::Vec<u8>,
}
/// NetworkOverride contains network settings which the driver may override
/// for the task, such as when the driver is setting up the task's network.
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct NetworkOverride {
    /// PortMap can be set to replace ports with driver-specific mappings
    #[prost(map = "string, int32", tag = "1")]
    pub port_map: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
    /// Addr is the IP address for the task created by the driver
    #[prost(string, tag = "2")]
    pub addr: ::prost::alloc::string::String,
    /// AutoAdvertise indicates whether the driver thinks services that choose
    /// to auto_advertise_addresses should use this IP instead of the host's.
    #[prost(bool, tag = "3")]
    pub auto_advertise: bool,
}
/// ExitResult contains information about the exit status of a task
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ExitResult {
    /// ExitCode returned from the task on exit
    #[prost(int32, tag = "1")]
    pub exit_code: i32,
    /// Signal is set if a signal was sent to the task
    #[prost(int32, tag = "2")]
    pub signal: i32,
    /// OomKilled is true if the task exited as a result of the OOM Killer
    #[prost(bool, tag = "3")]
    pub oom_killed: bool,
}
/// TaskStatus includes information of a specific task
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TaskStatus {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// State is the state of the task's execution
    #[prost(enumeration = "TaskState", tag = "3")]
    pub state: i32,
    /// StartedAt is the timestamp when the task was started
    #[prost(message, optional, tag = "4")]
    pub started_at:
        ::core::option::Option<super::super::super::super::super::google::protobuf::Timestamp>,
    /// CompletedAt is the timestamp when the task exited.
    /// If the task is still running, CompletedAt will not be set
    #[prost(message, optional, tag = "5")]
    pub completed_at:
        ::core::option::Option<super::super::super::super::super::google::protobuf::Timestamp>,
    /// Result is set when CompletedAt is set.
    #[prost(message, optional, tag = "6")]
    pub result: ::core::option::Option<ExitResult>,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TaskDriverStatus {
    /// Attributes is a set of string/string key value pairs specific to the
    /// implementing driver
    #[prost(map = "string, string", tag = "1")]
    pub attributes:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TaskStats {
    /// Id of the task
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Timestamp for which the stats were collected
    #[prost(message, optional, tag = "2")]
    pub timestamp:
        ::core::option::Option<super::super::super::super::super::google::protobuf::Timestamp>,
    /// AggResourceUsage is the aggreate usage of all processes
    #[prost(message, optional, tag = "3")]
    pub agg_resource_usage: ::core::option::Option<TaskResourceUsage>,
    /// ResourceUsageByPid breaks the usage stats by process
    #[prost(map = "string, message", tag = "4")]
    pub resource_usage_by_pid:
        ::std::collections::HashMap<::prost::alloc::string::String, TaskResourceUsage>,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TaskResourceUsage {
    /// CPU usage stats
    #[prost(message, optional, tag = "1")]
    pub cpu: ::core::option::Option<CpuUsage>,
    /// Memory usage stats
    #[prost(message, optional, tag = "2")]
    pub memory: ::core::option::Option<MemoryUsage>,
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CpuUsage {
    #[prost(double, tag = "1")]
    pub system_mode: f64,
    #[prost(double, tag = "2")]
    pub user_mode: f64,
    #[prost(double, tag = "3")]
    pub total_ticks: f64,
    #[prost(uint64, tag = "4")]
    pub throttled_periods: u64,
    #[prost(uint64, tag = "5")]
    pub throttled_time: u64,
    #[prost(double, tag = "6")]
    pub percent: f64,
    /// MeasuredFields indicates which fields were actually sampled
    #[prost(enumeration = "cpu_usage::Fields", repeated, tag = "7")]
    pub measured_fields: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `CPUUsage`.
pub mod cpu_usage {
    #[derive(
        serde::Deserialize,
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration,
    )]
    #[repr(i32)]
    pub enum Fields {
        SystemMode = 0,
        UserMode = 1,
        TotalTicks = 2,
        ThrottledPeriods = 3,
        ThrottledTime = 4,
        Percent = 5,
    }
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MemoryUsage {
    #[prost(uint64, tag = "1")]
    pub rss: u64,
    #[prost(uint64, tag = "2")]
    pub cache: u64,
    #[prost(uint64, tag = "3")]
    pub max_usage: u64,
    #[prost(uint64, tag = "4")]
    pub kernel_usage: u64,
    #[prost(uint64, tag = "5")]
    pub kernel_max_usage: u64,
    #[prost(uint64, tag = "7")]
    pub usage: u64,
    #[prost(uint64, tag = "8")]
    pub swap: u64,
    /// MeasuredFields indicates which fields were actually sampled
    #[prost(enumeration = "memory_usage::Fields", repeated, tag = "6")]
    pub measured_fields: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `MemoryUsage`.
pub mod memory_usage {
    #[derive(
        serde::Deserialize,
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration,
    )]
    #[repr(i32)]
    pub enum Fields {
        Rss = 0,
        Cache = 1,
        MaxUsage = 2,
        KernelUsage = 3,
        KernelMaxUsage = 4,
        Usage = 5,
        Swap = 6,
    }
}
#[derive(serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DriverTaskEvent {
    /// TaskId is the id of the task for the event
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    /// AllocId of the task for the event
    #[prost(string, tag = "2")]
    pub alloc_id: ::prost::alloc::string::String,
    /// TaskName is the name of the task for the event
    #[prost(string, tag = "3")]
    pub task_name: ::prost::alloc::string::String,
    /// Timestamp when the event occurred
    #[prost(message, optional, tag = "4")]
    pub timestamp:
        ::core::option::Option<super::super::super::super::super::google::protobuf::Timestamp>,
    /// Message is the body of the event
    #[prost(string, tag = "5")]
    pub message: ::prost::alloc::string::String,
    /// Annotations allows for additional key/value data to be sent along with the event
    #[prost(map = "string, string", tag = "6")]
    pub annotations:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(
    serde::Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum TaskState {
    Unknown = 0,
    Running = 1,
    Exited = 2,
}
#[doc = r" Generated client implementations."]
pub mod driver_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Driver service defines RPCs used to communicate with a nomad runtime driver."]
    #[doc = " Some rpcs may not be implemented by the driver based on it's capabilities."]
    #[derive(Debug, Clone)]
    pub struct DriverClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DriverClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DriverClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DriverClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            DriverClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " TaskConfigSchema returns the schema for parsing the driver"]
        #[doc = " configuration of a task."]
        pub async fn task_config_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::TaskConfigSchemaRequest>,
        ) -> Result<tonic::Response<super::TaskConfigSchemaResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/hashicorp.nomad.plugins.drivers.proto.Driver/TaskConfigSchema",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Capabilities returns a set of features which the driver implements. Some"]
        #[doc = " RPCs are not possible to implement on some runtimes, this allows the"]
        #[doc = " driver to indicate if it doesn't support these RPCs and features."]
        pub async fn capabilities(
            &mut self,
            request: impl tonic::IntoRequest<super::CapabilitiesRequest>,
        ) -> Result<tonic::Response<super::CapabilitiesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/hashicorp.nomad.plugins.drivers.proto.Driver/Capabilities",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Fingerprint starts a stream which emits information about the driver"]
        #[doc = " including whether the driver healthy and able to function in the"]
        #[doc = " existing environment."]
        #[doc = ""]
        #[doc = " The driver should immediately stream a FingerprintResponse when the RPC"]
        #[doc = " is initially called, then send any additional responses if there is a"]
        #[doc = " change in the driver's state."]
        pub async fn fingerprint(
            &mut self,
            request: impl tonic::IntoRequest<super::FingerprintRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::FingerprintResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/hashicorp.nomad.plugins.drivers.proto.Driver/Fingerprint",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " RecoverTask is used when a task has been started but the driver may not"]
        #[doc = " know about it. Such is the case if the driver restarts or is upgraded."]
        pub async fn recover_task(
            &mut self,
            request: impl tonic::IntoRequest<super::RecoverTaskRequest>,
        ) -> Result<tonic::Response<super::RecoverTaskResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/hashicorp.nomad.plugins.drivers.proto.Driver/RecoverTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " StartTask starts and tracks the task on the implemented runtime"]
        pub async fn start_task(
            &mut self,
            request: impl tonic::IntoRequest<super::StartTaskRequest>,
        ) -> Result<tonic::Response<super::StartTaskResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/hashicorp.nomad.plugins.drivers.proto.Driver/StartTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " WaitTask blocks until the given task exits, returning the result of the"]
        #[doc = " task. It may be called after the task has exited, but before the task is"]
        #[doc = " destroyed."]
        pub async fn wait_task(
            &mut self,
            request: impl tonic::IntoRequest<super::WaitTaskRequest>,
        ) -> Result<tonic::Response<super::WaitTaskResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/hashicorp.nomad.plugins.drivers.proto.Driver/WaitTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " StopTask stops a given task by sending the desired signal to the process."]
        #[doc = " If the task does not exit on its own within the given timeout, it will be"]
        #[doc = " forcefully killed."]
        pub async fn stop_task(
            &mut self,
            request: impl tonic::IntoRequest<super::StopTaskRequest>,
        ) -> Result<tonic::Response<super::StopTaskResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/hashicorp.nomad.plugins.drivers.proto.Driver/StopTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " DestroyTask removes the task from the driver's internal state and cleans"]
        #[doc = " up any additional resources created by the driver. It cannot be called"]
        #[doc = " on a running task, unless force is set to true."]
        pub async fn destroy_task(
            &mut self,
            request: impl tonic::IntoRequest<super::DestroyTaskRequest>,
        ) -> Result<tonic::Response<super::DestroyTaskResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/hashicorp.nomad.plugins.drivers.proto.Driver/DestroyTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " InspectTask returns detailed information for the given task"]
        pub async fn inspect_task(
            &mut self,
            request: impl tonic::IntoRequest<super::InspectTaskRequest>,
        ) -> Result<tonic::Response<super::InspectTaskResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/hashicorp.nomad.plugins.drivers.proto.Driver/InspectTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " TaskStats collects and returns runtime metrics for the given task"]
        pub async fn task_stats(
            &mut self,
            request: impl tonic::IntoRequest<super::TaskStatsRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::TaskStatsResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/hashicorp.nomad.plugins.drivers.proto.Driver/TaskStats",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " TaskEvents starts a streaming RPC where all task events emitted by the"]
        #[doc = " driver are streamed to the caller."]
        pub async fn task_events(
            &mut self,
            request: impl tonic::IntoRequest<super::TaskEventsRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::DriverTaskEvent>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/hashicorp.nomad.plugins.drivers.proto.Driver/TaskEvents",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " SignalTask sends a signal to the task"]
        pub async fn signal_task(
            &mut self,
            request: impl tonic::IntoRequest<super::SignalTaskRequest>,
        ) -> Result<tonic::Response<super::SignalTaskResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/hashicorp.nomad.plugins.drivers.proto.Driver/SignalTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " ExecTask executes a command inside the tasks execution context"]
        pub async fn exec_task(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecTaskRequest>,
        ) -> Result<tonic::Response<super::ExecTaskResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/hashicorp.nomad.plugins.drivers.proto.Driver/ExecTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " ExecTaskStreaming executes a command inside the tasks execution context"]
        #[doc = " and streams back results"]
        #[doc = " buf:lint:ignore RPC_REQUEST_RESPONSE_UNIQUE"]
        pub async fn exec_task_streaming(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::ExecTaskStreamingRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ExecTaskStreamingResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/hashicorp.nomad.plugins.drivers.proto.Driver/ExecTaskStreaming",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " CreateNetwork is implemented when the driver needs to create the network"]
        #[doc = " namespace instead of allowing the Nomad client to do."]
        pub async fn create_network(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNetworkRequest>,
        ) -> Result<tonic::Response<super::CreateNetworkResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/hashicorp.nomad.plugins.drivers.proto.Driver/CreateNetwork",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " DestroyNetwork destroys a previously created network. This rpc is only"]
        #[doc = " implemented if the driver needs to manage network namespace creation."]
        pub async fn destroy_network(
            &mut self,
            request: impl tonic::IntoRequest<super::DestroyNetworkRequest>,
        ) -> Result<tonic::Response<super::DestroyNetworkResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/hashicorp.nomad.plugins.drivers.proto.Driver/DestroyNetwork",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod driver_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with DriverServer."]
    #[async_trait]
    pub trait Driver: Send + Sync + 'static {
        #[doc = " TaskConfigSchema returns the schema for parsing the driver"]
        #[doc = " configuration of a task."]
        async fn task_config_schema(
            &self,
            request: tonic::Request<super::TaskConfigSchemaRequest>,
        ) -> Result<tonic::Response<super::TaskConfigSchemaResponse>, tonic::Status>;
        #[doc = " Capabilities returns a set of features which the driver implements. Some"]
        #[doc = " RPCs are not possible to implement on some runtimes, this allows the"]
        #[doc = " driver to indicate if it doesn't support these RPCs and features."]
        async fn capabilities(
            &self,
            request: tonic::Request<super::CapabilitiesRequest>,
        ) -> Result<tonic::Response<super::CapabilitiesResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the Fingerprint method."]
        type FingerprintStream: futures_core::Stream<Item = Result<super::FingerprintResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " Fingerprint starts a stream which emits information about the driver"]
        #[doc = " including whether the driver healthy and able to function in the"]
        #[doc = " existing environment."]
        #[doc = ""]
        #[doc = " The driver should immediately stream a FingerprintResponse when the RPC"]
        #[doc = " is initially called, then send any additional responses if there is a"]
        #[doc = " change in the driver's state."]
        async fn fingerprint(
            &self,
            request: tonic::Request<super::FingerprintRequest>,
        ) -> Result<tonic::Response<Self::FingerprintStream>, tonic::Status>;
        #[doc = " RecoverTask is used when a task has been started but the driver may not"]
        #[doc = " know about it. Such is the case if the driver restarts or is upgraded."]
        async fn recover_task(
            &self,
            request: tonic::Request<super::RecoverTaskRequest>,
        ) -> Result<tonic::Response<super::RecoverTaskResponse>, tonic::Status>;
        #[doc = " StartTask starts and tracks the task on the implemented runtime"]
        async fn start_task(
            &self,
            request: tonic::Request<super::StartTaskRequest>,
        ) -> Result<tonic::Response<super::StartTaskResponse>, tonic::Status>;
        #[doc = " WaitTask blocks until the given task exits, returning the result of the"]
        #[doc = " task. It may be called after the task has exited, but before the task is"]
        #[doc = " destroyed."]
        async fn wait_task(
            &self,
            request: tonic::Request<super::WaitTaskRequest>,
        ) -> Result<tonic::Response<super::WaitTaskResponse>, tonic::Status>;
        #[doc = " StopTask stops a given task by sending the desired signal to the process."]
        #[doc = " If the task does not exit on its own within the given timeout, it will be"]
        #[doc = " forcefully killed."]
        async fn stop_task(
            &self,
            request: tonic::Request<super::StopTaskRequest>,
        ) -> Result<tonic::Response<super::StopTaskResponse>, tonic::Status>;
        #[doc = " DestroyTask removes the task from the driver's internal state and cleans"]
        #[doc = " up any additional resources created by the driver. It cannot be called"]
        #[doc = " on a running task, unless force is set to true."]
        async fn destroy_task(
            &self,
            request: tonic::Request<super::DestroyTaskRequest>,
        ) -> Result<tonic::Response<super::DestroyTaskResponse>, tonic::Status>;
        #[doc = " InspectTask returns detailed information for the given task"]
        async fn inspect_task(
            &self,
            request: tonic::Request<super::InspectTaskRequest>,
        ) -> Result<tonic::Response<super::InspectTaskResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the TaskStats method."]
        type TaskStatsStream: futures_core::Stream<Item = Result<super::TaskStatsResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " TaskStats collects and returns runtime metrics for the given task"]
        async fn task_stats(
            &self,
            request: tonic::Request<super::TaskStatsRequest>,
        ) -> Result<tonic::Response<Self::TaskStatsStream>, tonic::Status>;
        #[doc = "Server streaming response type for the TaskEvents method."]
        type TaskEventsStream: futures_core::Stream<Item = Result<super::DriverTaskEvent, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " TaskEvents starts a streaming RPC where all task events emitted by the"]
        #[doc = " driver are streamed to the caller."]
        async fn task_events(
            &self,
            request: tonic::Request<super::TaskEventsRequest>,
        ) -> Result<tonic::Response<Self::TaskEventsStream>, tonic::Status>;
        #[doc = " SignalTask sends a signal to the task"]
        async fn signal_task(
            &self,
            request: tonic::Request<super::SignalTaskRequest>,
        ) -> Result<tonic::Response<super::SignalTaskResponse>, tonic::Status>;
        #[doc = " ExecTask executes a command inside the tasks execution context"]
        async fn exec_task(
            &self,
            request: tonic::Request<super::ExecTaskRequest>,
        ) -> Result<tonic::Response<super::ExecTaskResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the ExecTaskStreaming method."]
        type ExecTaskStreamingStream: futures_core::Stream<Item = Result<super::ExecTaskStreamingResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " ExecTaskStreaming executes a command inside the tasks execution context"]
        #[doc = " and streams back results"]
        #[doc = " buf:lint:ignore RPC_REQUEST_RESPONSE_UNIQUE"]
        async fn exec_task_streaming(
            &self,
            request: tonic::Request<tonic::Streaming<super::ExecTaskStreamingRequest>>,
        ) -> Result<tonic::Response<Self::ExecTaskStreamingStream>, tonic::Status>;
        #[doc = " CreateNetwork is implemented when the driver needs to create the network"]
        #[doc = " namespace instead of allowing the Nomad client to do."]
        async fn create_network(
            &self,
            request: tonic::Request<super::CreateNetworkRequest>,
        ) -> Result<tonic::Response<super::CreateNetworkResponse>, tonic::Status>;
        #[doc = " DestroyNetwork destroys a previously created network. This rpc is only"]
        #[doc = " implemented if the driver needs to manage network namespace creation."]
        async fn destroy_network(
            &self,
            request: tonic::Request<super::DestroyNetworkRequest>,
        ) -> Result<tonic::Response<super::DestroyNetworkResponse>, tonic::Status>;
    }
    #[doc = " Driver service defines RPCs used to communicate with a nomad runtime driver."]
    #[doc = " Some rpcs may not be implemented by the driver based on it's capabilities."]
    #[derive(Debug)]
    pub struct DriverServer<T: Driver> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Driver> DriverServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DriverServer<T>
    where
        T: Driver,
        B: Body + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/hashicorp.nomad.plugins.drivers.proto.Driver/TaskConfigSchema" => {
                    #[allow(non_camel_case_types)]
                    struct TaskConfigSchemaSvc<T: Driver>(pub Arc<T>);
                    impl<T: Driver> tonic::server::UnaryService<super::TaskConfigSchemaRequest>
                        for TaskConfigSchemaSvc<T>
                    {
                        type Response = super::TaskConfigSchemaResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TaskConfigSchemaRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).task_config_schema(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TaskConfigSchemaSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/hashicorp.nomad.plugins.drivers.proto.Driver/Capabilities" => {
                    #[allow(non_camel_case_types)]
                    struct CapabilitiesSvc<T: Driver>(pub Arc<T>);
                    impl<T: Driver> tonic::server::UnaryService<super::CapabilitiesRequest> for CapabilitiesSvc<T> {
                        type Response = super::CapabilitiesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CapabilitiesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).capabilities(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CapabilitiesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/hashicorp.nomad.plugins.drivers.proto.Driver/Fingerprint" => {
                    #[allow(non_camel_case_types)]
                    struct FingerprintSvc<T: Driver>(pub Arc<T>);
                    impl<T: Driver> tonic::server::ServerStreamingService<super::FingerprintRequest>
                        for FingerprintSvc<T>
                    {
                        type Response = super::FingerprintResponse;
                        type ResponseStream = T::FingerprintStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FingerprintRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).fingerprint(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FingerprintSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/hashicorp.nomad.plugins.drivers.proto.Driver/RecoverTask" => {
                    #[allow(non_camel_case_types)]
                    struct RecoverTaskSvc<T: Driver>(pub Arc<T>);
                    impl<T: Driver> tonic::server::UnaryService<super::RecoverTaskRequest> for RecoverTaskSvc<T> {
                        type Response = super::RecoverTaskResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RecoverTaskRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).recover_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RecoverTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/hashicorp.nomad.plugins.drivers.proto.Driver/StartTask" => {
                    #[allow(non_camel_case_types)]
                    struct StartTaskSvc<T: Driver>(pub Arc<T>);
                    impl<T: Driver> tonic::server::UnaryService<super::StartTaskRequest> for StartTaskSvc<T> {
                        type Response = super::StartTaskResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartTaskRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).start_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StartTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/hashicorp.nomad.plugins.drivers.proto.Driver/WaitTask" => {
                    #[allow(non_camel_case_types)]
                    struct WaitTaskSvc<T: Driver>(pub Arc<T>);
                    impl<T: Driver> tonic::server::UnaryService<super::WaitTaskRequest> for WaitTaskSvc<T> {
                        type Response = super::WaitTaskResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WaitTaskRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).wait_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WaitTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/hashicorp.nomad.plugins.drivers.proto.Driver/StopTask" => {
                    #[allow(non_camel_case_types)]
                    struct StopTaskSvc<T: Driver>(pub Arc<T>);
                    impl<T: Driver> tonic::server::UnaryService<super::StopTaskRequest> for StopTaskSvc<T> {
                        type Response = super::StopTaskResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopTaskRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stop_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StopTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/hashicorp.nomad.plugins.drivers.proto.Driver/DestroyTask" => {
                    #[allow(non_camel_case_types)]
                    struct DestroyTaskSvc<T: Driver>(pub Arc<T>);
                    impl<T: Driver> tonic::server::UnaryService<super::DestroyTaskRequest> for DestroyTaskSvc<T> {
                        type Response = super::DestroyTaskResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DestroyTaskRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).destroy_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DestroyTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/hashicorp.nomad.plugins.drivers.proto.Driver/InspectTask" => {
                    #[allow(non_camel_case_types)]
                    struct InspectTaskSvc<T: Driver>(pub Arc<T>);
                    impl<T: Driver> tonic::server::UnaryService<super::InspectTaskRequest> for InspectTaskSvc<T> {
                        type Response = super::InspectTaskResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InspectTaskRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).inspect_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = InspectTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/hashicorp.nomad.plugins.drivers.proto.Driver/TaskStats" => {
                    #[allow(non_camel_case_types)]
                    struct TaskStatsSvc<T: Driver>(pub Arc<T>);
                    impl<T: Driver> tonic::server::ServerStreamingService<super::TaskStatsRequest> for TaskStatsSvc<T> {
                        type Response = super::TaskStatsResponse;
                        type ResponseStream = T::TaskStatsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TaskStatsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).task_stats(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TaskStatsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/hashicorp.nomad.plugins.drivers.proto.Driver/TaskEvents" => {
                    #[allow(non_camel_case_types)]
                    struct TaskEventsSvc<T: Driver>(pub Arc<T>);
                    impl<T: Driver> tonic::server::ServerStreamingService<super::TaskEventsRequest>
                        for TaskEventsSvc<T>
                    {
                        type Response = super::DriverTaskEvent;
                        type ResponseStream = T::TaskEventsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TaskEventsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).task_events(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TaskEventsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/hashicorp.nomad.plugins.drivers.proto.Driver/SignalTask" => {
                    #[allow(non_camel_case_types)]
                    struct SignalTaskSvc<T: Driver>(pub Arc<T>);
                    impl<T: Driver> tonic::server::UnaryService<super::SignalTaskRequest> for SignalTaskSvc<T> {
                        type Response = super::SignalTaskResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignalTaskRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).signal_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SignalTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/hashicorp.nomad.plugins.drivers.proto.Driver/ExecTask" => {
                    #[allow(non_camel_case_types)]
                    struct ExecTaskSvc<T: Driver>(pub Arc<T>);
                    impl<T: Driver> tonic::server::UnaryService<super::ExecTaskRequest> for ExecTaskSvc<T> {
                        type Response = super::ExecTaskResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecTaskRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).exec_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExecTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/hashicorp.nomad.plugins.drivers.proto.Driver/ExecTaskStreaming" => {
                    #[allow(non_camel_case_types)]
                    struct ExecTaskStreamingSvc<T: Driver>(pub Arc<T>);
                    impl<T: Driver> tonic::server::StreamingService<super::ExecTaskStreamingRequest>
                        for ExecTaskStreamingSvc<T>
                    {
                        type Response = super::ExecTaskStreamingResponse;
                        type ResponseStream = T::ExecTaskStreamingStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::ExecTaskStreamingRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).exec_task_streaming(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExecTaskStreamingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/hashicorp.nomad.plugins.drivers.proto.Driver/CreateNetwork" => {
                    #[allow(non_camel_case_types)]
                    struct CreateNetworkSvc<T: Driver>(pub Arc<T>);
                    impl<T: Driver> tonic::server::UnaryService<super::CreateNetworkRequest> for CreateNetworkSvc<T> {
                        type Response = super::CreateNetworkResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateNetworkRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_network(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateNetworkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/hashicorp.nomad.plugins.drivers.proto.Driver/DestroyNetwork" => {
                    #[allow(non_camel_case_types)]
                    struct DestroyNetworkSvc<T: Driver>(pub Arc<T>);
                    impl<T: Driver> tonic::server::UnaryService<super::DestroyNetworkRequest> for DestroyNetworkSvc<T> {
                        type Response = super::DestroyNetworkResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DestroyNetworkRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).destroy_network(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DestroyNetworkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Driver> Clone for DriverServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Driver> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Driver> tonic::transport::NamedService for DriverServer<T> {
        const NAME: &'static str = "hashicorp.nomad.plugins.drivers.proto.Driver";
    }
}
