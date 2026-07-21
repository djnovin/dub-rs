use dub_api::workspaces::Workspace;
use dub_core::{DubHandle, Result};

/// Workspaces resource for managing workspaces/projects
#[derive(Debug, Clone)]
pub struct Workspaces {
    handle: DubHandle,
}

impl Workspaces {
    pub(crate) fn new(handle: DubHandle) -> Self {
        Self { handle }
    }

    /// List all workspaces
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dub::Dub;
    /// # async fn example() -> dub::Result<()> {
    /// let dub = Dub::new("token")?;
    /// let workspaces = dub.workspaces().list().await?;
    /// for workspace in workspaces {
    ///     println!("Workspace: {}", workspace.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self) -> Result<Vec<Workspace>> {
        let client = self.handle.client();
        client.get("/workspaces").await
    }

    /// Get a specific workspace by ID
    pub async fn get(&self, workspace_id: &str) -> Result<Workspace> {
        let client = self.handle.client();
        client.get(&format!("/workspaces/{}", workspace_id)).await
    }
}
