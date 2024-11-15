use crate::mixcomfy_types::*;
use icrc_ledger_types::icrc1::transfer::{BlockIndex, NumTokens};



impl MixComfy {
    pub fn get_comfy_nodes(&self) ->Option<Vec<ComfyUINode>>{
        Some(self.comfy_node.clone()) 
    }

    pub fn get_comfy_node_by_id(&self, node_id:u32) ->Result<ComfyUINode,MixComfyErr> {
        for node in self.comfy_node.iter() {
            if node.node_id == node_id {
                return Ok(node.clone())
            }
        }
        return Err(MixComfyErr::NoneNodeVaild(String::from("None nodes fund")))
    }
    //regedit new Comfy node
    pub fn reg_comfy_nodes(&mut self, nodes:Vec<ComfyUINode>) -> Result<(),MixComfyErr> {        
        for node in nodes.iter() {
            self.comfy_node.push(node.to_owned());
        }
        Ok(())
    }

    pub fn decide_comfy_node(&mut self) -> Option<ComfyUINode> {
        let mut mix_weight = -1;
        let mut curr_node_id:u32 = 0;

        for node in self.comfy_node.iter_mut() {
            if mix_weight == -1 {
                mix_weight = node.weight;
                curr_node_id = node.node_id;
            } else if mix_weight < node.weight {
                continue;
            } else {
                mix_weight = node.weight;
                curr_node_id = node.node_id;
            }
        }

        for node in self.comfy_node.iter() {
            if curr_node_id == node.node_id {
                return Some(node.clone())
            }
        }
        return None           
    }

    //For minning
    pub fn record_work_load(&mut self, record:ComfyUIPayload) ->Result<BlockIndex,MixComfyErr> {

        let top_index = self.workload_records.work_load_record.len() +1;
        let id = BlockIndex::try_from(top_index).unwrap();
        let record_item =WorkLoadLedgerItem {
            wkload_id : id.clone(),
            work_load: record
        };

        self.workload_records.work_load_record.push(record_item);
        Ok(id)
    }
 
}