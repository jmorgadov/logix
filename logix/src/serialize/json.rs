#![cfg(feature = "serialize")]

use crate::prelude::*;

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
/// ```no_run
/// # use logix::prelude::*;
/// # use logix::serialize::json::*;
/// #
/// let comp = ComposedComponentBuilder::new("MyComp").build().unwrap();
/// save("example_comp.json", &comp);
/// ```
pub fn save(file_path: &str, comp: &ComposedComponent) {
    let value = JsonSerializer::visit_composed(comp);
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
/// ```no_run
/// # use logix::serialize::json::load;
/// #
/// let comp = load("example_comp.json").unwrap();
/// ```
pub fn load(file_path: &str) -> Result<ComposedComponent, ParseError> {
    let data = read_to_string(file_path).expect("Unable to read file");
    let json_result = &serde_json::from_str::<Value>(&data);
    match json_result {
        Ok(json) => Ok(JsonDeserializer::parse_composed(json)?),
        _ => Err(Default::default()),
    }
}

#[derive(Default)]
struct JsonSerializer;

impl CompVisitor<Value> for JsonSerializer {
    fn visit_not_gate(comp: &NotGate) -> Value {
        serde_json::json!({
            "name": Primitive::NotGate.to_string(),
            "in_count": comp.ins.len(),
        })
    }

    fn visit_and_gate(comp: &AndGate) -> Value {
        serde_json::json!({
            "name": Primitive::AndGate.to_string(),
            "in_count": comp.ins.len(),
        })
    }

    fn visit_or_gate(comp: &OrGate) -> Value {
        serde_json::json!({
            "name": Primitive::OrGate.to_string(),
            "in_count": comp.ins.len(),
        })
    }

    fn visit_nand_gate(comp: &NandGate) -> Value {
        serde_json::json!({
            "name": Primitive::NandGate.to_string(),
            "in_count": comp.ins.len(),
        })
    }

    fn visit_nor_gate(comp: &NorGate) -> Value {
        serde_json::json!({
            "name": Primitive::NorGate.to_string(),
            "in_count": comp.ins.len(),
        })
    }

    fn visit_xor_gate(comp: &XorGate) -> Value {
        serde_json::json!({
            "name": Primitive::XorGate.to_string(),
            "in_count": comp.ins.len(),
        })
    }

    fn visit_clock(comp: &Clock) -> Value {
        serde_json::json!({
            "name": Primitive::Clock.to_string(),
            "frec": comp.frec,
        })
    }

    fn visit_const(comp: &Const) -> Value {
        let primitive = match comp.outs[0] {
            true => Primitive::ConstOne,
            false => Primitive::ConstZero,
        };
        serde_json::json!({
            "name": primitive.to_string(),
        })
    }

    fn visit_composed(comp: &ComposedComponent) -> Value {
        let mut val: Value = Default::default();
        let comps: Vec<Value> = comp
            .components
            .iter()
            .map(|e| {
                if let Ok(prim) = Primitive::from_name(&e.name()) {
                    match prim {
                        Primitive::NotGate => {
                            JsonSerializer::visit_not_gate(e.as_not_gate().unwrap())
                        }
                        Primitive::AndGate => {
                            JsonSerializer::visit_and_gate(e.as_and_gate().unwrap())
                        }
                        Primitive::OrGate => JsonSerializer::visit_or_gate(e.as_or_gate().unwrap()),
                        Primitive::NandGate => {
                            JsonSerializer::visit_nand_gate(e.as_nand_gate().unwrap())
                        }
                        Primitive::NorGate => {
                            JsonSerializer::visit_nor_gate(e.as_nor_gate().unwrap())
                        }
                        Primitive::XorGate => {
                            JsonSerializer::visit_xor_gate(e.as_xor_gate().unwrap())
                        }
                        Primitive::Clock => JsonSerializer::visit_clock(e.as_clock().unwrap()),
                        Primitive::ConstOne => JsonSerializer::visit_const(e.as_const().unwrap()),
                        Primitive::ConstZero => JsonSerializer::visit_const(e.as_const().unwrap()),
                    }
                } else {
                    JsonSerializer::visit_composed(e.as_composed().unwrap())
                }
            })
            .collect();

        val["name"] = json!(comp.name);

        let connections: Vec<Value> = comp
            .connections
            .iter()
            .map(|conn| json!({"from": conn.from, "to": conn.to}))
            .collect();
        val["connections"] = json!(connections);
        val["in_addrs"] = json!(comp.in_addrs);
        val["out_addrs"] = json!(comp.out_addrs);
        val["components"] = json!(comps);
        val
    }
}

#[derive(Default)]
struct JsonDeserializer;

impl CompParser<&Value> for JsonDeserializer {
    fn parse_not_gate(_: &Value) -> ParseResult<NotGate> {
        Ok(NotGate::new())
    }

    fn parse_and_gate(obj: &Value) -> ParseResult<AndGate> {
        Ok(AndGate::new(
            obj["in_count"].as_u64().ok_or_else(Default::default)? as usize,
        ))
    }

    fn parse_or_gate(obj: &Value) -> ParseResult<OrGate> {
        Ok(OrGate::new(
            obj["in_count"].as_u64().ok_or_else(Default::default)? as usize,
        ))
    }

    fn parse_nand_gate(obj: &Value) -> ParseResult<NandGate> {
        Ok(NandGate::new(
            obj["in_count"].as_u64().ok_or_else(Default::default)? as usize,
        ))
    }

    fn parse_nor_gate(obj: &Value) -> ParseResult<NorGate> {
        Ok(NorGate::new(
            obj["in_count"].as_u64().ok_or_else(Default::default)? as usize,
        ))
    }

    fn parse_xor_gate(obj: &Value) -> ParseResult<XorGate> {
        Ok(XorGate::new(
            obj["in_count"].as_u64().ok_or_else(Default::default)? as usize,
        ))
    }

    fn parse_clock(obj: &Value) -> ParseResult<Clock> {
        Ok(Clock::new(
            obj["frec"].as_f64().ok_or_else(Default::default)?,
        ))
    }

    fn parse_const(obj: &Value) -> ParseResult<Const> {
        let name = obj["name"].as_str().ok_or_else(Default::default)?;
        if name == Primitive::ConstOne.to_string() {
            Ok(Const::one())
        } else if name == Primitive::ConstZero.to_string() {
            Ok(Const::zero())
        } else {
            Err(Default::default())
        }
    }

    fn parse_composed(obj: &Value) -> ParseResult<ComposedComponent> {
        let mut builder =
            ComposedComponentBuilder::new(obj["name"].as_str().ok_or_else(Default::default)?);

        let mut components = vec![];
        for comp_json in obj["components"]
            .as_array()
            .ok_or_else(Default::default)?
            .iter()
        {
            let name = comp_json["name"].as_str().ok_or_else(Default::default)?;
            let sub_c: Box<dyn Component>;
            if let Ok(prim) = Primitive::from_name(name) {
                match prim {
                    Primitive::NotGate => {
                        sub_c = Box::new(JsonDeserializer::parse_not_gate(comp_json)?);
                    }
                    Primitive::AndGate => {
                        sub_c = Box::new(JsonDeserializer::parse_and_gate(comp_json)?);
                    }
                    Primitive::OrGate => {
                        sub_c = Box::new(JsonDeserializer::parse_or_gate(comp_json)?);
                    }
                    Primitive::NandGate => {
                        sub_c = Box::new(JsonDeserializer::parse_nand_gate(comp_json)?);
                    }
                    Primitive::NorGate => {
                        sub_c = Box::new(JsonDeserializer::parse_nor_gate(comp_json)?);
                    }
                    Primitive::XorGate => {
                        sub_c = Box::new(JsonDeserializer::parse_xor_gate(comp_json)?);
                    }
                    Primitive::Clock => {
                        sub_c = Box::new(JsonDeserializer::parse_clock(comp_json)?);
                    }
                    Primitive::ConstOne => {
                        sub_c = Box::new(JsonDeserializer::parse_const(comp_json)?);
                    }
                    Primitive::ConstZero => {
                        sub_c = Box::new(JsonDeserializer::parse_const(comp_json)?);
                    }
                }
            } else {
                sub_c = Box::new(JsonDeserializer::parse_composed(comp_json)?);
            }
            components.push(sub_c);
        }
        builder = builder.components(components);

        let mut connections = vec![];
        for conn_json in obj["connections"]
            .as_array()
            .ok_or_else(Default::default)?
            .iter()
        {
            let from = conn_json["from"].as_array().ok_or_else(Default::default)?;
            let from_pin = (
                from[0].as_u64().ok_or_else(Default::default)? as usize,
                from[1].as_u64().ok_or_else(Default::default)? as usize,
            );
            let to = conn_json["to"].as_array().ok_or_else(Default::default)?;
            let to_pin = (
                to[0].as_u64().ok_or_else(Default::default)? as usize,
                to[1].as_u64().ok_or_else(Default::default)? as usize,
            );
            connections.push(conn!(from_pin, to_pin));
        }
        builder = builder.connections(connections);

        let mut in_addrs: Vec<PinAddr> = vec![];
        for input_pin in obj["in_addrs"].as_array().ok_or_else(Default::default)? {
            in_addrs.push((
                input_pin[0].as_u64().ok_or_else(Default::default)? as usize,
                input_pin[1].as_u64().ok_or_else(Default::default)? as usize,
            ));
        }
        builder = builder.inputs(in_addrs);

        let mut out_addrs: Vec<PinAddr> = vec![];
        for output_pin in obj["out_addrs"].as_array().ok_or_else(Default::default)? {
            out_addrs.push((
                output_pin[0].as_u64().ok_or_else(Default::default)? as usize,
                output_pin[1].as_u64().ok_or_else(Default::default)? as usize,
            ));
        }
        builder = builder.outputs(out_addrs);

        match builder.build() {
            Ok(comp) => Ok(comp),
            Err(_) => Err(Default::default()),
        }
    }
}