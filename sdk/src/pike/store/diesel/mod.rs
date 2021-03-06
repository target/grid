// Copyright 2018-2021 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod models;
mod operations;
pub(in crate) mod schema;

use diesel::r2d2::{ConnectionManager, Pool};

use super::{
    Agent, AgentList, Organization, OrganizationList, OrganizationMetadata, PikeStore,
    PikeStoreError, Role,
};
use crate::error::ResourceTemporarilyUnavailableError;
use models::{make_org_metadata_models, make_role_models};
use operations::add_agent::PikeStoreAddAgentOperation as _;
use operations::add_organization::PikeStoreAddOrganizationOperation as _;
use operations::fetch_agent::PikeStoreFetchAgentOperation as _;
use operations::fetch_organization::PikeStoreFetchOrganizationOperation as _;
use operations::list_agents::PikeStoreListAgentsOperation as _;
use operations::list_organizations::PikeStoreListOrganizationsOperation as _;
use operations::update_agent::PikeStoreUpdateAgentOperation as _;
use operations::PikeStoreOperations;

/// Manages creating agents in the database
#[derive(Clone)]
pub struct DieselPikeStore<C: diesel::Connection + 'static> {
    connection_pool: Pool<ConnectionManager<C>>,
}

impl<C: diesel::Connection> DieselPikeStore<C> {
    /// Creates a new DieselPikeStore
    ///
    /// # Arguments
    ///
    ///  * `connection_pool`: connection pool to the database
    pub fn new(connection_pool: Pool<ConnectionManager<C>>) -> Self {
        DieselPikeStore { connection_pool }
    }
}

#[cfg(feature = "postgres")]
impl PikeStore for DieselPikeStore<diesel::pg::PgConnection> {
    fn add_agent(&self, agent: Agent) -> Result<(), PikeStoreError> {
        PikeStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            PikeStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .add_agent(agent.clone().into(), make_role_models(&agent))
    }

    fn list_agents(
        &self,
        service_id: Option<&str>,
        offset: i64,
        limit: i64,
    ) -> Result<AgentList, PikeStoreError> {
        PikeStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            PikeStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .list_agents(service_id, offset, limit)
    }

    fn fetch_agent(
        &self,
        pub_key: &str,
        service_id: Option<&str>,
    ) -> Result<Option<Agent>, PikeStoreError> {
        PikeStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            PikeStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .fetch_agent(pub_key, service_id)
    }

    fn update_agent(&self, agent: Agent) -> Result<(), PikeStoreError> {
        PikeStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            PikeStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .update_agent(agent.clone().into(), make_role_models(&agent))
    }

    fn add_organization(&self, org: Organization) -> Result<(), PikeStoreError> {
        PikeStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            PikeStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .add_organization(org.clone().into(), make_org_metadata_models(&org))
    }

    fn list_organizations(
        &self,
        service_id: Option<&str>,
        offset: i64,
        limit: i64,
    ) -> Result<OrganizationList, PikeStoreError> {
        PikeStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            PikeStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .list_organizations(service_id, offset, limit)
    }

    fn fetch_organization(
        &self,
        org_id: &str,
        service_id: Option<&str>,
    ) -> Result<Option<Organization>, PikeStoreError> {
        PikeStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            PikeStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .fetch_organization(org_id, service_id)
    }
}

#[cfg(feature = "sqlite")]
impl PikeStore for DieselPikeStore<diesel::sqlite::SqliteConnection> {
    fn add_agent(&self, agent: Agent) -> Result<(), PikeStoreError> {
        PikeStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            PikeStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .add_agent(agent.clone().into(), make_role_models(&agent))
    }

    fn list_agents(
        &self,
        service_id: Option<&str>,
        offset: i64,
        limit: i64,
    ) -> Result<AgentList, PikeStoreError> {
        PikeStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            PikeStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .list_agents(service_id, offset, limit)
    }

    fn fetch_agent(
        &self,
        pub_key: &str,
        service_id: Option<&str>,
    ) -> Result<Option<Agent>, PikeStoreError> {
        PikeStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            PikeStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .fetch_agent(pub_key, service_id)
    }

    fn update_agent(&self, agent: Agent) -> Result<(), PikeStoreError> {
        PikeStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            PikeStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .update_agent(agent.clone().into(), make_role_models(&agent))
    }

    fn add_organization(&self, org: Organization) -> Result<(), PikeStoreError> {
        PikeStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            PikeStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .add_organization(org.clone().into(), make_org_metadata_models(&org))
    }

    fn list_organizations(
        &self,
        service_id: Option<&str>,
        offset: i64,
        limit: i64,
    ) -> Result<OrganizationList, PikeStoreError> {
        PikeStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            PikeStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .list_organizations(service_id, offset, limit)
    }

    fn fetch_organization(
        &self,
        org_id: &str,
        service_id: Option<&str>,
    ) -> Result<Option<Organization>, PikeStoreError> {
        PikeStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            PikeStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .fetch_organization(org_id, service_id)
    }
}
