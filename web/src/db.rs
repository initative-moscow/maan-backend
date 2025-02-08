use anyhow::Error;
use async_trait::async_trait;
use maan_core::create_beneficiary::BeneficiaryData;
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
    async fn set_added_to_ms(&self, id: String) -> Result<(), Error>;
    async fn get_beneficiary(&self, id: &str) -> Result<Option<StoredBeneficiaryData>, Error>;
    async fn store_charity_project(
        &self,
        beneficiary_id: String,
        project: CharityProject,
    ) -> Result<(), Error>;
    async fn get_charity_project(&self, id: &str) -> Result<Option<CharityProject>, Error>;
    async fn increase_collected_by(&self, charity_id: &str, amount: u32) -> Result<(), Error>;
    async fn get_beneficiary_charity_projects(
        &self,
        id: &str,
    ) -> Result<Option<Vec<String>>, Error>;
    async fn get_all_charity_projects(&self) -> Result<Vec<CharityProject>, Error>;
    async fn store_beneficiary_document(
        &self,
        beneficiary_id: String,
        (document_id, b64_document): (String, String),
    ) -> Result<(), Error>;
    async fn get_beneficiary_document(
        &self,
        beneficiary_id: &str,
        document_id: &str,
    ) -> Result<Option<String>, Error>;
    async fn store_donation_data(
        &self,
        qr_code_id: String,
        project_id: String,
        amount: u32,
    ) -> Result<(), Error>;
    async fn get_donation_data(&self, qr_code_id: &str) -> Result<Option<(String, u32)>, Error>;
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
            beneficiary_documents: BTreeMap::new(),
            donation_data: BTreeMap::new(),
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

    async fn set_added_to_ms(&self, id: String) -> Result<(), Error> {
        self.inner.lock().await.set_added_to_ms(id)
    }

    async fn get_beneficiary(&self, id: &str) -> Result<Option<StoredBeneficiaryData>, Error> {
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

    async fn increase_collected_by(&self, charity_id: &str, amount: u32) -> Result<(), Error> {
        self.inner
            .lock()
            .await
            .increase_collected_by(charity_id, amount)
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

    async fn store_beneficiary_document(
        &self,
        beneficiary_id: String,
        (document_id, b64_document): (String, String),
    ) -> Result<(), Error> {
        self.inner.lock().await.store_beneficiary_document(
            beneficiary_id,
            document_id,
            b64_document,
        )
    }

    async fn get_beneficiary_document(
        &self,
        beneficiary_id: &str,
        document_id: &str,
    ) -> Result<Option<String>, Error> {
        self.inner
            .lock()
            .await
            .get_beneficiary_document(beneficiary_id, document_id)
    }

    async fn store_donation_data(
        &self,
        qr_code_id: String,
        project_id: String,
        amount: u32,
    ) -> Result<(), Error> {
        self.inner
            .lock()
            .await
            .donation_data
            .insert(qr_code_id, (project_id, amount));
        Ok(())
    }

    async fn get_donation_data(&self, qr_code_id: &str) -> Result<Option<(String, u32)>, Error> {
        Ok(self
            .inner
            .lock()
            .await
            .donation_data
            .get(qr_code_id)
            .cloned())
    }
}

#[derive(Debug, Clone)]
struct InMemoryStoreInner {
    beneficiaries: BTreeMap<String, StoredBeneficiaryData>,
    beneficiary_documents: BTreeMap<String, Vec<(String, String)>>,
    charity_projects: BTreeMap<String, CharityProject>,
    beneficiary_projects: BTreeMap<String, Vec<String>>,
    donation_data: BTreeMap<String, (String, u32)>,
}

impl InMemoryStoreInner {
    fn store_beneficiary(&mut self, id: String, beneficiary: BeneficiaryData) -> Result<(), Error> {
        let stored_beneficiary_data = StoredBeneficiaryData {
            beneficiary,
            is_addded_to_ms: false,
        };
        if self
            .beneficiaries
            .insert(id.clone(), stored_beneficiary_data)
            .is_some()
        {
            return Err(anyhow::anyhow!("Beneficiary with id {id} already exists"));
        }

        Ok(())
    }

    fn set_added_to_ms(&mut self, id: String) -> Result<(), Error> {
        let beneficiary = self
            .beneficiaries
            .get_mut(&id)
            .ok_or_else(|| anyhow::anyhow!("Beneficiary with id {id} not found"))?;
        beneficiary.is_addded_to_ms = true;

        Ok(())
    }

    fn get_beneficiary(&self, id: &str) -> Result<Option<StoredBeneficiaryData>, Error> {
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
                .or_default()
                .push(id);

            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Charity project with id {id} already exists",
            ))
        }
    }

    fn increase_collected_by(&mut self, charity_id: &str, amount: u32) -> Result<(), Error> {
        let project = self
            .charity_projects
            .get_mut(charity_id)
            .ok_or_else(|| anyhow::anyhow!("Charity project with id {charity_id} not found"))?;
        project.collected += amount;

        Ok(())
    }

    fn store_beneficiary_document(
        &mut self,
        beneficiary_id: String,
        document_id: String,
        b64_document: String,
    ) -> Result<(), Error> {
        self.beneficiary_documents
            .entry(beneficiary_id)
            .or_default()
            .push((document_id, b64_document));

        Ok(())
    }

    fn get_beneficiary_document(
        &self,
        beneficiary_id: &str,
        document_id: &str,
    ) -> Result<Option<String>, Error> {
        Ok(self
            .beneficiary_documents
            .get(beneficiary_id)
            .and_then(|documents| {
                documents
                    .iter()
                    .find(|(id, _)| id == document_id)
                    .map(|(_, b64)| b64.clone())
            }))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct StoredBeneficiaryData {
    pub(crate) beneficiary: BeneficiaryData,
    pub(crate) is_addded_to_ms: bool,
}
