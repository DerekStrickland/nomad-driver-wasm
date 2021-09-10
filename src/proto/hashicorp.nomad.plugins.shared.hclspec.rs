/// Spec defines the available specification types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Spec {
    #[prost(oneof = "spec::Block", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10")]
    pub block: ::core::option::Option<spec::Block>,
}
/// Nested message and enum types in `Spec`.
pub mod spec {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Block {
        #[prost(message, tag = "1")]
        Object(super::Object),
        #[prost(message, tag = "2")]
        Array(super::Array),
        /// buf:lint:ignore FIELD_LOWER_SNAKE_CASE
        #[prost(message, tag = "3")]
        Attr(super::Attr),
        #[prost(message, tag = "4")]
        BlockValue(::prost::alloc::boxed::Box<super::Block>),
        #[prost(message, tag = "5")]
        BlockAttrs(super::BlockAttrs),
        #[prost(message, tag = "6")]
        BlockList(::prost::alloc::boxed::Box<super::BlockList>),
        #[prost(message, tag = "7")]
        BlockSet(::prost::alloc::boxed::Box<super::BlockSet>),
        #[prost(message, tag = "8")]
        BlockMap(::prost::alloc::boxed::Box<super::BlockMap>),
        #[prost(message, tag = "9")]
        Default(::prost::alloc::boxed::Box<super::Default>),
        #[prost(message, tag = "10")]
        Literal(super::Literal),
    }
}
/// Attr spec type reads the value of an attribute in the current body
///and returns that value as its result. It also creates validation constraints
///for the given attribute name and its value.
///
///```hcl
///Attr {
///name     = "document_root"
///type     = string
///required = true
///}
///```
///
///`Attr` spec blocks accept the following arguments:
///
/// `name` (required) - The attribute name to expect within the HCL input file.
///This may be omitted when a default name selector is created by a parent
///`Object` spec, if the input attribute name should match the output JSON
///object property name.
///
/// `type` (optional) - A [type expression](#type-expressions) that the given
///attribute value must conform to. If this argument is set, `hcldec` will
///automatically convert the given input value to this type or produce an
///error if that is not possible.
///
/// `required` (optional) - If set to `true`, `hcldec` will produce an error
///if a value is not provided for the source attribute.
///
///`Attr` is a leaf spec type, so no nested spec blocks are permitted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attr {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub required: bool,
}
/// Block spec type applies one nested spec block to the contents of a
///block within the current body and returns the result of that spec. It also
///creates validation constraints for the given block type name.
///
///```hcl
///Block {
///name = "logging"
///
///Object {
///Attr "level" {
///type = string
///}
///Attr "file" {
///type = string
///}
///}
///}
///```
///
///`Block` spec blocks accept the following arguments:
///
/// `name` (required) - The block type name to expect within the HCL
///input file. This may be omitted when a default name selector is created
///by a parent `Object` spec, if the input block type name should match the
///output JSON object property name.
///
/// `required` (optional) - If set to `true`, `hcldec` will produce an error
///if a block of the specified type is not present in the current body.
///
///`Block` creates a validation constraint that there must be zero or one blocks
///of the given type name, or exactly one if `required` is set.
///
///`Block` expects a single nested spec block, which is applied to the body of
///the block of the given type when it is present.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub required: bool,
    #[prost(message, optional, boxed, tag = "3")]
    pub nested: ::core::option::Option<::prost::alloc::boxed::Box<Spec>>,
}
///
///The BlockAttrs spec type is similar to an Attr spec block of a map type,
///but it produces a map from the attributes of a block rather than from an
///attribute's expression.
///
///```hcl
///BlockAttrs {
///name     = "variables"
///type     = string
///required = false
///}
///```
///
///This allows a map with user-defined keys to be produced within block syntax,
///but due to the constraints of that syntax it also means that the user will
///be unable to dynamically-generate either individual key names using key
///expressions or the entire map value using a `for` expression.
///
///`BlockAttrs` spec blocks accept the following arguments:
///
/// `name` (required) - The block type name to expect within the HCL
///input file. This may be omitted when a default name selector is created
///by a parent `object` spec, if the input block type name should match the
///output JSON object property name.
///
/// `type` (required) - The value type to require for each of the
///attributes within a matched block. The resulting value will be a JSON
///object whose property values are of this type.
///
/// `required` (optional) - If `true`, an error will be produced if a block
///of the given type is not present. If `false` -- the default -- an absent
///block will be indicated by producing `null`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockAttrs {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub required: bool,
}
/// BlockList spec type is similar to `Block`, but it accepts zero or
///more blocks of a specified type rather than requiring zero or one. The
///result is a JSON array with one entry per block of the given type.
///
///```hcl
///BlockList {
///name = "log_file"
///
///Object {
///Attr "level" {
///type = string
///}
///Attr "filename" {
///type     = string
///required = true
///}
///}
///}
///```
///
///`BlockList` spec blocks accept the following arguments:
///
/// `name` (required) - The block type name to expect within the HCL
///input file. This may be omitted when a default name selector is created
///by a parent `Object` spec, if the input block type name should match the
///output JSON object property name.
///
/// `min_items` (optional) - If set to a number greater than zero, `hcldec` will
///produce an error if fewer than the given number of blocks are present.
///
/// `max_items` (optional) - If set to a number greater than zero, `hcldec` will
///produce an error if more than the given number of blocks are present. This
///attribute must be greater than or equal to `min_items` if both are set.
///
///`Block` creates a validation constraint on the number of blocks of the given
///type that must be present.
///
///`Block` expects a single nested spec block, which is applied to the body of
///each matching block to produce the resulting list items.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockList {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub min_items: u64,
    #[prost(uint64, tag = "3")]
    pub max_items: u64,
    #[prost(message, optional, boxed, tag = "4")]
    pub nested: ::core::option::Option<::prost::alloc::boxed::Box<Spec>>,
}
/// BlockSet spec type behaves the same as BlockList except that
///the result is in no specific order and any duplicate items are removed.
///
///```hcl
///BlockSet {
///name = "log_file"
///
///Object {
///Attr "level" {
///type = string
///}
///Attr "filename" {
///type     = string
///required = true
///}
///}
///}
///```
///
///The contents of `BlockSet` are the same as for `BlockList`.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockSet {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub min_items: u64,
    #[prost(uint64, tag = "3")]
    pub max_items: u64,
    #[prost(message, optional, boxed, tag = "4")]
    pub nested: ::core::option::Option<::prost::alloc::boxed::Box<Spec>>,
}
/// BlockMap spec type is similar to `Block`, but it accepts zero or
///more blocks of a specified type rather than requiring zero or one. The
///result is a JSON object, or possibly multiple nested JSON objects, whose
///properties are derived from the labels set on each matching block.
///
///```hcl
///BlockMap {
///name = "log_file"
///labels = ["filename"]
///
///Object {
///Attr "level" {
///type     = string
///required = true
///}
///}
///}
///```
///
///`BlockMap` spec blocks accept the following arguments:
///
/// `name` (required) - The block type name to expect within the HCL
///input file. This may be omitted when a default name selector is created
///by a parent `Object` spec, if the input block type name should match the
///output JSON object property name.
///
/// `labels` (required) - A list of user-oriented block label names. Each entry
///in this list creates one level of object within the output value, and
///requires one additional block header label on any child block of this type.
///Block header labels are the quoted strings that appear after the block type
///name but before the opening `{`.
///
///`Block` creates a validation constraint on the number of labels that blocks
///of the given type must have.
///
///`Block` expects a single nested spec block, which is applied to the body of
///each matching block to produce the resulting map items.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockMap {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, boxed, tag = "3")]
    pub nested: ::core::option::Option<::prost::alloc::boxed::Box<Spec>>,
}
/// Literal spec type returns a given literal value, and creates no
///validation constraints. It is most commonly used with the `Default` spec
///type to create a fallback value, but can also be used e.g. to fill out
///required properties in an `Object` spec that do not correspond to any
///construct in the input configuration.
///
///```hcl
///Literal {
///value = "hello world"
///}
///```
///
///`Literal` spec blocks accept the following argument:
///
/// `value` (required) - The value to return. This attribute may be an expression
///that uses [functions](#spec-definition-functions).
///
///`Literal` is a leaf spec type, so no nested spec blocks are permitted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Literal {
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// Default spec type evaluates a sequence of nested specs in turn and
///returns the result of the first one that produces a non-null value.
///It creates no validation constraints of its own, but passes on the validation
///constraints from its first nested block.
///
///```hcl
///Default {
///Attr {
///name = "private"
///type = bool
///}
///Literal {
///value = false
///}
///}
///```
///
///A `Default` spec block must have at least one nested spec block, and should
///generally have at least two since otherwise the `Default` wrapper is a no-op.
///
///The second and any subsequent spec blocks are _fallback_ specs. These exhibit
///their usual behavior but are not able to impose validation constraints on the
///current body since they are not evaluated unless all prior specs produce
///`null` as their result.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Default {
    #[prost(message, optional, boxed, tag = "1")]
    pub primary: ::core::option::Option<::prost::alloc::boxed::Box<Spec>>,
    #[prost(message, optional, boxed, tag = "2")]
    pub default: ::core::option::Option<::prost::alloc::boxed::Box<Spec>>,
}
/// Object spec type is the most commonly used at the root of a spec file.
///Its result is a JSON object whose properties are set based on any nested
///spec blocks:
///
///```hcl
///Object {
///Attr "name" {
///type = "string"
///}
///Block "address" {
///Object {
///Attr "street" {
///type = "string"
///}
///# ...
///}
///}
///}
///```
///
///Nested spec blocks inside `Object` must always have an extra block label
///`"name"`, `"address"` and `"street"` in the above example) that specifies
///the name of the property that should be created in the JSON object result.
///This label also acts as a default name selector for the nested spec, allowing
///the `Attr` blocks in the above example to omit the usually-required `name`
///argument in cases where the HCL input name and JSON output name are the same.
///
///An `Object` spec block creates no validation constraints, but it passes on
///any validation constraints created by the nested specs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Object {
    #[prost(map = "string, message", tag = "1")]
    pub attributes: ::std::collections::HashMap<::prost::alloc::string::String, Spec>,
}
/// Array spec type produces a JSON array whose elements are set based on
///any nested spec blocks:
///
///```hcl
///Array {
///Attr {
///name = "first_element"
///type = "string"
///}
///Attr {
///name = "second_element"
///type = "string"
///}
///}
///```
///
///An `Array` spec block creates no validation constraints, but it passes on
///any validation constraints created by the nested specs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Array {
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<Spec>,
}
