use thiserror::Error;

pub type OrbitResult<T> = Result<T, OrbitError>;

#[derive(Debug, Error)]
pub enum OrbitError {
    #[error("amount overflow")]
    AmountOverflow,
    #[error("insufficient balance")]
    InsufficientBalance,
    #[error("asset not found")]
    AssetNotFound,
    #[error("asset disabled")]
    AssetDisabled,
    #[error("account not found")]
    AccountNotFound,
    #[error("vault not found")]
    VaultNotFound,
    #[error("route not found")]
    RouteNotFound,
    #[error("route disabled")]
    RouteDisabled,
    #[error("intent not found")]
    IntentNotFound,
    #[error("intent is not pending")]
    IntentNotPending,
    #[error("settlement session not found")]
    SessionNotFound,
    #[error("price not available")]
    PriceNotAvailable,
    #[error("price confidence is outside route policy")]
    PriceConfidence,
    #[error("minimum output was not satisfied")]
    MinimumOutput,
    #[error("settlement limit exceeded")]
    SettlementLimitExceeded,
    #[error("route capacity exceeded")]
    RouteCapacityExceeded,
    #[error("invalid vault asset")]
    InvalidVaultAsset,
    #[error("codec error: {0}")]
    Codec(String),
}
