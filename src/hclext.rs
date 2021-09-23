use std::collections::HashMap;

use crate::proto::hashicorp::nomad::plugins::shared::hclspec::{
    Array, Attr, Block, BlockAttrs, BlockList, BlockMap, BlockSet, Default, Literal, Object, Spec,
};

use crate::proto::hashicorp::nomad::plugins::shared::hclspec::spec::Block as BlockEnum;

pub fn object_spec(obj: Object) -> Spec {
    Spec {
        block: Some(BlockEnum::Object(obj)),
    }
}

pub fn array_spec(array: Array) -> Spec {
    Spec {
        block: Some(BlockEnum::Array(array)),
    }
}

pub fn attr_spec(attr: Attr) -> Spec {
    Spec {
        block: Some(BlockEnum::Attr(attr)),
    }
}

pub fn block_spec(block: Block) -> Spec {
    Spec {
        block: Some(BlockEnum::BlockValue(Box::from(block))),
    }
}

pub fn block_attr_spec(block_attrs: BlockAttrs) -> Spec {
    Spec {
        block: Some(BlockEnum::BlockAttrs(block_attrs)),
    }
}

pub fn block_list_spec(block_list: BlockList) -> Spec {
    Spec {
        block: Some(BlockEnum::BlockList(Box::from(block_list))),
    }
}

pub fn block_map_spec(block_map: BlockMap) -> Spec {
    Spec {
        block: Some(BlockEnum::BlockMap(Box::from(block_map))),
    }
}

pub fn block_set_spec(block_set: BlockSet) -> Spec {
    Spec {
        block: Some(BlockEnum::BlockSet(Box::from(block_set))),
    }
}

pub fn default_spec(default: Default) -> Spec {
    Spec {
        block: Some(BlockEnum::Default(Box::from(default))),
    }
}

pub fn literal_spec(literal: Literal) -> Spec {
    Spec {
        block: Some(BlockEnum::Literal(literal)),
    }
}

// prefix functions that build hclspec types from primitives with new_
pub fn new_object_spec(attrs: HashMap<String, Spec>) -> Spec {
    object_spec(Object { attributes: attrs })
}

pub fn new_attr_spec(name: String, attr_type: String, required: bool) -> Spec {
    attr_spec(Attr {
        name,
        required,
        r#type: attr_type.to_string(),
    })
}

pub fn new_block_spec(name: String, required: bool, nested: Spec) -> Spec {
    block_spec(Block {
        name,
        required,
        nested: Some(Box::from(nested)),
    })
}

pub fn new_block_attrs_spec(name: String, elem_type: String, required: bool) -> Spec {
    block_attr_spec(BlockAttrs {
        name,
        r#type: elem_type.to_string(),
        required,
    })
}

pub fn new_block_list_spec(name: String, nested: Spec) -> Spec {
    new_block_list_limited_spec(name, 0, 0, nested)
}

pub fn new_block_list_limited_spec(
    name: String,
    min_items: u64,
    max_items: u64,
    nested: Spec,
) -> Spec {
    block_list_spec(BlockList {
        name,
        min_items,
        max_items,
        nested: Some(Box::from(nested)),
    })
}

pub fn new_block_set_spec(name: String, nested: Spec) -> Spec {
    new_block_set_limited_spec(name, 0, 0, nested)
}

pub fn new_block_set_limited_spec(
    name: String,
    min_items: u64,
    max_items: u64,
    nested: Spec,
) -> Spec {
    block_set_spec(BlockSet {
        name,
        min_items,
        max_items,
        nested: Some(Box::from(nested)),
    })
}

pub fn new_block_map_spec(name: String, labels: Vec<String>, nested: Spec) -> Spec {
    block_map_spec(BlockMap {
        name,
        labels,
        nested: Some(Box::from(nested)),
    })
}

pub fn new_literal_spec(value: String) -> Spec {
    literal_spec(Literal { value })
}

pub fn new_default_spec(primary: Spec, default: Spec) -> Spec {
    default_spec(Default {
        primary: Some(Box::from(primary)),
        default: Some(Box::from(default)),
    })
}

pub fn new_array_spec(values: Vec<Spec>) -> Spec {
    array_spec(Array { values })
}
