# appres

`appres` is a library for interacting with application resources.

## Examples

Basic usage:

```rust
use appres::Resources;

let resources = Resources::new_app_relative_to_config("projectile").unwrap();

// Check if the projectile directory has a file named list.yaml.
assert!(!resources.has_file("list.yaml"));

// Check if the projectile directory has a directory named hello_world.
assert!(!resources.has_dir("hello_world"));
```

## Features

This library also contains extended functionality for json, toml, and yaml files that
are feature-gated.

- **json_resources**: Enabling this feature gives you extra methods (through the
  `JsonResourcesExt` trait) and extra functions for working with json files.
- **toml_resources**: Enabling this feature gives you extra methods (through the
  `TomlResourcesExt` trait) and extra functions for working with toml files.
- **yaml_resources**: Enabling this feature gives you extra methods (through the
  `YamlResourcesExt` trait) and extra functions for working with yaml files.
  
### Examples

For example, if you enable the `yaml_resources` feature in Cargo.toml...

```rust
use appres::Resources;
use appres::yaml::YamlResourcesExt;

// Create a resources manager for the projectile directory in the config directory.
let resources = Resources::new_app_relative_to_config("projectile").unwrap();

// Create a list of strings.
let list = vec!["a".to_string(), "b".to_string(), "c".to_string()];

// Write the serialized list to list.yaml in the projectile directory of the config
// directory.
resources.save_to_yaml_file("list.yaml", &list).unwrap();

// Load the list from the file again.
let list_copy: Vec<String> = resources.load_from_yaml_file("list.yaml").unwrap();
assert_eq!(list, list_copy);

// Check for the presence of the list.yaml file
assert!(resources.has_file("list.yaml"));
```

