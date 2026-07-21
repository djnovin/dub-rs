mod builder;
mod client;
mod resources;

pub use builder::LinkBuilder;
pub use client::Dub;
pub use dub_api::analytics::{AnalyticsDataPoint, AnalyticsSummary};
pub use dub_api::bounties::{
    ApproveBountyRequest, BountySubmission, FileInfo, ListBountySubmissionsParams,
    RejectBountyRequest,
};
pub use dub_api::commissions::{
    Commission, CreateCommissionRequest, CreateCommissionResponse, CustomerInfo,
    ListCommissionsParams, UpdateCommissionRequest,
};
pub use dub_api::customers::{Customer, ListCustomersParams, UpdateCustomerRequest};
pub use dub_api::domains::{
    CheckDomainAvailabilityRequest, CreateDomainRequest, Domain, DomainAvailabilityResponse,
    RegisterDomainRequest, UpdateDomainRequest,
};
pub use dub_api::events::{Event, ListEventsParams};
pub use dub_api::links::{
    BulkDeleteLinksRequest, BulkUpdateLinksRequest, CreateLinkRequest, Link, LinkCountResponse,
    ListLinksParams, UpdateLinkRequest, UpsertLinkRequest,
};
pub use dub_api::partners::{
    CreatePartnerLinkRequest, CreatePartnerRequest, ListPartnersParams, Partner, PartnerSale,
    UpsertPartnerLinkRequest,
};
pub use dub_api::payouts::{ListPayoutsParams, PartnerInfo, Payout, UserInfo};
pub use dub_api::tags::{CreateTagRequest, Tag, UpdateTagRequest};
pub use dub_api::track::{TrackLeadRequest, TrackResponse, TrackSaleRequest};
pub use dub_api::workspaces::Workspace;
pub use dub_core::{DubError, Result};

// Re-export for convenience
pub use resources::*;
