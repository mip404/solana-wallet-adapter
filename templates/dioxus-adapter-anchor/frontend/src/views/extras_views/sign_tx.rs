use dioxus::prelude::*;
use wallet_adapter::Utils;

use crate::{
    fetch_parser::get_blockhash, NotificationInfo, SignTxSvg, ACTIVE_CONNECTION, CLUSTER_STORAGE,
    GLOBAL_MESSAGE, WALLET_ADAPTER,
};
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use solana_transaction::Transaction;

#[component]
pub fn SignTx() -> Element {
    let mut public_key = [0u8; 32];

    if let Ok(wallet_account) = ACTIVE_CONNECTION.read().connected_account() {
        public_key = wallet_account.public_key();
    }

    let idl_buffer = include_bytes!(concat!(
        env!("CARGO_WORKSPACE_DIR"),
        "/target/idl/{{crate_name}}_anchor.json"
    ));

    let parsed_idl = anchor_lang_idl::convert::convert_idl(idl_buffer).unwrap();
    let initialize_ix = parsed_idl
        .instructions
        .iter()
        .find(|instruction| instruction.name.as_bytes() == b"initialize")
        .unwrap();

    let discriminant = initialize_ix.discriminator.clone();

    let program_id = parsed_idl.address;

    rsx! {
        div { class:"flex dark:bg-[#160231] bg-white flex-col w-[300px] p-5 rounded-lg dark:shadow-2xl shadow-sm border dark:border-none",
            div {class:"w-full flex flex-col items-center text-center text-true-blue justify-center mb-10",
                div{class:"w-[80px] flex flex-col", {SignTxSvg()}}
                div{class:"w-full text-sm", "Sign Transaction"}
            }
            div { class:"text-lg text-center",
            "Greetings from " {Utils::shorten_base58(&program_id).map(|value|value.to_string()).unwrap_or("Error: Invalid Base58 program ID".to_string())} " program!"
            }

        div { class:"flex items-center justify-center",
                button{
                    class: "bg-true-blue  hover:bg-cobalt-blue mt-5 text-sm text-white px-5 py-2 rounded-full",
                    onclick: move |_| {
                        let discriminant = discriminant.clone();
                        let program_id = program_id.clone();
                        spawn(async move {
                            let pubkey = Pubkey::new_from_array(public_key);

                            let program_id = Pubkey::from_str_const(&program_id);

                            let ix = Instruction {
                                program_id,
                                accounts: vec![],
                                data: discriminant,
                            };

                            match get_blockhash().await {
                                Err(error) => {
                                    let notification =  NotificationInfo::error(format!("Unable to get the blockhash. This transactions is likely to fail. Error: {error:?}!"));
                                    GLOBAL_MESSAGE.write().entry(*notification.key()).or_insert(notification);
                                },
                                Ok(blockhash) => {
                                    let mut tx = Transaction::new_with_payer(&[ix], Some(&pubkey));
                                    tx.message.recent_blockhash = blockhash;
                                    let tx_bytes = bincode::serialize(&tx).unwrap();
                                    let cluster = CLUSTER_STORAGE.read().active_cluster().cluster();

                                    match WALLET_ADAPTER.read().sign_transaction(&tx_bytes, Some(cluster)).await{
                                        Err(error) => {
                                            let notification =  NotificationInfo::error(
                                                    format!("SIGN MESSAGE ERROR: {error:?}")
                                                );
                                                GLOBAL_MESSAGE.write().entry(*notification.key()).or_insert(
                                                   notification
                                                );
                                        },
                                        Ok(output) => {
                                            if let Err(error) = bincode::deserialize::<Transaction>(&output[0]){
                                              let notification =  NotificationInfo::error(
                                                        format!("SIGN TX ERROR: {error:?}")
                                                    );
                                                GLOBAL_MESSAGE.write().entry(*notification.key()).or_insert(
                                                   notification
                                                );
                                            }else {
                                                let notification =  NotificationInfo::new("Sign Transaction Successful");
                                                GLOBAL_MESSAGE.write().entry(*notification.key()).or_insert(
                                                   notification
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        });
                    },
                    "SIGN TRANSACTION"
                }
            }
        }
    }
}
