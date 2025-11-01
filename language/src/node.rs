use std::{collections::HashMap, fmt::Error};

use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::language::c::writers::node_writer::node_type::NodeType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Node {
    pub id: Uuid,
    pub node_type: u64,
    pub content: String,
    pub tags: HashMap<String, Vec<Node>>,
    pub children: Vec<Node>,
}

pub fn serialize(nodes: &Vec<Node>) -> Result<Vec<u8>, Error> {
    let bin = bincode::serialize(nodes).unwrap();
    Ok(bin)
}

pub trait UuidFromSeed {
    fn from_seed(seed: &str) -> Uuid;
}

impl UuidFromSeed for Uuid {
    fn from_seed(seed: &str) -> Uuid {
        let value = u128_from_seed(seed);
        Uuid::from_u128(value)
    }
}

fn u128_from_seed(seed: &str) -> u128 {
    let mut hasher = Sha256::new();
    hasher.update(seed.as_bytes());
    let digest = hasher.finalize(); // 32 bytes

    // Take the first 16 bytes as a big-endian u128
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&digest[..16]);
    u128::from_be_bytes(bytes)
}

pub trait ToNode {
    /// Converts the current value into a Node with a symbol tag.
    ///
    /// Example:
    ///
    /// ```
    /// use uuid::Uuid;
    /// use std::collections::HashMap;
    /// use language::{node::{Node, ToNode}, language::c::writers::node_writer::node_type::NodeType};
    /// let id = Uuid::new_v4();
    /// let node = "example".to_symbol_node(id);
    /// assert_eq!(node.id, id);
    /// assert_eq!(node.content, "example");
    /// assert_eq!(node.node_type, NodeType::Symbol.as_u64());
    /// assert_eq!(node.tags, HashMap::new());
    /// assert_eq!(node.children, Vec::<Node>::new());
    /// ```
    fn to_symbol_node(self, id: Uuid) -> Node;

    /// Converts the current value into a Node whose id is the string representation of the value.
    fn to_str_node(self, node_type: u64) -> Node;
}

// TODO this have C specific types. Move to C module
impl ToNode for String {
    fn to_symbol_node(self, id: Uuid) -> Node {
        Node {
            id,
            node_type: NodeType::Symbol.as_u64(),
            content: self,
            tags: HashMap::new(),
            children: vec![],
        }
    }

    fn to_str_node(self, node_type: u64) -> Node {
        Node {
            node_type,
            id: Uuid::new_v4(),
            content: self,
            tags: HashMap::new(),
            children: vec![],
        }
    }
}

impl ToNode for &str {
    fn to_symbol_node(self, id: Uuid) -> Node {
        self.to_string().to_symbol_node(id)
    }

    fn to_str_node(self, node_type: u64) -> Node {
        self.to_string().to_str_node(node_type)
    }
}

impl ToNode for Uuid {
    fn to_symbol_node(self, id: Uuid) -> Node {
        self.to_string().to_symbol_node(id)
    }

    fn to_str_node(self, node_type: u64) -> Node {
        self.to_string().to_str_node(node_type)
    }
}

pub trait ToTags {
    fn to_tags(self) -> HashMap<String, Vec<Node>>;
}

impl ToTags for Vec<(&str, Vec<Node>)> {
    fn to_tags(self) -> HashMap<String, Vec<Node>> {
        self.into_iter().map(|(k, v)| (k.to_string(), v)).collect()
    }
}

impl ToTags for Vec<(String, Vec<Node>)> {
    fn to_tags(self) -> HashMap<String, Vec<Node>> {
        self.into_iter().collect()
    }
}

impl ToTags for Vec<(&str, &str, u64)> {
    fn to_tags(self) -> HashMap<String, Vec<Node>> {
        self.into_iter()
            .map(|(k, v, node_type)| (k.to_string(), vec![v.to_str_node(node_type)]))
            .collect()
    }
}
