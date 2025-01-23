use anyhow::Error;
use async_trait::async_trait;
use maan_core::tochka::create_beneficiary::BeneficiaryData;
use std::{collections::BTreeMap, fmt::Debug, sync::Arc};
use tokio::sync::Mutex;

#[async_trait]
pub trait Store: Send + Sync {
    async fn store_beneficiary(
        &self,
        id: String,
        beneficiary: BeneficiaryData,
    ) -> Result<(), Error>;
    async fn get_beneficiary(&self, id: &str) -> Result<Option<BeneficiaryData>, Error>;
}

#[derive(Debug, Clone)]
pub struct InMemoryStore {
    inner: Arc<Mutex<InMemoryStoreInner>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        let inner = InMemoryStoreInner {
            beneficiaries: BTreeMap::new(),
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
}

#[derive(Debug, Clone)]
struct InMemoryStoreInner {
    beneficiaries: BTreeMap<String, BeneficiaryData>,
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
}
