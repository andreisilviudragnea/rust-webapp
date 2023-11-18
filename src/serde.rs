use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Machine<'a> {
    #[serde(borrow)]
    buffer: Buffer<'a>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Buffer<'a> {
    str: &'a str,
}

#[test]
fn test() {
    let str = "abc";
    let machine = Machine {
        buffer: Buffer { str },
    };

    let serialized = "{\"buffer\":{\"str\":\"abc\"}}".to_string();

    assert_eq!(serde_json::to_string(&machine).unwrap(), serialized);

    let deserialized_machine = serde_json::from_str(&serialized).unwrap();
    assert_eq!(machine, deserialized_machine);
}
