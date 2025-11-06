pub trait Commitment {
    fn processed(&self) -> Self;

    fn confirmed(&self) -> Self;

    fn finalized(&self) -> Self;

    fn into(commitment_str: &str) -> Self;

    /// Get the commitment as a [str] format
    fn as_str(&self) -> &str;
}
