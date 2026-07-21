use dub_api::bounties::{
    ApproveBountyRequest, BountySubmission, ListBountySubmissionsParams, RejectBountyRequest,
};
use dub_core::{DubHandle, Result};

/// Bounties resource for managing bounty submissions
#[derive(Debug, Clone)]
pub struct Bounties {
    handle: DubHandle,
}

impl Bounties {
    pub(crate) fn new(handle: DubHandle) -> Self {
        Self { handle }
    }

    /// List all submissions for a bounty
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dub::Dub;
    /// # use dub_api::bounties::ListBountySubmissionsParams;
    /// # async fn example() -> dub::Result<()> {
    /// let dub = Dub::new("token")?;
    ///
    /// // List all submissions
    /// let submissions = dub.bounties().list_submissions("bounty_123", None).await?;
    ///
    /// // List with filters
    /// let params = ListBountySubmissionsParams {
    ///     status: Some("submitted".to_string()),
    ///     partner_id: Some("partner_123".to_string()),
    ///     ..Default::default()
    /// };
    /// let filtered_submissions = dub.bounties()
    ///     .list_submissions("bounty_123", Some(params))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_submissions(
        &self,
        bounty_id: &str,
        params: Option<ListBountySubmissionsParams>,
    ) -> Result<Vec<BountySubmission>> {
        let client = self.handle.client();
        let base_path = format!("/bounties/{}/submissions", bounty_id);

        if let Some(params) = params {
            let mut query_parts = Vec::new();

            if let Some(status) = params.status {
                query_parts.push(format!("status={}", status));
            }
            if let Some(group_id) = params.group_id {
                query_parts.push(format!("groupId={}", group_id));
            }
            if let Some(partner_id) = params.partner_id {
                query_parts.push(format!("partnerId={}", partner_id));
            }
            if let Some(sort_by) = params.sort_by {
                query_parts.push(format!("sortBy={}", sort_by));
            }
            if let Some(sort_order) = params.sort_order {
                query_parts.push(format!("sortOrder={}", sort_order));
            }
            if let Some(page) = params.page {
                query_parts.push(format!("page={}", page));
            }
            if let Some(page_size) = params.page_size {
                query_parts.push(format!("pageSize={}", page_size));
            }

            if query_parts.is_empty() {
                client.get(&base_path).await
            } else {
                let query = query_parts.join("&");
                client.get(&format!("{}?{}", base_path, query)).await
            }
        } else {
            client.get(&base_path).await
        }
    }

    /// Approve a bounty submission
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dub::Dub;
    /// # async fn example() -> dub::Result<()> {
    /// let dub = Dub::new("token")?;
    ///
    /// let submission = dub.bounties()
    ///     .approve_submission("bounty_123", "submission_456")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn approve_submission(
        &self,
        bounty_id: &str,
        submission_id: &str,
    ) -> Result<BountySubmission> {
        let client = self.handle.client();
        let path = format!(
            "/bounties/{}/submissions/{}/approve",
            bounty_id, submission_id
        );
        let request = ApproveBountyRequest::default();
        client.post(&path, &request).await
    }

    /// Reject a bounty submission
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dub::Dub;
    /// # use dub_api::bounties::RejectBountyRequest;
    /// # async fn example() -> dub::Result<()> {
    /// let dub = Dub::new("token")?;
    ///
    /// let request = RejectBountyRequest {
    ///     reason: Some("Does not meet requirements".to_string()),
    ///     note: Some("Please resubmit with corrections".to_string()),
    /// };
    ///
    /// let submission = dub.bounties()
    ///     .reject_submission("bounty_123", "submission_456", request)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn reject_submission(
        &self,
        bounty_id: &str,
        submission_id: &str,
        request: RejectBountyRequest,
    ) -> Result<BountySubmission> {
        let client = self.handle.client();
        let path = format!(
            "/bounties/{}/submissions/{}/reject",
            bounty_id, submission_id
        );
        client.post(&path, &request).await
    }
}
