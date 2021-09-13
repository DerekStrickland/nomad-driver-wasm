use super::hclspec::{Attr, Array, BlockAttrs, BlockList, BlockMap, BlockSet, BlockSpec, Default, Literal, Object, Spec};
use super::hclspec::spec::{Block};
use std::collections::HashMap;

pub fn object_spec(obj: Object) -> Spec {
    return Spec {
        block: Some(Block::Object(obj))
    }
}

pub fn array_spec(array: Array) -> Spec {
    return Spec {
        block: Some(Block::Array(array))
    }
}

pub fn attr_spec(attr: Attr) -> Spec {
    return Spec {
        block: Some(Block::Attr(attr))
    }
}

pub fn block_spec(block: super::hclspec::Block) -> Spec {
    return Spec {
        block: Some(Block::BlockValue(Box::from(block)))
    }
}

pub fn block_attr_spec(blockAttrs: BlockAttrs) -> Spec {
    return Spec {
        block: Some(Block::BlockAttrs(blockAttrs))
    }
}

pub fn block_list_spec(blockList: BlockList) -> Spec {
    return Spec {
        block: Some(Block::BlockList(Box::from(blockList)))
    }
}

pub fn block_map_spec(blockMap: BlockMap) -> Spec {
    return Spec{
        block: Some(Block::BlockMap(Box::from(blockMap)))
    }
}

pub fn block_set_spec(blockSet: BlockSet) -> Spec {
    return Spec {
        block: Some(Block::BlockSet(Box::from(blockSet)))
    }
}

pub fn default_spec(def: Default) -> Spec {
    return Spec {
        block: Some(Block::Default(Box::from(def)))
    }
}

pub fn literal_spec(lit: Literal) -> Spec {
    return Spec {
        block: Some(Block::Literal(lit))
    }
}

pub fn new_object_spec(attrs: HashMap<String, Spec>) -> Spec {
    return object_spec(Object {
        attributes: attrs
    })
}

// prefix functions that build hclspec types from primitives with new_
pub fn new_attr_spec(name: String, attr_type: String, required: bool) -> Spec {
    return attr_spec(Attr {
            name,
            required,
            r#type: attr_type.to_string()
        }
    )
}

pub fn new_block_spec(name: string, required: bool, nested: Spec) -> Spec {
    return block_spec(super::hclspec::Block {
        name,
        required,
        nested: Some(Box::from(nested))
    })
}

pub fn new_block_attrs_spec(name: String, elem_type: String, required: bool) -> Spec {
    return block_attr_spec(BlockAttrs{
        name,
        r#type: elem_type.to_string(),
        required
    })
}

pub fn new_block_list_spec(name: String, nested: Spec) -> Spec {
    return new_block_list_limited_spec(name, 0, 0, nested)
}

pub fn new_block_list_limited_spec(name: String, min_items: u64, max_items: u64, nested: Spec) -> Spec {
    return block_list_spec(BlockList{
        name,
        min_items,
        max_items,
        nested: Some(Box::from(nested))
    })
}

pub fn new_block_set_spec(name: String, nested: Spec) -> Spec {
    return new_block_set_limited_spec(name, 0, 0, nested)
}

pub fn new_block_set_limited_spec(name: String, min_items: u64, max_items: u64, nested: Spec) -> Spec {
    return block_set_spec(BlockSet{
        name,
        min_items,
        max_items,
        nested: Some(Box::from(nested))
    })
}

pub fn new_block_map_spec(name: String, labels: Vec<String>, nested: Spec) -> Spec {
    return block_map_spec(BlockMap{
        name,
        labels,
        nested: Some(Box::from(nested))
    })
}

pub fn new_literal_spec(value: String) -> Spec {
    return literal_spec(Literal{
        value
    })
}

pub fn new_default_spec(primary:Spec, default: Spec) -> Spec {
    return default_spec(Default{
        primary: Some(Box::from(primary)),
        default: Some(Box::from(default))
    })
}

pub fn new_array_spec(values: Vec<Spec>) -> Spec {
    return array_spec(Array{
        values
    })
}