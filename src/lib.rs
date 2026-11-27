#![forbid(unsafe_code)]
//! SigmaOS Jupyter integration: kernel and session lifecycle management.

use thiserror::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Error)]
pub enum JupyterError {
    #[error("kernel not found: {0}")]
    KernelNotFound(String),
    #[error("session not found: {0}")]
    SessionNotFound(String),
    #[error("kernel already running: {0}")]
    KernelAlreadyRunning(String),
    #[error("invalid kernel spec: {0}")]
    InvalidKernelSpec(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KernelLanguage {
    Python,
    R,
    Julia,
    Rust,
    JavaScript,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KernelState {
    Idle,
    Busy,
    Dead,
    Starting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JupyterKernel {
    pub kernel_id: String,
    pub name: String,
    pub language: KernelLanguage,
    pub state: KernelState,
    pub execution_count: u64,
}

impl JupyterKernel {
    pub fn new(kernel_id: &str, name: &str, language: KernelLanguage) -> Self {
        Self { kernel_id: kernel_id.to_string(), name: name.to_string(), language, state: KernelState::Starting, execution_count: 0 }
    }
    pub fn mark_idle(&mut self) { self.state = KernelState::Idle; }
    pub fn execute(&mut self) -> Result<u64, JupyterError> {
        if self.state == KernelState::Dead { return Err(JupyterError::KernelNotFound(self.kernel_id.clone())); }
        self.state = KernelState::Busy;
        self.execution_count += 1;
        self.state = KernelState::Idle;
        Ok(self.execution_count)
    }
    pub fn shutdown(&mut self) { self.state = KernelState::Dead; }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JupyterSession {
    pub session_id: String,
    pub notebook_path: String,
    pub kernel_id: String,
}

pub struct JupyterManager {
    kernels: HashMap<String, JupyterKernel>,
    sessions: HashMap<String, JupyterSession>,
}

impl JupyterManager {
    pub fn new() -> Self { Self { kernels: HashMap::new(), sessions: HashMap::new() } }
    pub fn start_kernel(&mut self, kernel: JupyterKernel) -> Result<(), JupyterError> {
        if self.kernels.contains_key(&kernel.kernel_id) {
            return Err(JupyterError::KernelAlreadyRunning(kernel.kernel_id.clone()));
        }
        self.kernels.insert(kernel.kernel_id.clone(), kernel);
        Ok(())
    }
    pub fn create_session(&mut self, session: JupyterSession) -> Result<(), JupyterError> {
        if !self.kernels.contains_key(&session.kernel_id) {
            return Err(JupyterError::KernelNotFound(session.kernel_id.clone()));
        }
        self.sessions.insert(session.session_id.clone(), session);
        Ok(())
    }
    pub fn get_kernel(&self, id: &str) -> Result<&JupyterKernel, JupyterError> {
        self.kernels.get(id).ok_or_else(|| JupyterError::KernelNotFound(id.to_string()))
    }
    pub fn list_kernels(&self) -> Vec<&JupyterKernel> { self.kernels.values().collect() }
    pub fn active_sessions(&self) -> usize { self.sessions.len() }
}

impl Default for JupyterManager { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_start_kernel_aaa() {
        // Arrange
        let mut mgr = JupyterManager::new();
        let kernel = JupyterKernel::new("k1", "python3", KernelLanguage::Python);
        // Act
        mgr.start_kernel(kernel).unwrap();
        // Assert
        assert_eq!(mgr.list_kernels().len(), 1);
    }
    #[test]
    fn test_kernel_execution_count_aaa() {
        // Arrange
        let mut kernel = JupyterKernel::new("k2", "ir", KernelLanguage::R);
        kernel.mark_idle();
        // Act
        let n1 = kernel.execute().unwrap();
        let n2 = kernel.execute().unwrap();
        // Assert
        assert_eq!(n1, 1);
        assert_eq!(n2, 2);
    }
    #[test]
    fn test_duplicate_kernel_aaa() {
        // Arrange
        let mut mgr = JupyterManager::new();
        mgr.start_kernel(JupyterKernel::new("k3", "python3", KernelLanguage::Python)).unwrap();
        // Act
        let result = mgr.start_kernel(JupyterKernel::new("k3", "python3", KernelLanguage::Python));
        // Assert
        assert!(matches!(result, Err(JupyterError::KernelAlreadyRunning(_))));
    }
    #[test]
    fn test_session_requires_kernel_aaa() {
        // Arrange
        let mut mgr = JupyterManager::new();
        let session = JupyterSession { session_id: "s1".into(), notebook_path: "/notebooks/test.ipynb".into(), kernel_id: "k-missing".into() };
        // Act
        let result = mgr.create_session(session);
        // Assert
        assert!(matches!(result, Err(JupyterError::KernelNotFound(_))));
    }
}
