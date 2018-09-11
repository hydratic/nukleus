// export.rs
// exports all functions/structs for docker/container to ext.rs
// this allows users to directly access functions from the shell

#![no_std]

extern crate ux;

type time = 

mod memory;

// archive.go
extern {
    fn ResolvePath();
    fn StatPath();
}

pub fn Container_ResolvePath() { ResolvePath(); }
pub fn Container_StatPath() { StatPath(); }

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
}
