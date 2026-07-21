use dub_api::payouts::{ListPayoutsParams, Payout};
use dub_core::{DubHandle, Result};

/// Payouts resource for managing partner payouts
#[derive(Debug, Clone)]
pub struct Payouts {
    handle: DubHandle,
}

impl Payouts {
    pub(crate) fn new(handle: DubHandle) -> Self {
        Self { handle }
    }

    /// List all payouts
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dub::Dub;
    /// # use dub_api::payouts::ListPayoutsParams;
    /// # async fn example() -> dub::Result<()> {
    /// let dub = Dub::new("token")?;
    ///
    /// // List all payouts
    /// let payouts = dub.payouts().list(None).await?;
    ///
    /// // List with filters
    /// let params = ListPayoutsParams {
    ///     status: Some("completed".to_string()),
    ///     partner_id: Some("partner_123".to_string()),
    ///     ..Default::default()
    /// };
    /// let filtered_payouts = dub.payouts().list(Some(params)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, params: Option<ListPayoutsParams>) -> Result<Vec<Payout>> {
        let client = self.handle.client();

        if let Some(params) = params {
            let mut query_parts = Vec::new();

            if let Some(status) = params.status {
                query_parts.push(format!("status={}", status));
            }
            if let Some(partner_id) = params.partner_id {
                query_parts.push(format!("partnerId={}", partner_id));
            }
            if let Some(tenant_id) = params.tenant_id {
                query_parts.push(format!("tenantId={}", tenant_id));
            }
            if let Some(invoice_id) = params.invoice_id {
                query_parts.push(format!("invoiceId={}", invoice_id));
            }
            if let Some(group_id) = params.group_id {
                query_parts.push(format!("groupId={}", group_id));
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
                client.get("/payouts").await
            } else {
                let query = query_parts.join("&");
                client.get(&format!("/payouts?{}", query)).await
            }
        } else {
            client.get("/payouts").await
        }
    }
}
