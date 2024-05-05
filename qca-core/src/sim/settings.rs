use std::collections::HashMap;

use serde::{Serialize, Deserialize};

pub type OptionsList = Vec<OptionsEntry>;
pub type OptionsValueList = HashMap<String, OptionValue>;

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum OptionValue{
    Number{value: f32},
    String{value: String},
    Bool{value:bool}
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OptionsEntry{
    Header{label: String},
    Break,
    Input{unique_id: String, name: String, description: String, descriptor: InputDescriptor},
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InputDescriptor{
    NumberInput{min: Option<f32>, max: Option<f32>, unit: Option<String>, whole_num: bool},
    StringInput{},
    BoolInput{},
}