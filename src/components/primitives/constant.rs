use crate::{components::component::Component, serialize::JSONSerialize};

use super::primitive::Primitive;

#[derive(Debug)]
pub struct Const {
    pub id: u32,
    pub ins: Vec<bool>,
    pub outs: Vec<bool>,
}

impl Const {
    fn new(id: u32, value: bool) -> Self {
        Const {
            id,
            ins: vec![],
            outs: vec![value],
        }
    }

    pub fn one(id: u32) -> Self {
        Const::new(id, true)
    }

    pub fn zero(id: u32) -> Self {
        Const::new(id, false)
    }
}

impl JSONSerialize for Const {
    fn to_json(&self) -> serde_json::Value {
        let primitive = match self.outs[0] {
            true => Primitive::ConstOne,
            false => Primitive::ConstZero,
        };
        serde_json::json!({
            "id": self.id,
            "name": primitive.to_string(),
        })
    }

    fn from_json(json: &serde_json::Value) -> Self
    where
        Self: Sized,
    {
        let id = json["id"].as_u64().unwrap() as u32;
        let name = json["name"].as_str().unwrap();
        if name == Primitive::ConstOne.to_string() {
            Const::one(id)
        } else if name == Primitive::ConstZero.to_string() {
            Const::zero(id)
        } else {
            panic!("Unkown name")
        }
    }
}

impl Component for Const {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> String {
        match self.outs[0] {
            true => Primitive::ConstOne.to_string(),
            false => Primitive::ConstZero.to_string(),
        }
    }

    fn ins(&mut self) -> &mut Vec<bool> {
        &mut self.ins
    }

    fn outs(&mut self) -> &mut Vec<bool> {
        &mut self.outs
    }
}

#[cfg(test)]
mod tests {
    use super::Const;
    use crate::components::component::{Component, SimEvent};

    #[test]
    fn cont_one() {
        let comp = &mut Const::one(0);
        comp.on_event(&SimEvent::UpdateValues);
        assert!(comp.outs[0]);
    }

    #[test]
    fn cont_zero() {
        let comp = &mut Const::zero(0);
        comp.on_event(&SimEvent::UpdateValues);
        assert!(!comp.outs[0]);
    }
}
