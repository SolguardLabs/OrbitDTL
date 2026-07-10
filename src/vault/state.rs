use crate::amount::Amount;
use crate::error::{OrbitError, OrbitResult};
use crate::ids::{AccountId, AssetId, VaultId};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum VaultKind {
    Reserve,
    Settlement,
    Buffer,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vault {
    pub id: VaultId,
    pub asset: AssetId,
    pub controller: AccountId,
    pub kind: VaultKind,
    pub reserve: Amount,
    pub locked: Amount,
    pub paid: Amount,
    pub received: Amount,
}

impl Vault {
    pub fn new(id: VaultId, asset: AssetId, controller: AccountId, kind: VaultKind) -> Self {
        Self {
            id,
            asset,
            controller,
            kind,
            reserve: Amount::zero(),
            locked: Amount::zero(),
            paid: Amount::zero(),
            received: Amount::zero(),
        }
    }

    pub fn available(&self) -> Amount {
        self.reserve.saturating_sub(self.locked)
    }

    pub fn deposit(&mut self, amount: Amount) -> OrbitResult<()> {
        self.reserve = self.reserve.checked_add(amount)?;
        self.received = self.received.checked_add(amount)?;
        Ok(())
    }

    pub fn reserve(&mut self, amount: Amount) -> OrbitResult<()> {
        if self.available() < amount {
            return Err(OrbitError::InsufficientBalance);
        }

        self.locked = self.locked.checked_add(amount)?;
        Ok(())
    }

    pub fn consume_reserved(&mut self, amount: Amount) -> OrbitResult<()> {
        self.locked = self.locked.checked_sub(amount)?;
        self.reserve = self.reserve.checked_sub(amount)?;
        self.paid = self.paid.checked_add(amount)?;
        Ok(())
    }

    pub fn release_reserved(&mut self, amount: Amount) -> OrbitResult<()> {
        self.locked = self.locked.checked_sub(amount)?;
        Ok(())
    }

    pub fn pay_out(&mut self, amount: Amount) -> OrbitResult<()> {
        if self.available() < amount {
            return Err(OrbitError::InsufficientBalance);
        }

        self.reserve = self.reserve.checked_sub(amount)?;
        self.paid = self.paid.checked_add(amount)?;
        Ok(())
    }

    pub fn accrue(&mut self, amount: Amount) -> OrbitResult<()> {
        self.reserve = self.reserve.checked_add(amount)?;
        self.received = self.received.checked_add(amount)?;
        Ok(())
    }
}
