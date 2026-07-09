//! Data type definitions.

/// Runtime data type tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
    USize,
}

impl DType {
    pub fn size_bytes(&self) -> usize {
        match self {
            DType::U8 => 1,
            DType::U16 => 2,
            DType::U32 => 4,
            DType::U64 => 8,
            DType::I8 => 1,
            DType::I16 => 2,
            DType::I32 => 4,
            DType::I64 => 8,
            DType::F32 => 4,
            DType::F64 => 8,
            DType::Bool => 1,
            DType::USize => std::mem::size_of::<usize>(),
        }
    }
}
