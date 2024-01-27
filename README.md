# Agri Insurance System

## Overview
The Agri Insurance System is a Rust-based implementation designed for the Internet Computer (IC) environment. This system manages agricultural-related financial transactions, including debts, escrows, crop insurance, and insurance claims. Leveraging the IC framework, it utilizes stable storage structures and memory management to ensure persistent and secure data storage.

## Dependencies
- `serde`: A serialization/deserialization library for Rust.
- `candid`: A library for Candid serialization used in IC.
- `ic_cdk`: The Internet Computer's Candid Development Kit, providing access to IC-specific functionality.
- `ic_stable_structures`: A library for stable storage structures and memory management in IC.

## Entities

### Debt
- Represents a financial obligation related to agricultural activities.
- Fields:
  - `id`: Unique identifier for the debt.
  - `debtor`: String representing the party responsible for the debt.
  - `creditor`: String representing the party owed the debt.
  - `amount`: The financial amount associated with the debt.
  - `created_at`: Timestamp indicating when the debt was created.

### Escrow
- Represents an amount of money held in trust for agricultural transactions.
- Fields:
  - `debt_id`: Identifier linking the escrow to a specific debt.
  - `amount`: The amount held in escrow.
  - `created_at`: Timestamp indicating when the escrow was created.

### CropInsurance
- Represents an insurance policy for agricultural crops.
- Fields:
  - `id`: Unique identifier for the insurance.
  - `farmer`: String representing the farmer who owns the insured crop.
  - `crop_type`: String representing the type of the insured crop.
  - `coverage_amount`: The coverage amount provided by the insurance.
  - `coverage_start_date`: Timestamp indicating when the insurance coverage starts.
  - `coverage_end_date`: Timestamp indicating when the insurance coverage ends.

### InsuranceClaim
- Represents a claim made against an agricultural insurance policy.
- Fields:
  - `insurance_id`: Identifier linking the claim to a specific crop insurance.
  - `claim_amount`: The amount claimed.
  - `claim_date`: Timestamp indicating when the claim was submitted.

## Memory and Storage
- The system uses a thread-local memory manager (`MEMORY_MANAGER`) for handling virtual memory.
- Unique identifiers (`ID_COUNTER`) generate sequential IDs for debts, escrows, insurance, and claims.
- Stable BTreeMaps (`DEBT_STORAGE`, `ESCROW_STORAGE`, `CROP_INSURANCE_STORAGE`, `INSURANCE_CLAIM_STORAGE`) store and manage data persistently.

## Functions

### Query Functions
1. `get_debt(id: u64) -> Result<Debt, Error>`: Retrieve details of a debt by its ID.
2. `get_escrow(debt_id: u64) -> Result<Escrow, Error>`: Retrieve details of an escrow by the associated debt ID.
3. `get_crop_insurance(id: u64) -> Result<CropInsurance, Error>`: Retrieve details of crop insurance by its ID.
4. `get_insurance_claim(claim_id: u64) -> Result<InsuranceClaim, Error>`: Retrieve details of an insurance claim by its ID.

### Update Functions
1. `add_debt(debt: DebtPayload) -> Option<Debt>`: Add a new debt with the provided payload.
2. `update_debt(id: u64, payload: DebtPayload) -> Result<Debt, Error>`: Update details of an existing debt.
3. `create_escrow(payload: EscrowPayload) -> Result<Escrow, Error>`: Create an escrow for a specified debt.
4. `purchase_crop_insurance(payload: CropInsurancePayload) -> Option<CropInsurance>`: Purchase crop insurance with the provided details.
5. `submit_insurance_claim(payload: InsuranceClaimPayload) -> Result<InsuranceClaim, Error>`: Submit an insurance claim for a specified crop insurance.

## Helper Functions
- `do_insert_debt(debt: &Debt)`: Helper function to insert a debt into the stable storage.
- `do_insert_escrow(escrow: &Escrow)`: Helper function to insert an escrow into the stable storage.
- `_get_debt(id: &u64) -> Option<Debt>`: Helper function to retrieve a debt by its ID.
- `_get_escrow(debt_id: &u64) -> Option<Escrow>`: Helper function to retrieve an escrow by the associated debt ID.
- `_get_crop_insurance(id: &u64) -> Option<CropInsurance>`: Helper function to retrieve crop insurance by its ID.
- `_get_insurance_claim(claim_id: &u64) -> Option<InsuranceClaim>`: Helper function to retrieve an insurance claim by its ID.

## Candid Export
- The system includes a macro (`ic_cdk::export_candid!()`) for exporting the Candid interface.

### How to Clone and Use

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/ken-dev001/agri-insurance-system.git
   cd  agri-insurance-system
   ```

2. **Install Dependencies:**
   ```bash
   npm install
   ```

3. **Run the Code:**
   ```bash
   dfx start
   ```
4. **Deploy the Code:**
   ```bash
   dfx deploy
   ```
## Usage
- The Agri Insurance System is intended for use within the Internet Computer environment, where it can be deployed as a canister.


