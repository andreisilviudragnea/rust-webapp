use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

trait Database<'a> {
    #[allow(dead_code)]
    fn get_code(&mut self) -> Buffer<'a>;
}

#[derive(PartialEq, Debug)]
struct Db<'a>(&'a str);

impl<'a> Database<'a> for Db<'a> {
    fn get_code(&mut self) -> Buffer<'a> {
        Buffer { str: self.0 }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(bound = "B: Database<'a>")]
struct Machine<'a, B: Database<'a>> {
    parent: Option<Box<Self>>,
    #[serde(borrow)]
    buffer: Buffer<'a>,
    #[serde(skip)]
    phantom: PhantomData<*const B>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Buffer<'a> {
    str: &'a str,
}

impl<'a, B: Database<'a>> Machine<'a, B> {
    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[test]
fn test() {
    let str = "abc";
    let machine: Machine<Db> = Machine {
        parent: None,
        buffer: Buffer { str },
        phantom: PhantomData,
    };

    let serialized = "{\"parent\":null,\"buffer\":{\"str\":\"abc\"}}".to_string();

    assert_eq!(machine.serialize(), serialized);

    let deserialized_machine = serde_json::from_str(&serialized).unwrap();
    assert_eq!(machine, deserialized_machine);
}
