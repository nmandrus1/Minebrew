
/// Trait for any struct that can be turned into an API Request
pub trait ToRequest {
    /// Outputs a string that represents an API request
    fn to_req(&self) -> String;
}
