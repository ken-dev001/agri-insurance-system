#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

// Define memory types
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Define common trait implementations for Storable
trait CommonStorable: Storable + BoundedStorable {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implement CommonStorable for Debt
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Debt {
    id: u64,
    debtor: String,
    creditor: String,
    amount: u64,
    created_at: u64,
}

impl CommonStorable for Debt {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implement CommonStorable for Escrow
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Escrow {
    debt_id: u64,
    amount: u64,
    created_at: u64,
}

impl CommonStorable for Escrow {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implement CommonStorable for CropInsurance
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct CropInsurance {
    id: u64,
    farmer: String,
    crop_type: String,
    coverage_amount: u64,
    coverage_start_date: u64,
    coverage_end_date: u64,
}

impl CommonStorable for CropInsurance {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implement CommonStorable for InsuranceClaim
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct InsuranceClaim {
    insurance_id: u64,
    claim_amount: u64,
    claim_date: u64,
}

impl CommonStorable for InsuranceClaim {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Thread-local storage for various components
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

// Payload structs for transactions
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

// Error enum for transactions
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    InvalidInput { msg: String },
}

// Query function to get debt by ID
#[ic_cdk::query]
fn get_debt(id: u64) -> Result<Debt, Error> {
    match _get_debt(&id) {
        Some(debt) => Ok(debt),
        None => Err(Error::NotFound {
            msg: format!("a debt with id={} not found", id),
        }),
    }
}

// Query function to get escrow by debt ID
#[ic_cdk::query]
fn get_escrow(debt_id: u64) -> Result<Escrow, Error> {
    match _get_escrow(&debt_id) {
        Some(escrow) => Ok(escrow),
        None => Err(Error::NotFound {
            msg: format!("escrow for debt_id={} not found", debt_id),
        }),
    }
}

// Query function to get crop insurance by ID
#[ic_cdk::query]
fn get_crop_insurance(id: u64) -> Result<CropInsurance, Error> {
    match _get_crop_insurance(&id) {
        Some(insurance) => Ok(insurance),
        None => Err(Error::NotFound {
            msg: format!("crop insurance with id={} not found", id),
        }),
    }
}

// Query function to get insurance claim by ID
#[ic_cdk::query]
fn get_insurance_claim(claim_id: u64) -> Result<InsuranceClaim, Error> {
    match _get_insurance_claim(&claim_id) {
        Some(claim) => Ok(claim),
        None => Err(Error::NotFound {
            msg: format!("insurance claim with id={} not found", claim_id),
        }),
    }
}

// Update function to add a new debt
#[ic_cdk::update]
fn add_debt(debt: DebtPayload) -> Option<Debt> {
    // Validate input data
    if debt.debtor.is_empty() || debt.creditor.is_empty() || debt.amount == 0 {
        return None; // Invalid input, return early
    }

    // Generate a new ID
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    // Create a new Debt instance
    let new_debt = Debt {
        id,
        debtor: debt.debtor,
        creditor: debt.creditor,
        amount: debt.amount,
        created_at: time(),
    };

    // Insert the new debt into storage
    do_insert_debt(&new_debt);
    Some(new_debt)
}

// Update function to update an existing debt
#[ic_cdk::update]
fn update_debt(id: u64, payload: DebtPayload) -> Result<Debt, Error> {
    // Validate input data
    if payload.debtor.is_empty() || payload.creditor.is_empty() || payload.amount == 0 {
        return Err(Error::InvalidInput {
            msg: "Invalid input data".to_string(),
        });
    }

    // Try to find the existing debt
    match DEBT_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut debt) => {
            // Update the fields
            debt.debtor = payload.debtor;
            debt.creditor = payload.creditor;
            debt.amount = payload.amount;

            // Insert the updated debt into storage
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

// Update function to create a new escrow
#[ic_cdk::update]
fn create_escrow(payload: EscrowPayload) -> Result<Escrow, Error> {
    // Validate input data
    if payload.amount == 0 {
        return Err(Error::InvalidInput {
            msg: "Invalid escrow amount".to_string(),
        });
    }

    // Try to find the associated debt
    match DEBT_STORAGE.with(|service| service.borrow().get(&payload.debt_id)) {
        Some(_) => {
            // Generate a new ID
            let id = ID_COUNTER
                .with(|counter| {
                    let current_value = *counter.borrow().get();
                    counter.borrow_mut().set(current_value + 1)
                })
                .expect("cannot increment id counter");

            // Create a new Escrow instance
            let escrow = Escrow {
                debt_id: payload.debt_id,
                amount: payload.amount,
                created_at: time(),
            };

            // Insert the new escrow into storage
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

// Update function to purchase crop insurance
#[ic_cdk::update]
fn purchase_crop_insurance(payload: CropInsurancePayload) -> Option<CropInsurance> {
    // Validate input data
    if payload.farmer.is_empty()
        || payload.crop_type.is_empty()
        || payload.coverage_amount == 0
    {
        return None; // Invalid input, return early
    }

    // Generate a new ID
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    // Create a new CropInsurance instance
    let insurance = CropInsurance {
        id,
        farmer: payload.farmer,
        crop_type: payload.crop_type,
        coverage_amount: payload.coverage_amount,
        coverage_start_date: payload.coverage_start_date,
        coverage_end_date: payload.coverage_end_date,
    };

    // Insert the new insurance into storage
    CROP_INSURANCE_STORAGE.with(|service| service.borrow_mut().insert(id, insurance.clone()));
    Some(insurance)
}

// Update function to submit an insurance claim
#[ic_cdk::update]
fn submit_insurance_claim(payload: InsuranceClaimPayload) -> Result<InsuranceClaim, Error> {
    // Try to find the associated crop insurance
    match CROP_INSURANCE_STORAGE.with(|service| service.borrow().get(&payload.insurance_id)) {
        Some(_) => {
            // Generate a new ID
            let claim_id = ID_COUNTER
                .with(|counter| {
                    let current_value = *counter.borrow().get();
                    counter.borrow_mut().set(current_value + 1)
                })
                .expect("cannot increment id counter");

            // Create a new InsuranceClaim instance
            let claim = InsuranceClaim {
                insurance_id: payload.insurance_id,
                claim_amount: payload.claim_amount,
                claim_date: time(),
            };

            // Insert the new claim into storage
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

// Helper function to insert a debt into storage
fn do_insert_debt(debt: &Debt) {
    DEBT_STORAGE.with(|service| service.borrow_mut().insert(debt.id, debt.clone()));
}

// Helper function to insert an escrow into storage
fn do_insert_escrow(escrow: &Escrow) {
    ESCROW_STORAGE
        .with(|service| service.borrow_mut().insert(escrow.debt_id, escrow.clone()));
}

// Helper function to get a debt by ID
fn _get_debt(id: &u64) -> Option<Debt> {
    DEBT_STORAGE.with(|service| service.borrow().get(id))
}

// Helper function to get an escrow by debt ID
fn _get_escrow(debt_id: &u64) -> Option<Escrow> {
    ESCROW_STORAGE.with(|service| service.borrow().get(debt_id))
}

// Helper function to get crop insurance by ID
fn _get_crop_insurance(id: &u64) -> Option<CropInsurance> {
    CROP_INSURANCE_STORAGE.with(|service| service.borrow().get(id))
}

// Helper function to get an insurance claim by ID
fn _get_insurance_claim(claim_id: &u64) -> Option<InsuranceClaim> {
    INSURANCE_CLAIM_STORAGE.with(|service| service.borrow().get(claim_id))
}

// Export Candid
ic_cdk::export_candid!();
