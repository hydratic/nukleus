// ext.rs
// provides a call t every function in nukleus for sudo level hacking on the kernel
// --------------------------------------------------------------------------------
// implemented:
//
// terminal
//
//
// partial:
//
// docker
// security
//
// 
// TODO:
// arguments + types for each function
// --------------------------------------------------------------------------------
#[warn(dead_code)]
#![no_std]

pub mod docker;
pub mod security;
pub mod terminal;

// SECURITY

pub fn no_sudo() { 
  	security::disable_sudo();
  	unsafe {
    	if SUDO == 1 {
			SUDO -= 1;	
		}
	}
}

pub fn sudo() {
	security::enable_sudo();
	unsafe {
		if SUDO == 0 {
			
		SUDO += 1;
	}
}

// TERMINAL

pub fn init() { terminal::shell_init(); }
pub fn switch() { terminal::switch(); }

// DOCKER
	
pub fn NewBaseContainer() { docker::export::NewBaseContainer(); }
pub fn FromDisk() { docker::export::FromDisk(); }
pub fn toDisk() { docker::export::toDisk(); } 
pub fn CheckpointTo() { docker::export::CheckpointTo(); }
pub fn readHostConfig() { docker::export::readHostConfig(); }
pub fn WriteHostConfig() { docker::export::WriteHostConfig(); }
pub fn SetupWorkingDirectory() { docker::export::SetupWorkingDirectory(); }
pub fn GetResourcePath() { docker::export::GetResourcePath(); }
pub fn GetRootResourcePath() { docker::export::GetRootResourcePath(); }
pub fn ExitOnNext() { docker::export::ExitOnNext(); }
pub fn HostConfigPath() { docker::export::HostConfigPath(); }
pub fn ConfigPath() { docker::export::ConfigPath(); }
pub fn CheckpointDir() { docker::export::CheckpointDir(); }
pub fn StartLogger() { docker::export::StartLogger(); }
pub fn GetProcessLabel() { docker::export::GetProcessLabel(); }
pub fn GetMountLabel() { docker::export::GetMountLabel(); }
pub fn GetExecIDs() { docker::export::GetExecIDs(); }
pub fn ShouldRestart() { docker::export::ShouldRestart(); }
pub fn AddMountPointWithVolume() { docker::export::AddMountPointWithVolume(); }
pub fn UnmountVolumes() { docker::export::UnmountVolumes(); }
pub fn IsDestinationMounted() { docker::export::IsDestinationMounted(); }
pub fn StopSignal() { docker::export::StopSignal(); }
pub fn StopTimeout() { docker::export::StopTimeout(); }
pub fn InitDNSHostConfig() { docker::export::InitDNSHostConfig(); }
pub fn UpdateMonitor() { docker::export::UpdateMonitor(); }
pub fn FullHostname() { docker::export::FullHostname(); }
pub fn RestartManager() { docker::export::RestartManager(); }
pub fn InitAttachContext() { docker::export::InitAttachContext(); }
pub fn CancelAttachContext() { docker::export::CancelAttachContext(); }
pub fn startLogging() { docker::export::startLogging(); }
pub fn StdinPipe() { docker::export::StdinPipe(); }
pub fn StdoutPipe() { docker::export::StdoutPipe(); }
pub fn StderrPipe() { docker::export::StderrPipe(); }
pub fn CloseStreams() { docker::export::CloseStreams(); }
pub fn InitializeStdio() { docker::export::InitializeStdio(); }
pub fn MountsResourcePath() { docker::export::MountsResourcePath(); }
pub fn SecretMountPath() { docker::export::SecretMountPath(); }
pub fn SecretFilePath() { docker::export::SecretFilePath(); }
pub fn getSecretTargetPath() { docker::export::getSecretTargetPath(); }
pub fn CreateDaemonEnvironment() { docker::export::CreateDaemonEnvironment(); }
pub fn Close() { docker::export::Close(); }
pub fn Wait() { docker::export::Wait(); }
