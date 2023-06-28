use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum CreateNodePullTaintEffectType {
    #[serde(rename = "NoSchedule")]
    NoSchedule,
    #[serde(rename = "PreferNoSchedule")]
    PreferNoSchedule,
    #[serde(rename = "NoExecute")]
    NoExecute,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodePullTaintModel {
    pub key: String,
    pub value: String,
    pub effect: CreateNodePullTaintEffectType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodePoolModel {
    pub id: String,
    pub name: String,
    pub size: String,
    pub count: i32,
    pub tags: Vec<String>,
    pub taints: Vec<NodePullTaintModel>,
    pub auto_scale: bool,
    pub min_nodes: i32,
    pub max_nodes: i32,
    pub nodes: Vec<NodePoolNodeModel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodePullResponse {
    pub node_pool: NodePoolModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NodePoolStatus {
    #[serde(rename = "provisioning")]
    Provisioning,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "draining")]
    Draining,
    #[serde(rename = "deleting")]
    Deleting,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct NodePoolStatusModel{
    pub state: NodePoolStatus
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodePoolNodeModel {
    pub id: String,
    pub name: String,
    pub status: NodePoolStatusModel,
    pub droplet_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNodePoolRequest {
    pub name: String,
    pub size: String,
    pub labels: HashMap<String, String>,
    pub count: i32,
    pub tags: Vec<String>,
    pub taints: Option<Vec<NodePullTaintModel>>,
    pub auto_scale: bool,
    pub min_nodes: i32,
    pub max_nodes: i32,
}
