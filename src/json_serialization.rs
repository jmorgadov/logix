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
            "name": Primitive::NotGate.to_string(),
            "in_count": comp.ins.len(),
        })
    }

    fn visit_and_gate(&self, comp: &AndGate) -> Value {
        serde_json::json!({
            "name": Primitive::AndGate.to_string(),
            "in_count": comp.ins.len(),
        })
    }

    fn visit_or_gate(&self, comp: &OrGate) -> Value {
        serde_json::json!({
            "name": Primitive::OrGate.to_string(),
            "in_count": comp.ins.len(),
        })
    }

    fn visit_nand_gate(&self, comp: &NandGate) -> Value {
        serde_json::json!({
            "name": Primitive::NandGate.to_string(),
            "in_count": comp.ins.len(),
        })
    }

    fn visit_nor_gate(&self, comp: &NorGate) -> Value {
        serde_json::json!({
            "name": Primitive::NorGate.to_string(),
            "in_count": comp.ins.len(),
        })
    }

    fn visit_xor_gate(&self, comp: &XorGate) -> Value {
        serde_json::json!({
            "name": Primitive::XorGate.to_string(),
            "in_count": comp.ins.len(),
        })
    }

    fn visit_clock(&self, comp: &Clock) -> Value {
        serde_json::json!({
            "name": Primitive::Clock.to_string(),
            "frec": comp.frec,
        })
    }

    fn visit_input_pin(&self, comp: &InputPin) -> Value {
        serde_json::json!({
            "num": comp.num,
            "name": Primitive::InputPin.to_string(),
        })
    }

    fn visit_output_pin(&self, comp: &OutputPin) -> Value {
        serde_json::json!({
            "num": comp.num,
            "name": Primitive::OutputPin.to_string(),
        })
    }

    fn visit_const(&self, comp: &Const) -> Value {
        let primitive = match comp.outs[0] {
            true => Primitive::ConstOne,
            false => Primitive::ConstZero,
        };
        serde_json::json!({
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

        val["name"] = json!(comp.name);
        val["connections"] = json!(comp.connections);
        val["in_addrs"] = json!(comp.in_addrs);
        val["out_addrs"] = json!(comp.out_addrs);
        val["components"] = json!(comps);
        val
    }
}

#[derive(Default)]
struct JsonDeserializer;

impl CompParser<&Value> for JsonDeserializer {
    fn parse_not_gate(&self, _: &Value) -> ParseResult<NotGate> {
        Ok(NotGate::new())
    }

    fn parse_and_gate(&self, obj: &Value) -> ParseResult<AndGate> {
        Ok(AndGate::new(obj["in_count"].as_u64().ok_or(())? as usize))
    }

    fn parse_or_gate(&self, obj: &Value) -> ParseResult<OrGate> {
        Ok(OrGate::new(obj["in_count"].as_u64().ok_or(())? as usize))
    }

    fn parse_nand_gate(&self, obj: &Value) -> ParseResult<NandGate> {
        Ok(NandGate::new(obj["in_count"].as_u64().ok_or(())? as usize))
    }

    fn parse_nor_gate(&self, obj: &Value) -> ParseResult<NorGate> {
        Ok(NorGate::new(obj["in_count"].as_u64().ok_or(())? as usize))
    }

    fn parse_xor_gate(&self, obj: &Value) -> ParseResult<XorGate> {
        Ok(XorGate::new(obj["in_count"].as_u64().ok_or(())? as usize))
    }

    fn parse_clock(&self, obj: &Value) -> ParseResult<Clock> {
        Ok(Clock::new(obj["frec"].as_f64().ok_or(())?))
    }

    fn parse_input_pin(&self, obj: &Value) -> ParseResult<InputPin> {
        Ok(InputPin::new(obj["num"].as_u64().ok_or(())? as usize))
    }

    fn parse_output_pin(&self, obj: &Value) -> ParseResult<OutputPin> {
        Ok(OutputPin::new(obj["num"].as_u64().ok_or(())? as usize))
    }

    fn parse_const(&self, obj: &Value) -> ParseResult<Const> {
        let name = obj["name"].as_str().ok_or(())?;
        if name == Primitive::ConstOne.to_string() {
            Ok(Const::one())
        } else if name == Primitive::ConstZero.to_string() {
            Ok(Const::zero())
        } else {
            Err(())
        }
    }

    fn parse_composed(&self, obj: &Value) -> ParseResult<ComposedComponent> {
        let mut builder = ComposedComponentBuilder::new(obj["name"].as_str().ok_or(())?);

        let mut components = vec![];
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
            components.push(sub_c);
        }
        builder = builder.components(components);

        let mut connections = vec![];
        for conn_json in obj["connections"].as_array().ok_or(())?.iter() {
            let from = conn_json["from"].as_array().ok_or(())?;
            let from_pin = (
                from[0].as_u64().ok_or(())? as usize,
                from[1].as_u64().ok_or(())? as usize,
            );
            let to = conn_json["to"].as_array().ok_or(())?;
            let to_pin = (
                to[0].as_u64().ok_or(())? as usize,
                to[1].as_u64().ok_or(())? as usize,
            );
            connections.push(conn!(from_pin, to_pin));
        }
        builder = builder.connections(connections);

        let mut in_addrs: Vec<PinAddr> = vec![];
        for input_pin in obj["in_addrs"].as_array().ok_or(())? {
            in_addrs.push((
                input_pin[0].as_u64().ok_or(())? as usize,
                input_pin[1].as_u64().ok_or(())? as usize,
            ));
        }
        builder = builder.inputs(in_addrs);

        let mut out_addrs: Vec<PinAddr> = vec![];
        for output_pin in obj["out_addrs"].as_array().ok_or(())? {
            out_addrs.push((
                output_pin[0].as_u64().ok_or(())? as usize,
                output_pin[1].as_u64().ok_or(())? as usize,
            ));
        }
        builder = builder.outputs(out_addrs);

        builder.build()
    }
}
