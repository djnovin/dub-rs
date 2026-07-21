use dub_core::{DubClient, DubConfig, DubHandle, Result};

use crate::resources::analytics::Analytics;
use crate::resources::bounties::Bounties;
use crate::resources::commissions::Commissions;
use crate::resources::customers::Customers;
use crate::resources::domains::Domains;
use crate::resources::events::Events;
use crate::resources::links::Links;
use crate::resources::partners::Partners;
use crate::resources::payouts::Payouts;
use crate::resources::tags::Tags;
use crate::resources::track::Track;
use crate::resources::workspaces::Workspaces;

/// The main Dub SDK client
///
/// # Example
///
/// ```no_run
/// use dub::Dub;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let dub = Dub::new("your-api-token")?;
///
///     let links = dub.links().list(None).await?;
///     println!("Found {} links", links.len());
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Dub {
    handle: DubHandle,
}

impl Dub {
    /// Create a new Dub client with an API token
    pub fn new(api_token: impl Into<String>) -> Result<Self> {
        let config = DubConfig::new(api_token);
        let client = DubClient::new(config)?;
        Ok(Self {
            handle: DubHandle::new(client),
        })
    }

    /// Create a new Dub client with custom configuration
    pub fn with_config(config: DubConfig) -> Result<Self> {
        let client = DubClient::new(config)?;
        Ok(Self {
            handle: DubHandle::new(client),
        })
    }

    /// Get the links resource
    pub fn links(&self) -> Links {
        Links::new(self.handle.clone())
    }

    /// Get the analytics resource
    pub fn analytics(&self) -> Analytics {
        Analytics::new(self.handle.clone())
    }

    /// Get the workspaces resource
    pub fn workspaces(&self) -> Workspaces {
        Workspaces::new(self.handle.clone())
    }

    /// Get the customers resource
    pub fn customers(&self) -> Customers {
        Customers::new(self.handle.clone())
    }

    /// Get the tags resource
    pub fn tags(&self) -> Tags {
        Tags::new(self.handle.clone())
    }

    /// Get the track resource
    pub fn track(&self) -> Track {
        Track::new(self.handle.clone())
    }

    /// Get the domains resource
    pub fn domains(&self) -> Domains {
        Domains::new(self.handle.clone())
    }

    /// Get the events resource
    pub fn events(&self) -> Events {
        Events::new(self.handle.clone())
    }

    /// Get the partners resource
    pub fn partners(&self) -> Partners {
        Partners::new(self.handle.clone())
    }

    /// Get the payouts resource
    pub fn payouts(&self) -> Payouts {
        Payouts::new(self.handle.clone())
    }

    /// Get the commissions resource
    pub fn commissions(&self) -> Commissions {
        Commissions::new(self.handle.clone())
    }

    /// Get the bounties resource
    pub fn bounties(&self) -> Bounties {
        Bounties::new(self.handle.clone())
    }
}
