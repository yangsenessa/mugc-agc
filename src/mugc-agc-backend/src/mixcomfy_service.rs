mod http;

use ic_cdk::caller;
use crate::mixcomfy_types::*;
use candid::{Nat, Principal};

//
#[derive(Default)]
pub struct MixComfy{
    pub comfy_node:Vec<ComfyUINode>   
}

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
   
}