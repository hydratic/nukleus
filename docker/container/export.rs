// export.rs
// exports all functions/structs for docker/container to ext.rs
// this allows users to directly access functions from the shell

#![no_std]

extern crate ux;

type time = 

mod memory;

// archive.go
extern {
    pub fn ResolvePath();
    pub fn StatPath();
}

pub fn archive_Container_ResolvePath() { ResolvePath(); }
pub fn archive_Container_StatPath() { StatPath(); }

// container.go
pub const configFileName: String = "config.v2.json";

pub struct ExitStatus {
    ExitCode: i32,
    OOMKilled: bool,
    ExitedAt: time,
}

pub struct Container {
    StreamConfig: streamConfig,
    State: String,
    Root: String,
    BaseFS: containerfs,
    RWLayer: layer_RWLayer,
    ID: String,
    Created: time,
    Managed: bool,
    Path: String,
    Args: Vec<u64>,
    Config: containertypes_Config,
    ImageID: image_ID,
    NetworkSettings: network_Settings,
    LogPath: String,
    Name: String,
    Driver: String,
    OS: String,
    
    MountLabel: String,
	ProcessLabel: String,
	RestartCount: i32,
	HasBeenStartedBefore: bool,
	HasBeenManuallyDtopped: bool,
	MountPoints: volumemounts_MountPoint,
	HostConfig: containertypes_HostConfig,
	ExecCommands: exec_Store,
	DependencyStore: agentexec.DependencyGetter,
	SecretReferences: swarmtypes_SecretReference,
	ConfigReferences: swarmtypes_ConfigReference,
	
	LogDriver: logger_Logger,
	LogCopier: logger_Copier,
	restartManager: restartmanager_RestartManager,
	attachContext: attachContext,
}

// TODO: types/arguments,
// document each function
extern {
	pub fn NewBaseContainer(),
	pub fn FromDisk(),
	pub fn toDisk(),
	pub fn CheckpointTo(),
	pub fn readHostConfig(),
	pub fn WriteHostConfig(),
	pub fn SetupWorkingDirectory(),
	pub fn GetResourcePath(),
	pub fn GetRootResourcePath(),
	pub fn ExitOnNext(),
	pub fn HostConfigPath(),
	pub fn ConfigPath(),
	pub fn CheckpointDir(),
	pub fn StartLogger(),
	pub fn GetProcessLabel(),
	pub fn GetMountLabel(),
	pub fn GetExecIDs(),
	pub fn ShouldRestart(),
	pub fn AddMountPointWithVolume(),
	pub fn UnmountVolumes(),
	pub fn IsDestinationMounted(),
	pub fn StopSignal(),
	pub fn StopTimeout(),
	pub fn InitDNSHostConfig(),
	pub fn UpdateMonitor(),
	pub fn FullHostname(),
	pub fn RestartManager(),
	pub fn InitAttachContext(),
	pub fn CancelAttachContext(),
	pub fn startLogging(),
	pub fn StdinPipe(),
	pub fn StdoutPipe(),
	pub fn StderrPipe(),
	pub fn CloseStreams(),
	pub fn InitializeStdio(),
	pub fn MountsResourcePath(),
	pub fn SecretMountPath(),
	pub fn SecretFilePath(),
	pub fn getSecretTargetPath(),
	pub fn CreateDaemonEnvironment(),
	pub fn Close(),
	pub fn Wait(),
}
