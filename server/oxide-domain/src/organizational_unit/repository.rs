use async_trait::async_trait;
use uuid::Uuid;
use crate::error::DomainError;
use crate::organizational_unit::{Unit, UnitType};

#[async_trait]
pub trait UnitTypeRepository {
    async fn create_type(&self, unit_type: UnitType) -> Result<UnitType, DomainError>;
}

#[async_trait]
pub trait UnitRepository {
    async fn create_unit(&self, unit: Unit) -> Result<Unit, DomainError>;
    async fn get_unit_by_id(&self, id: Uuid) -> Result<Unit, DomainError>;
    async fn get_units_by_parent_id(&self, pid: Uuid) -> Result<Vec<Unit>, DomainError>;
}