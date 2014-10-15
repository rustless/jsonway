
extern crate serialize;
extern crate jsonway;

use jsonway::{JsonWay};

#[deriving(Show)]
enum Side {
    Light,
    Dark
}

struct Jedi {
    name: String,
    side: Side
}

fn jedi_list() -> Vec<Jedi> {
    vec![
        Jedi { name: "Saes Rrogon".to_string(), side: Dark },
        Jedi { name: "Qui-Gon Jinn".to_string(), side: Light },
        Jedi { name: "Obi-Wan Kenobi".to_string(), side: Light }
    ]
}

#[test]
fn simple_list_of_objects() {
    let jedi_list = jedi_list();

    let json = JsonWay::list(|json| {
        json.objects(&mut jedi_list.iter(), |jedi, json| {
            match jedi.side {
                Light => {
                    json.set("name".to_string(), jedi.name.to_string());
                    json.set("side".to_string(), jedi.side.to_string());
                },
                Dark => json.skip()
            }
        })
    }).unwrap();

    let list = json.as_list().unwrap();

    assert_eq!(list.len(), 2);
}

#[test]
fn simple_list_of_lists() {
    let jedi_list = jedi_list();

    let json = JsonWay::list(|json| {
        json.objects(&mut jedi_list.iter(), |jedi, json| {
            match jedi.side {
                Light => {
                    json.set("name".to_string(), jedi.name.to_string());
                    json.set("side".to_string(), jedi.side.to_string());
                },
                Dark => json.skip()
            }
        })
    }).unwrap();

    let list = json.as_list().unwrap();
    
    assert_eq!(list.len(), 2);
}