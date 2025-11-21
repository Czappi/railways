pub struct TypeDescriptor {
    name: &'static str,
    structure: TypeStructure,
}

impl TypeDescriptor {
    pub fn new(name: &'static str, structure: TypeStructure) -> Self {
        Self { name, structure }
    }
}

pub enum TypeStructure {
    Struct(Struct),
    Enum(Enum),
    Value,
}

pub struct Struct {
    fields: Vec<Field>,
}

pub struct Enum {
    variants: Vec<EnumVariant>,
}

pub struct EnumVariant {
    name: &'static str,
    fields: Vec<Field>,
}

pub struct Field {
    name: &'static str,
    r#type: TypeDescriptor,
}

pub trait TypeDescription {
    fn type_description() -> TypeDescriptor;
}

macro_rules! value_type_description {
    ($type:ty) => {
        place_macro::place! {
            impl TypeDescription for $type {
                fn type_description() -> TypeDescriptor {
                    TypeDescriptor::new(__stringify__($type), TypeStructure::Value)
                }
            }
        }
    };
}

value_type_description!(u8);
value_type_description!(u16);
value_type_description!(u32);
value_type_description!(u64);
value_type_description!(u128);
value_type_description!(i8);
value_type_description!(i16);
value_type_description!(i32);
value_type_description!(i64);
value_type_description!(i128);
value_type_description!(String);
value_type_description!(bool);
