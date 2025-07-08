# Ahindra D. - Advanced Pre-requisites

Probable Optimizations in Sequence - Codebase annotation in comments below
AdvanceSVM Repo: https://github.com/AhindraD/turbin3_advance_svm/tree/main/pre-req-improvs

## OPTIMIZATION 1: Account Deduplication - O(n²) → O(n)

In InvokeContext impl ---> prepare_instruction() --> Duplicate account const search O(1)
https://github.com/anza-xyz/agave/blob/825f16d3d5d0349b012a3a53e627d136312859f0/program-runtime/src/invoke_context.rs#L45 - Line 325

```rs
#[allow(clippy::type_complexity)]
pub fn prepare_instruction(
    &mut self,
    instruction: &StableInstruction,
    signers: &[Pubkey],
) -> Result<(Vec<InstructionAccount>, Vec<IndexOfAccount>), InstructionError> {
    //Probable Optimization[O(n²) → O(n*1)] - using HashMap - O(1) lookup - PITFALL: heap allocation
    use std::collections::HashMap;
    let mut account_index_map: HashMap<IndexOfAccount, usize> = HashMap::new();

    let mut deduplicated_instruction_accounts: Vec<InstructionAccount> = Vec::new();
    let mut duplicate_indicies = Vec::with_capacity(instruction.accounts.len() as usize);

    for (instruction_account_index, account_meta) in instruction.accounts.iter().enumerate() {
        let index_in_transaction = self
            .transaction_context
            .find_index_of_account(&account_meta.pubkey)
            .ok_or_else(|| {
                ic_msg!(
                    self,
                    "Instruction references an unknown account {}",
                    account_meta.pubkey,
                );
                InstructionError::MissingAccount
            })?

        // O(1) lookup instead of O(n) linear search
        if let Some(&duplicate_index) = account_index_map.get(&index_in_transaction) {
            // Found duplicate - update existing entry
            duplicate_indices.push(duplicate_index);
            let instruction_account = &mut deduplicated_instruction_accounts[duplicate_index];
            // update privileges
            instruction_account.is_signer |= account_meta.is_signer;
            instruction_account.is_writable |= account_meta.is_writable;
        } else {
            // New account - add to both map and vector
            let index_in_caller = self.get_index_in_caller(&account_meta.pubkey)?;
            let new_index = deduplicated_instruction_accounts.len();

            account_index_map.insert(index_in_transaction, new_index);
            duplicate_indices.push(new_index);

            deduplicated_instruction_accounts.push(InstructionAccount {
                index_in_transaction,
                index_in_caller,
                index_in_callee: instruction_account_index as IndexOfAccount,
                is_signer: account_meta.is_signer,
                is_writable: account_meta.is_writable,
            });
        }
    }
}

```
