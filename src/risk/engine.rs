use crate::amount::Amount;
use crate::error::{OrbitError, OrbitResult};
use crate::ids::{AccountId, RouteId};
use crate::routes::RouteConfig;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RiskSnapshot {
    pub route_residual: BTreeMap<RouteId, Amount>,
    pub account_notional: BTreeMap<AccountId, Amount>,
    pub session_notional: Amount,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RiskEngine {
    pub snapshot: RiskSnapshot,
    pub max_account_notional: Amount,
    pub max_session_notional: Amount,
}

impl RiskEngine {
    pub fn new(max_account_notional: Amount, max_session_notional: Amount) -> Self {
        Self {
            snapshot: RiskSnapshot::default(),
            max_account_notional,
            max_session_notional,
        }
    }

    pub fn authorize(
        &mut self,
        route: &RouteConfig,
        account: AccountId,
        payout: Amount,
        counterflow_credit: Amount,
    ) -> OrbitResult<()> {
        let residual = payout.saturating_sub(counterflow_credit);

        if residual > route.max_unhedged {
            return Err(OrbitError::RouteCapacityExceeded);
        }

        let route_current = self
            .snapshot
            .route_residual
            .get(&route.id)
            .copied()
            .unwrap_or_default();
        let route_next = route_current.checked_add(residual)?;
        if route_next > route.session_limit {
            return Err(OrbitError::SettlementLimitExceeded);
        }

        let account_current = self
            .snapshot
            .account_notional
            .get(&account)
            .copied()
            .unwrap_or_default();
        let account_next = account_current.checked_add(payout)?;
        if account_next > self.max_account_notional {
            return Err(OrbitError::SettlementLimitExceeded);
        }

        let session_next = self.snapshot.session_notional.checked_add(payout)?;
        if session_next > self.max_session_notional {
            return Err(OrbitError::SettlementLimitExceeded);
        }

        self.snapshot.route_residual.insert(route.id, route_next);
        self.snapshot.account_notional.insert(account, account_next);
        self.snapshot.session_notional = session_next;
        Ok(())
    }
}
