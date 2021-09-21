/// Attribute is used to describe the value of an attribute, optionally
/// specifying units
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Attribute {
    /// unit gives the unit type: MHz, MB, etc.
    #[prost(string, tag = "5")]
    pub unit: ::prost::alloc::string::String,
    #[prost(oneof = "attribute::Value", tags = "1, 2, 3, 4")]
    pub value: ::core::option::Option<attribute::Value>,
}
/// Nested message and enum types in `Attribute`.
pub mod attribute {
    #[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// float_val exposes a floating point value.
        #[prost(double, tag = "1")]
        FloatVal(f64),
        /// int_numerator_val exposes a int value.
        #[prost(int64, tag = "2")]
        IntVal(i64),
        /// string_val exposes a string value.
        #[prost(string, tag = "3")]
        StringVal(::prost::alloc::string::String),
        /// bool_val exposes a boolean statistic.
        #[prost(bool, tag = "4")]
        BoolVal(bool),
    }
}
