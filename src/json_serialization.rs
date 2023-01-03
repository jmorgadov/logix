use crate::components::composed_component::*;
use crate::components::prelude::*;
use crate::parser::{CompParser, ParseResult};
use crate::visitor::CompVisitor;
use serde_json::{json, Value};
use std::fs::{read_to_string, write};

/// Saves a `ComposedComponent` as a JSON file to a given location
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the file path where to save the component.
/// * `comp` - A reference to the component that will be stored.
///
/// # Examples
///
/// ```
/// // Assuming `comp` is a variable that holds a `ComposedComponent`
/// save("example_comp.json", &comp);
/// ```
pub fn save(file_path: &str, comp: &ComposedComponent) {
    let serialzier: JsonSerializer = Default::default();
    let value = serialzier.visit_composed(comp);
    write(file_path, serde_json::to_string(&value).unwrap()).expect("Unable to write file");
}

/// Loads a `ComposedComponent` from a JSON file
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the file path where to load the component.
///
/// # Examples
///
/// ```
/// let comp = load("example_comp.json").unwrap();
/// ```
pub fn load(file_path: &str) -> Result<ComposedComponent, ()> {
    let data = read_to_string(file_path).expect("Unable to read file");
    let deserialzier: JsonDeserializer = Default::default();
    let json_result = &serde_json::from_str::<Value>(&data);
    match json_result {
        Ok(json) => Ok(deserialzier.parse_composed(json)?),
        _ => Err(()),
    }
}

#[derive(Default)]
struct JsonSerializer;

impl CompVisitor<Value> for JsonSerializer {
    fn visit_not_gate(&self, comp: &NotGate) -> Value {
        serde_json::json!({
            "id": comp.id(),
            "name": Primitive::NotGate.to_string(),
            "in_count": comp.ins.len(),
        })
    }

    fn visit_and_gate(&self, comp: &AndGate) -> Value {
        serde_json::json!({
            "id": comp.id,
            "name": Primitive::AndGate.to_string(),
            "in_count": comp.ins.len(),
        })
    }

    fn visit_or_gate(&self, comp: &OrGate) -> Value {
        serde_json::json!({
            "id": comp.id,
            "name": Primitive::OrGate.to_string(),
            "in_count": comp.ins.len(),
        })
    }

    fn visit_nand_gate(&self, comp: &NandGate) -> Value {
        serde_json::json!({
            "id": comp.id,
            "name": Primitive::NandGate.to_string(),
            "in_count": comp.ins.len(),
        })
    }

    fn visit_nor_gate(&self, comp: &NorGate) -> Value {
        serde_json::json!({
            "id": comp.id,
            "name": Primitive::NorGate.to_string(),
            "in_count": comp.ins.len(),
        })
    }

    fn visit_xor_gate(&self, comp: &XorGate) -> Value {
        serde_json::json!({
            "id": comp.id,
            "name": Primitive::XorGate.to_string(),
            "in_count": comp.ins.len(),
        })
    }

    fn visit_clock(&self, comp: &Clock) -> Value {
        serde_json::json!({
            "id": comp.id,
            "name": Primitive::Clock.to_string(),
            "frec": comp.frec,
        })
    }

    fn visit_input_pin(&self, comp: &InputPin) -> Value {
        serde_json::json!({
            "id": comp.id,
            "name": Primitive::InputPin.to_string(),
        })
    }

    fn visit_output_pin(&self, comp: &OutputPin) -> Value {
        serde_json::json!({
            "id": comp.id,
            "name": Primitive::OutputPin.to_string(),
        })
    }

    fn visit_const(&self, comp: &Const) -> Value {
        let primitive = match comp.outs[0] {
            true => Primitive::ConstOne,
            false => Primitive::ConstZero,
        };
        serde_json::json!({
            "id": comp.id,
            "name": primitive.to_string(),
        })
    }

    fn visit_composed(&self, comp: &ComposedComponent) -> Value {
        let mut val: Value = Default::default();
        let comps: Vec<Value> = comp
            .components
            .iter()
            .map(|e| {
                if let Ok(prim) = Primitive::from_str(&e.name()) {
                    match prim {
                        Primitive::NotGate => self.visit_and_gate(e.as_and_gate().unwrap()),
                        Primitive::AndGate => self.visit_and_gate(e.as_and_gate().unwrap()),
                        Primitive::OrGate => self.visit_or_gate(e.as_or_gate().unwrap()),
                        Primitive::NandGate => self.visit_nand_gate(e.as_nand_gate().unwrap()),
                        Primitive::NorGate => self.visit_nor_gate(e.as_nor_gate().unwrap()),
                        Primitive::XorGate => self.visit_xor_gate(e.as_xor_gate().unwrap()),
                        Primitive::Clock => self.visit_clock(e.as_clock().unwrap()),
                        Primitive::InputPin => self.visit_input_pin(e.as_input_pin().unwrap()),
                        Primitive::OutputPin => self.visit_output_pin(e.as_output_pin().unwrap()),
                        Primitive::ConstOne => self.visit_const(e.as_const().unwrap()),
                        Primitive::ConstZero => self.visit_const(e.as_const().unwrap()),
                    }
                } else {
                    self.visit_composed(e.as_composed().unwrap())
                }
            })
            .collect();

        val["id"] = json!(comp.id);
        val["name"] = json!(comp.name);
        val["connections"] = json!(comp.connections);
        val["components"] = json!(comps);
        val
    }
}

