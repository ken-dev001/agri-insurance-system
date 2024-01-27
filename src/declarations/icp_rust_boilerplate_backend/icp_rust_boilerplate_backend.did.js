export const idlFactory = ({ IDL }) => {
  const DebtPayload = IDL.Record({
    'debtor' : IDL.Text,
    'amount' : IDL.Nat64,
    'creditor' : IDL.Text,
  });
  const Debt = IDL.Record({
    'id' : IDL.Nat64,
    'debtor' : IDL.Text,
    'created_at' : IDL.Nat64,
    'amount' : IDL.Nat64,
    'creditor' : IDL.Text,
  });
  const EscrowPayload = IDL.Record({
    'debt_id' : IDL.Nat64,
    'amount' : IDL.Nat64,
  });
  const Escrow = IDL.Record({
    'debt_id' : IDL.Nat64,
    'created_at' : IDL.Nat64,
    'amount' : IDL.Nat64,
  });
  const Error = IDL.Variant({
    'InvalidInput' : IDL.Record({ 'msg' : IDL.Text }),
    'NotFound' : IDL.Record({ 'msg' : IDL.Text }),
  });
  const Result = IDL.Variant({ 'Ok' : Escrow, 'Err' : Error });
  const CropInsurance = IDL.Record({
    'id' : IDL.Nat64,
    'coverage_amount' : IDL.Nat64,
    'coverage_start_date' : IDL.Nat64,
    'coverage_end_date' : IDL.Nat64,
    'crop_type' : IDL.Text,
    'farmer' : IDL.Text,
  });
  const Result_1 = IDL.Variant({ 'Ok' : CropInsurance, 'Err' : Error });
  const Result_2 = IDL.Variant({ 'Ok' : Debt, 'Err' : Error });
  const InsuranceClaim = IDL.Record({
    'insurance_id' : IDL.Nat64,
    'claim_date' : IDL.Nat64,
    'claim_amount' : IDL.Nat64,
  });
  const Result_3 = IDL.Variant({ 'Ok' : InsuranceClaim, 'Err' : Error });
  const CropInsurancePayload = IDL.Record({
    'coverage_amount' : IDL.Nat64,
    'coverage_start_date' : IDL.Nat64,
    'coverage_end_date' : IDL.Nat64,
    'crop_type' : IDL.Text,
    'farmer' : IDL.Text,
  });
  const InsuranceClaimPayload = IDL.Record({
    'insurance_id' : IDL.Nat64,
    'claim_amount' : IDL.Nat64,
  });
  return IDL.Service({
    'add_debt' : IDL.Func([DebtPayload], [IDL.Opt(Debt)], []),
    'create_escrow' : IDL.Func([EscrowPayload], [Result], []),
    'get_crop_insurance' : IDL.Func([IDL.Nat64], [Result_1], ['query']),
    'get_debt' : IDL.Func([IDL.Nat64], [Result_2], ['query']),
    'get_escrow' : IDL.Func([IDL.Nat64], [Result], ['query']),
    'get_insurance_claim' : IDL.Func([IDL.Nat64], [Result_3], ['query']),
    'purchase_crop_insurance' : IDL.Func(
        [CropInsurancePayload],
        [IDL.Opt(CropInsurance)],
        [],
      ),
    'submit_insurance_claim' : IDL.Func(
        [InsuranceClaimPayload],
        [Result_3],
        [],
      ),
    'update_debt' : IDL.Func([IDL.Nat64, DebtPayload], [Result_2], []),
  });
};
export const init = ({ IDL }) => { return []; };
