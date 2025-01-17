
use candid::{Principal, Nat};
use ic_cdk_macros::{init, query, update};
use std::cell::RefCell;
use ic_stable_structures::{Memory, DefaultMemoryImpl, memory_manager::{MemoryId, VirtualMemory,MemoryManager}, StableVec};
use serde::{Serialize, Deserialize};

use crate::mixcomfy_types::{ClientPrompt, WorkflowLedgerItem, WorkflowLedger, ClientPromptStore};


thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = 
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
        
    static WORKFLOW_DATA: RefCell<StableVec<ClientPromptStore, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
            StableVec::init(
                MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
            ).expect("Failed to initialize WORKFLOW_DATA")
        );
    
        static WORKFLOW_LEDGER: RefCell<StableVec<WorkflowLedgerItem, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
            StableVec::init(
                MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
            ).expect("Failed to initialize WORKFLOW_LEDGER") 
        );

}


#[update]
fn store_workflow_data(principal_id: String, prompt_json: String) {
    let client_prompt = match ClientPrompt::from_json(&prompt_json) {
        Ok(prompt) => prompt,
        Err(err) => {
            ic_cdk::println!("Failed to parse prompt JSON: {}", err);
            return;
        }
    };

    let client_id = client_prompt.client_id.clone();

    WORKFLOW_DATA.with(|data| {
        let mut data = data.borrow_mut();
        let store = ClientPromptStore(principal_id.clone(), vec![client_prompt]);
        data.push(&store).expect("Failed to store workflow data");
    });

    // Record the rewards token and set the status to 'WAITCLAIM'
    // Assuming you have a function to handle this logic
    record_rewards_and_set_status(&principal_id, client_id);

    ic_cdk::println!("Workflow data stored for principal_id: {}", principal_id);
}



fn record_rewards_and_set_status(principal_id: &String, client_id: String) {
    let timestamp = ic_cdk::api::time();
    let token_reward = Nat::from(10 as u32); // Example token reward, adjust as needed
    let status = "WAITCLAIM".to_string();

    let workflow_id = format!("{}-{}", principal_id, timestamp); // Example workflow ID generation

    let ledger_item = WorkflowLedgerItem {
        principal_id: principal_id.clone(),
        client_id: client_id.clone(),
        workflow_id,
        timestamp,
        token_reward,
        status: status.clone(),
    };

    WORKFLOW_LEDGER.with(|ledger| {
        ledger.borrow_mut().push(&ledger_item).expect("Failed to push ledger item");
    });

    ic_cdk::println!("Rewards recorded and status set to '{}' for principal_id: {}, client_id: {}", status, principal_id, client_id);
   
}
#[query]
fn fetch_workflow_data() -> String {
    WORKFLOW_DATA.with(|data| {
        let data = data.borrow();
        let result: Vec<_> = (0..data.len())
            .filter_map(|i| data.get(i))
            .collect();
        serde_json::to_string(&result).unwrap_or_else(|_| "[]".to_string())
    })
}

