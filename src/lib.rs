#[allow(unused)]
mod pb;

use pb::mydata::v1 as mydata;
use pb::sf::substreams::solana::v1::Transactions;

#[substreams::handlers::map]
fn map_my_data(transactions: Transactions) -> mydata::MyData {
    let mut my_data = mydata::MyData::default();

    for trx in transactions.transactions {

        // Skip failed transactions
        if let Some(meta) = trx.meta {
            if meta.err.is_some() {
                continue;
            }
        }

        // Safely extract transaction message
        let Some(tx) = trx.transaction else { continue; };
        let Some(msg) = tx.message else { continue; };

        let account_keys = msg.account_keys;

        for instr in msg.instructions {

            // Check program index safely
            let program_index = instr.program_id_index as usize;
            if program_index >= account_keys.len() {
                continue;
            }

            let program_id =
                bs58::encode(&account_keys[program_index]).into_string();

            // Only System Program transfers
            if program_id != "11111111111111111111111111111111" {
                continue;
            }

            // Need at least from and to accounts
            if instr.accounts.len() < 2 {
                continue;
            }

            let from_index = instr.accounts[0] as usize;
            let to_index = instr.accounts[1] as usize;

            if from_index >= account_keys.len() || to_index >= account_keys.len() {
                continue;
            }

            let from =
                bs58::encode(&account_keys[from_index]).into_string();
            let to =
                bs58::encode(&account_keys[to_index]).into_string();

            // Extract lamports amount
            let amount = if instr.data.len() >= 12 {
                let lamports_bytes = &instr.data[4..12];
                u64::from_le_bytes(
                    lamports_bytes.try_into().unwrap_or([0; 8])
                )
            } else {
                0
            };

            if amount == 0 {
                continue;
            }

            // Format JSON output
            let json = format!(
                r#"{{"from":"{}","to":"{}","amount":{}}}"#,
                from, to, amount
            );

            my_data.transfers.push(json);

            // RETURN immediately after first transfer
            return my_data;
        }
    }

    // If nothing found, return empty result
    my_data
}
