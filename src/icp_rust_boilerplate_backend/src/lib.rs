#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Debt {
    id: u64,
    debtor: String,
    creditor: String,
    amount: u64,
    created_at: u64,
}

impl Storable for Debt {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Debt {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Escrow {
    debt_id: u64,
    amount: u64,
    created_at: u64,
}

impl Storable for Escrow {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Escrow {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct CropInsurance {
    id: u64,
    farmer: String,
    crop_type: String,
    coverage_amount: u64,
    coverage_start_date: u64,
    coverage_end_date: u64,
}

impl Storable for CropInsurance {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for CropInsurance {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct InsuranceClaim {
    insurance_id: u64,
    claim_amount: u64,
    claim_date: u64,
}

impl Storable for InsuranceClaim {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for InsuranceClaim {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static DEBT_STORAGE: RefCell<StableBTreeMap<u64, Debt, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static ESCROW_STORAGE: RefCell<StableBTreeMap<u64, Escrow, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static CROP_INSURANCE_STORAGE: RefCell<StableBTreeMap<u64, CropInsurance, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static INSURANCE_CLAIM_STORAGE: RefCell<StableBTreeMap<u64, InsuranceClaim, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct DebtPayload {
    debtor: String,
    creditor: String,
    amount: u64,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct EscrowPayload {
    debt_id: u64,
    amount: u64,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct CropInsurancePayload {
    farmer: String,
    crop_type: String,
    coverage_amount: u64,
    coverage_start_date: u64,
    coverage_end_date: u64,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct InsuranceClaimPayload {
    insurance_id: u64,
    claim_amount: u64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    InvalidInput { msg: String },
}

#[ic_cdk::query]
fn get_debt(id: u64) -> Result<Debt, Error> {
    match _get_debt(&id) {
        Some(debt) => Ok(debt),
        None => Err(Error::NotFound {
            msg: format!("a debt with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_escrow(debt_id: u64) -> Result<Escrow, Error> {
    match _get_escrow(&debt_id) {
        Some(escrow) => Ok(escrow),
        None => Err(Error::NotFound {
            msg: format!("escrow for debt_id={} not found", debt_id),
        }),
    }
}

#[ic_cdk::query]
fn get_crop_insurance(id: u64) -> Result<CropInsurance, Error> {
    match _get_crop_insurance(&id) {
        Some(insurance) => Ok(insurance),
        None => Err(Error::NotFound {
            msg: format!("crop insurance with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_insurance_claim(claim_id: u64) -> Result<InsuranceClaim, Error> {
    match _get_insurance_claim(&claim_id) {
        Some(claim) => Ok(claim),
        None => Err(Error::NotFound {
            msg: format!("insurance claim with id={} not found", claim_id),
        }),
    }
}

#[ic_cdk::update]
fn add_debt(debt: DebtPayload) -> Option<Debt> {
    // Validate input data
    if debt.debtor.is_empty() || debt.creditor.is_empty() || debt.amount == 0 {
        return None; // Invalid input, return early
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let debt = Debt {
        id,
        debtor: debt.debtor,
        creditor: debt.creditor,
        amount: debt.amount,
        created_at: time(),
    };

    do_insert_debt(&debt);
    Some(debt)
}

#[ic_cdk::update]
fn update_debt(id: u64, payload: DebtPayload) -> Result<Debt, Error> {
    // Validate input data
    if payload.debtor.is_empty() || payload.creditor.is_empty() || payload.amount == 0 {
        return Err(Error::InvalidInput {
            msg: "Invalid input data".to_string(),
        });
    }

    match DEBT_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut debt) => {
            debt.debtor = payload.debtor;
            debt.creditor = payload.creditor;
            debt.amount = payload.amount;
            do_insert_debt(&debt);
            Ok(debt)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update a debt with id={}. debt not found",
                id
            ),
        }),
    }
}

#[ic_cdk::update]
fn create_escrow(payload: EscrowPayload) -> Result<Escrow, Error> {
    // Validate input data
    if payload.amount == 0 {
        return Err(Error::InvalidInput {
            msg: "Invalid escrow amount".to_string(),
        });
    }

    match DEBT_STORAGE.with(|service| service.borrow().get(&payload.debt_id)) {
        Some(_) => {
            let _id = ID_COUNTER
                .with(|counter| {
                    let current_value = *counter.borrow().get();
                    counter.borrow_mut().set(current_value + 1)
                })
                .expect("cannot increment id counter");

            let escrow = Escrow {
                debt_id: payload.debt_id,
                amount: payload.amount,
                created_at: time(),
            };

            do_insert_escrow(&escrow);
            Ok(escrow)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't create escrow for debt_id={}. debt not found",
                payload.debt_id
            ),
        }),
    }
}

#[ic_cdk::update]
fn purchase_crop_insurance(payload: CropInsurancePayload) -> Option<CropInsurance> {
    // Validate input data
    if payload.farmer.is_empty() || payload.crop_type.is_empty() || payload.coverage_amount == 0 {
        return None; // Invalid input, return early
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let insurance = CropInsurance {
        id,
        farmer: payload.farmer,
        crop_type: payload.crop_type,
        coverage_amount: payload.coverage_amount,
        coverage_start_date: payload.coverage_start_date,
        coverage_end_date: payload.coverage_end_date,
    };

    CROP_INSURANCE_STORAGE.with(|service| service.borrow_mut().insert(id, insurance.clone()));
    Some(insurance)
}

#[ic_cdk::update]
fn submit_insurance_claim(payload: InsuranceClaimPayload) -> Result<InsuranceClaim, Error> {
    match CROP_INSURANCE_STORAGE.with(|service| service.borrow().get(&payload.insurance_id)) {
        Some(_insurance) => {
            let claim_id = ID_COUNTER
                .with(|counter| {
                    let current_value = *counter.borrow().get();
                    counter.borrow_mut().set(current_value + 1)
                })
                .expect("cannot increment id counter");

            let claim = InsuranceClaim {
                insurance_id: payload.insurance_id,
                claim_amount: payload.claim_amount,
                claim_date: time(),
            };

            INSURANCE_CLAIM_STORAGE.with(|service| service.borrow_mut().insert(claim_id, claim.clone()));
            Ok(claim)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't submit a claim for crop insurance with id={}. insurance not found",
                payload.insurance_id
            ),
        }),
    }
}

fn do_insert_debt(debt: &Debt) {
    DEBT_STORAGE.with(|service| service.borrow_mut().insert(debt.id, debt.clone()));
}

fn do_insert_escrow(escrow: &Escrow) {
    ESCROW_STORAGE
        .with(|service| service.borrow_mut().insert(escrow.debt_id, escrow.clone()));
}

fn _get_debt(id: &u64) -> Option<Debt> {
    DEBT_STORAGE.with(|service| service.borrow().get(id))
}

fn _get_escrow(debt_id: &u64) -> Option<Escrow> {
    ESCROW_STORAGE.with(|service| service.borrow().get(debt_id))
}

fn _get_crop_insurance(id: &u64) -> Option<CropInsurance> {
    CROP_INSURANCE_STORAGE.with(|service| service.borrow().get(id))
}

fn _get_insurance_claim(claim_id: &u64) -> Option<InsuranceClaim> {
    INSURANCE_CLAIM_STORAGE.with(|service| service.borrow().get(claim_id))
}


    // need this to generate candid
    ic_cdk::export_candid!();
