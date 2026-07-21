use super::types::Customer;

/// Response when creating or updating a customer
pub type CustomerResponse = Customer;

/// Response when listing customers
pub type ListCustomersResponse = Vec<Customer>;
