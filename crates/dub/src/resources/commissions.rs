use dub_api::commissions::{
    Commission, CreateCommissionRequest, CreateCommissionResponse, ListCommissionsParams,
    UpdateCommissionRequest,
};
use dub_core::{DubHandle, Result};

/// Commissions resource for managing partner commissions
#[derive(Debug, Clone)]
pub struct Commissions {
    handle: DubHandle,
}

impl Commissions {
    pub(crate) fn new(handle: DubHandle) -> Self {
        Self { handle }
    }

    /// List all commissions
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dub::Dub;
    /// # use dub_api::commissions::ListCommissionsParams;
    /// # async fn example() -> dub::Result<()> {
    /// let dub = Dub::new("token")?;
    ///
    /// // List all commissions
    /// let commissions = dub.commissions().list(None).await?;
    ///
    /// // List with filters
    /// let params = ListCommissionsParams {
    ///     status: Some("pending".to_string()),
    ///     partner_id: Some("partner_123".to_string()),
    ///     ..Default::default()
    /// };
    /// let filtered_commissions = dub.commissions().list(Some(params)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, params: Option<ListCommissionsParams>) -> Result<Vec<Commission>> {
        let client = self.handle.client();

        if let Some(params) = params {
            let mut query_parts = Vec::new();

            if let Some(commission_type) = params.commission_type {
                query_parts.push(format!("type={}", commission_type));
            }
            if let Some(customer_id) = params.customer_id {
                query_parts.push(format!("customerId={}", customer_id));
            }
            if let Some(payout_id) = params.payout_id {
                query_parts.push(format!("payoutId={}", payout_id));
            }
            if let Some(partner_id) = params.partner_id {
                query_parts.push(format!("partnerId={}", partner_id));
            }
            if let Some(tenant_id) = params.tenant_id {
                query_parts.push(format!("tenantId={}", tenant_id));
            }
            if let Some(group_id) = params.group_id {
                query_parts.push(format!("groupId={}", group_id));
            }
            if let Some(partner_tag_id) = params.partner_tag_id {
                query_parts.push(format!("partnerTagId={}", partner_tag_id));
            }
            if let Some(invoice_id) = params.invoice_id {
                query_parts.push(format!("invoiceId={}", invoice_id));
            }
            if let Some(status) = params.status {
                query_parts.push(format!("status={}", status));
            }
            if let Some(sort_by) = params.sort_by {
                query_parts.push(format!("sortBy={}", sort_by));
            }
            if let Some(sort_order) = params.sort_order {
                query_parts.push(format!("sortOrder={}", sort_order));
            }
            if let Some(interval) = params.interval {
                query_parts.push(format!("interval={}", interval));
            }
            if let Some(start) = params.start {
                query_parts.push(format!("start={}", start));
            }
            if let Some(end) = params.end {
                query_parts.push(format!("end={}", end));
            }
            if let Some(timezone) = params.timezone {
                query_parts.push(format!("timezone={}", timezone));
            }
            if let Some(ending_before) = params.ending_before {
                query_parts.push(format!("endingBefore={}", ending_before));
            }
            if let Some(starting_after) = params.starting_after {
                query_parts.push(format!("startingAfter={}", starting_after));
            }
            if let Some(page) = params.page {
                query_parts.push(format!("page={}", page));
            }
            if let Some(page_size) = params.page_size {
                query_parts.push(format!("pageSize={}", page_size));
            }

            if query_parts.is_empty() {
                client.get("/commissions").await
            } else {
                let query = query_parts.join("&");
                client.get(&format!("/commissions?{}", query)).await
            }
        } else {
            client.get("/commissions").await
        }
    }

    /// Create a new commission
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dub::Dub;
    /// # use dub_api::commissions::CreateCommissionRequest;
    /// # async fn example() -> dub::Result<()> {
    /// let dub = Dub::new("token")?;
    ///
    /// let request = CreateCommissionRequest {
    ///     commission_type: "sale".to_string(),
    ///     partner_id: "partner_123".to_string(),
    ///     amount: 100.0,
    ///     date: None,
    ///     description: Some("Q4 bonus".to_string()),
    /// };
    ///
    /// let response = dub.commissions().create(request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        request: CreateCommissionRequest,
    ) -> Result<CreateCommissionResponse> {
        let client = self.handle.client();
        client.post("/commissions", &request).await
    }

    /// Update an existing commission
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dub::Dub;
    /// # use dub_api::commissions::UpdateCommissionRequest;
    /// # async fn example() -> dub::Result<()> {
    /// let dub = Dub::new("token")?;
    ///
    /// let request = UpdateCommissionRequest {
    ///     status: Some("processed".to_string()),
    ///     amount: Some(150.0),
    ///     description: Some("Updated description".to_string()),
    /// };
    ///
    /// let commission = dub.commissions().update("commission_123", request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(&self, id: &str, request: UpdateCommissionRequest) -> Result<Commission> {
        let client = self.handle.client();
        client
            .patch(&format!("/commissions/{}", id), &request)
            .await
    }
}
