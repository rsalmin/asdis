use crate::primitives::*;
use std::collections::HashMap;

pub type ShowFun<T:Num> = fn(T::DType) -> String;
pub type ShowDict<T> = HashMap<String, ShowFun<T>>;

///RV32C instructions subset
#[derive(Debug)]
pub struct ISA<T:Num> {
    pub list : Vec<Instruction::<T>>,
    pub show_dict : ShowDict::<T>,
}
