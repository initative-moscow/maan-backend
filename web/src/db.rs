use anyhow::Error;
use async_trait::async_trait;
use maan_core::tochka::create_beneficiary::BeneficiaryData;
use std::{collections::BTreeMap, fmt::Debug, sync::Arc};
use tokio::sync::Mutex;

use crate::CharityProject;

#[async_trait]
pub trait Store: Send + Sync {
    async fn store_beneficiary(
        &self,
        id: String,
        beneficiary: BeneficiaryData,
    ) -> Result<(), Error>;
    async fn get_beneficiary(&self, id: &str) -> Result<Option<BeneficiaryData>, Error>;
    async fn store_charity_project(
        &self,
        beneficiary_id: String,
        project: CharityProject,
    ) -> Result<(), Error>;
    async fn get_charity_project(&self, id: &str) -> Result<Option<CharityProject>, Error>;
    async fn get_beneficiary_charity_projects(
        &self,
        id: &str,
    ) -> Result<Option<Vec<String>>, Error>;
    async fn get_all_charity_projects(&self) -> Result<Vec<CharityProject>, Error>;
}

#[derive(Debug, Clone)]
pub struct InMemoryStore {
    inner: Arc<Mutex<InMemoryStoreInner>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        let inner = InMemoryStoreInner {
            beneficiaries: BTreeMap::new(),
            charity_projects: BTreeMap::new(),
            beneficiary_projects: BTreeMap::new(),
        };

        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }
}

#[async_trait]
impl Store for InMemoryStore {
    async fn store_beneficiary(
        &self,
        id: String,
        beneficiary: BeneficiaryData,
    ) -> Result<(), Error> {
        self.inner.lock().await.store_beneficiary(id, beneficiary)
    }

    async fn get_beneficiary(&self, id: &str) -> Result<Option<BeneficiaryData>, Error> {
        self.inner.lock().await.get_beneficiary(id)
    }

    async fn store_charity_project(
        &self,
        beneficiary_id: String,
        project: CharityProject,
    ) -> Result<(), Error> {
        self.inner
            .lock()
            .await
            .store_charity_project(beneficiary_id, project)
    }

    async fn get_charity_project(&self, id: &str) -> Result<Option<CharityProject>, Error> {
        Ok(self.inner.lock().await.charity_projects.get(id).cloned())
    }

    async fn get_beneficiary_charity_projects(
        &self,
        id: &str,
    ) -> Result<Option<Vec<String>>, Error> {
        Ok(self
            .inner
            .lock()
            .await
            .beneficiary_projects
            .get(id)
            .cloned())
    }

    async fn get_all_charity_projects(&self) -> Result<Vec<CharityProject>, Error> {
        Ok(self
            .inner
            .lock()
            .await
            .charity_projects
            .values()
            .cloned()
            .collect())
    }
}

#[derive(Debug, Clone)]
struct InMemoryStoreInner {
    beneficiaries: BTreeMap<String, BeneficiaryData>,
    charity_projects: BTreeMap<String, CharityProject>,
    beneficiary_projects: BTreeMap<String, Vec<String>>,
}

impl InMemoryStoreInner {
    fn store_beneficiary(&mut self, id: String, beneficiary: BeneficiaryData) -> Result<(), Error> {
        if self.beneficiaries.insert(id.clone(), beneficiary).is_none() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Beneficiary with id {} already exists", id))
        }
    }

    fn get_beneficiary(&self, id: &str) -> Result<Option<BeneficiaryData>, Error> {
        Ok(self.beneficiaries.get(id).cloned())
    }

    fn store_charity_project(
        &mut self,
        beneficiary_id: String,
        project: CharityProject,
    ) -> Result<(), Error> {
        let id = project.id.clone();
        if self.charity_projects.insert(id.clone(), project).is_none() {
            self.beneficiary_projects
                .entry(beneficiary_id)
                .or_insert(vec![])
                .push(id);

            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Charity project with id {id} already exists",
            ))
        }
    }
}
