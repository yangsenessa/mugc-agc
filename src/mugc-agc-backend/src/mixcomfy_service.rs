use ic_cdk::caller;
use crate::mixcomfy_types::*;
use candid::{Nat, Principal};

//


impl MixComfy {
    pub fn get_comfy_nodes(&self) ->Option<Vec<ComfyUINode>>{
        Some(self.comfy_node.clone()) 
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
 
}