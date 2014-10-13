# JsonWay

[![Build Status](https://travis-ci.org/rustless/jsonway.svg?branch=master)](https://travis-ci.org/rustless/jsonway)

JsonWay gives you a simple DSL for declaring JSON structures. This is particularly helpful when the generation process is fraught with conditionals and loops.

```toml
# Cargo.toml
[dependencies.jsonway]
git = "https://github.com/rustless/jsonway"
```

[API docs](http://rustless.org/jsonway/doc/jsonway/)

## Examples

``` rust
ObjectBuilder::build(|json| {
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
```