#[derive(Default)]
struct JsonDeserializer;

impl CompParser<&Value> for JsonDeserializer {
    fn parse_not_gate(&self, obj: &Value) -> ParseResult<NotGate> {
        Ok(NotGate::new(obj["id"].as_u64().ok_or(())? as u32))
    }

    fn parse_and_gate(&self, obj: &Value) -> ParseResult<AndGate> {
        Ok(AndGate::new(
            obj["id"].as_u64().ok_or(())? as u32,
            obj["in_count"].as_u64().ok_or(())? as usize,
        ))
    }

    fn parse_or_gate(&self, obj: &Value) -> ParseResult<OrGate> {
        Ok(OrGate::new(
            obj["id"].as_u64().ok_or(())? as u32,
            obj["in_count"].as_u64().ok_or(())? as usize,
        ))
    }

    fn parse_nand_gate(&self, obj: &Value) -> ParseResult<NandGate> {
        Ok(NandGate::new(
            obj["id"].as_u64().ok_or(())? as u32,
            obj["in_count"].as_u64().ok_or(())? as usize,
        ))
    }

    fn parse_nor_gate(&self, obj: &Value) -> ParseResult<NorGate> {
        Ok(NorGate::new(
            obj["id"].as_u64().ok_or(())? as u32,
            obj["in_count"].as_u64().ok_or(())? as usize,
        ))
    }

    fn parse_xor_gate(&self, obj: &Value) -> ParseResult<XorGate> {
        Ok(XorGate::new(
            obj["id"].as_u64().ok_or(())? as u32,
            obj["in_count"].as_u64().ok_or(())? as usize,
        ))
    }

    fn parse_clock(&self, obj: &Value) -> ParseResult<Clock> {
        Ok(Clock::new(
            obj["id"].as_u64().ok_or(())? as u32,
            obj["frec"].as_f64().ok_or(())?,
        ))
    }

    fn parse_input_pin(&self, obj: &Value) -> ParseResult<InputPin> {
        Ok(InputPin::new(obj["id"].as_u64().ok_or(())? as u32))
    }

    fn parse_output_pin(&self, obj: &Value) -> ParseResult<OutputPin> {
        Ok(OutputPin::new(obj["id"].as_u64().ok_or(())? as u32))
    }

    fn parse_const(&self, obj: &Value) -> ParseResult<Const> {
        let id = obj["id"].as_u64().ok_or(())? as u32;
        let name = obj["name"].as_str().ok_or(())?;
        if name == Primitive::ConstOne.to_string() {
            Ok(Const::one(id))
        } else if name == Primitive::ConstZero.to_string() {
            Ok(Const::zero(id))
        } else {
            panic!("Unkown name")
        }
    }

    fn parse_composed(&self, obj: &Value) -> ParseResult<ComposedComponent> {
        let mut builder = ComposedComponentBuilder::new()
            .id(obj["id"].as_u64().ok_or(())? as u32)
            .name(obj["name"].as_str().ok_or(())?);

        for comp_json in obj["components"].as_array().ok_or(())?.iter() {
            let name = comp_json["name"].as_str().ok_or(())?;
            let sub_c: Box<dyn Component>;
            if let Ok(prim) = Primitive::from_str(name) {
                match prim {
                    Primitive::NotGate => {
                        sub_c = Box::new(self.parse_not_gate(comp_json)?);
                    }
                    Primitive::AndGate => {
                        sub_c = Box::new(self.parse_and_gate(comp_json)?);
                    }
                    Primitive::OrGate => {
                        sub_c = Box::new(self.parse_or_gate(comp_json)?);
                    }
                    Primitive::NandGate => {
                        sub_c = Box::new(self.parse_nand_gate(comp_json)?);
                    }
                    Primitive::NorGate => {
                        sub_c = Box::new(self.parse_nor_gate(comp_json)?);
                    }
                    Primitive::XorGate => {
                        sub_c = Box::new(self.parse_xor_gate(comp_json)?);
                    }
                    Primitive::Clock => {
                        sub_c = Box::new(self.parse_clock(comp_json)?);
                    }
                    Primitive::InputPin => {
                        sub_c = Box::new(self.parse_input_pin(comp_json)?);
                    }
                    Primitive::OutputPin => {
                        sub_c = Box::new(self.parse_output_pin(comp_json)?);
                    }
                    Primitive::ConstOne => {
                        sub_c = Box::new(self.parse_const(comp_json)?);
                    }
                    Primitive::ConstZero => {
                        sub_c = Box::new(self.parse_const(comp_json)?);
                    }
                }
            } else {
                sub_c = Box::new(self.parse_composed(comp_json)?);
            }
            builder = builder.add_comp(sub_c);
        }

        for conn_json in obj["connections"].as_array().ok_or(())?.iter() {
            let from = conn_json["from"].as_object().ok_or(())?;
            let from_pin = pin!(
                from["id"].as_u64().ok_or(())? as u32,
                from["addr"].as_u64().ok_or(())? as usize
            );
            let to = conn_json["to"].as_object().ok_or(())?;
            let to_pin = pin!(
                to["id"].as_u64().ok_or(())? as u32,
                to["addr"].as_u64().ok_or(())? as usize
            );
            builder = builder.connect(from_pin, to_pin);
        }
        Ok(builder.build())
    }
}
