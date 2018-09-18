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
	fn NewBaseContainer(),
	fn FromDisk(),
	fn toDisk(),
	fn CheckpointTo(),
	fn readHostConfig(),
	fn WriteHostConfig(),
	fn SetupWorkingDirectory(),
	fn GetResourcePath(),
	fn GetRootResourcePath(),
	fn ExitOnNext(),
	fn HostConfigPath(),
	fn ConfigPath(),
	fn CheckpointDir(),
	fn StartLogger(),
	fn GetProcessLabel(),
	fn GetMountLabel(),
	fn GetExecIDs(),
	fn ShouldRestart(),
	fn AddMountPointWithVolume(),
	fn UnmountVolumes(),
	fn IsDestinationMounted(),
	fn StopSignal(),
	fn StopTimeout(),
	fn InitDNSHostConfig(),
	fn UpdateMonitor(),
	fn FullHostname(),
	fn RestartManager(),
	fn InitAttachContext(),
	fn CancelAttachContext(),
	fn startLogging(),
	fn StdinPipe(),
	fn StdoutPipe(),
	fn StderrPipe(),
	fn CloseStreams(),
	fn InitializeStdio(),
	fn MountsResourcePath(),
	fn SecretMountPath(),
	fn SecretFilePath(),
	fn getSecretTargetPath(),
	fn CreateDaemonEnvironment(),
	fn Close(),
	fn Wait(),
        fn }
}
