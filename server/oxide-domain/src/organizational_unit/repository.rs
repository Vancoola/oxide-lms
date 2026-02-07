use async_trait::async_trait;
use crate::error::DomainError;
use crate::organizational_unit::{Unit, UnitType};

#[async_trait]
pub trait UnitTypeRepository {
    async fn create_type(&self, unit_type: UnitType) -> Result<UnitType, DomainError>;
}

#[async_trait]
pub trait UnitRepository {
    async fn create_unit(&self, unit: Unit) -> Result<UnitType, DomainError>;
}