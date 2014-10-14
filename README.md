# JsonWay

[![Build Status](https://travis-ci.org/rustless/jsonway.svg?branch=master)](https://travis-ci.org/rustless/jsonway)

JsonWay gives you a simple DSL for declaring JSON structures. This is particularly helpful when the generation process is fraught with conditionals and loops. It is inspired by [jbuilder](https://github.com/rails/jbuilder) and has similar functional.

```toml
# Cargo.toml
[dependencies.jsonway]
git = "https://github.com/rustless/jsonway"
```

[API docs](http://rustless.org/jsonway/doc/jsonway/)

## Simple example

``` rust
JsonWay::object(|json| {
    json.set("first_name", "Luke".to_string()); 
    json.set("last_name", "Skywalker".to_string());

    json.object("info", |json| {
        json.set("homeworld", "Tatooine".to_string());
        json.set("born", "19 BBY".to_string());
        json.set("died", "Between 45 ABY and 137 ABY".to_string());
    });

    json.list("masters", |json| {
        json.push("Obi-Wan Kenobi".to_string());
        json.push("Yoda".to_string());
        json.push("Joruus C'baoth (Briefly)".to_string());
        json.push("Darth Sidious (Briefly)".to_string());
    });
});

// {
//   "first_name": "Luke",
//   "last_name": "Skywalker",
//   "info": {
//     "born": "19 BBY",
//     "died": "Between 45 ABY and 137 ABY",
//     "homeworld": "Tatooine"
//   },
//   "masters": [
//     "Obi-Wan Kenobi",
//     "Yoda",
//     "Joruus C'baoth (Briefly)",
//     "Darth Sidious (Briefly)"
//   ]
// }
```

## Build with iterators

~~~rust

#[deriving(Show)]
enum Side {
    Light,
    Dark
}

struct Jedi {
    name: String,
    side: Side
}

let jedi = vec![
    Jedi { name: "Saes Rrogon".to_string(), side: Dark },
    Jedi { name: "Qui-Gon Jinn".to_string(), side: Light },
    Jedi { name: "Obi-Wan Kenobi".to_string(), side: Light }
];

let light_jedi_objects_list = JsonWay::list(|json| {
    // Use `objects` method to make list of objects
    json.objects(&mut jedi.iter(), |jedi, json| {
        match jedi.side {
            Light => {
                json.set("name".to_string(), jedi.name.to_string());
                json.set("side".to_string(), jedi.side.to_string());
            },
            Dark => json.skip()
        }
    })
});

// [
//   {
//     "name": "Qui-Gon Jinn",
//     "side": "Light"
//   },
//   {
//     "name": "Obi-Wan Kenobi",
//     "side": "Light"
//   }
// ]

let light_jedi_tuple_list = JsonWay::list(|json| {
    // Use `lists` method to make list of lists
    json.lists(&mut jedi.iter(), |jedi, json| {
        match jedi.side {
            Light => {
                json.push(jedi.name.to_string());
                json.push(jedi.side.to_string());
            },
            Dark => json.skip()
        }
    })
});

// [
//   [
//     "Qui-Gon Jinn",
//     "Light"
//   ],
//   [
//     "Obi-Wan Kenobi",
//     "Light"
//   ]
// ]

~~~

You can explicitly make `JsonWay` object return `null` if you want:

~~~rust
// .. 
match jedi.side {
    Light => {
        json.push(jedi.name.to_string());
        json.push(jedi.side.to_string());
    },
    Dark => json.null()
}
~~~

