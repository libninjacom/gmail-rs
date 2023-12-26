use std::{env, fs};
use std::fs::File;
use serde_yaml::Value;
use openapiv3::{OpenAPI};

// use as cargo run -- <input> <output>
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let fpath: String = args.next().expect("Need an input fpath to an openapi spec");
    let out_fpath: String = args.next().expect("Need an output fpath");

    let f = File::open(fpath)?;
    let mut spec: OpenAPI = serde_yaml::from_reader(f)?;
    spec.parameters.remove("_.xgafv").unwrap();
    for path in spec.paths.values_mut() {
        let mut path = path.as_mut().unwrap();
        path.parameters.clear();
        for (path, item) in path.iter_mut() {
            item.operation_id.as_mut().map(|id| {
                *id = id.replace("gmail.users.", "");
            });
        }
    }
    spec.security.push({
        let mut map = openapiv3::IndexMap::new();
        map.insert("Oauth2c".to_string(), vec![]);
        map
    });

    fs::write(&out_fpath, serde_yaml::to_string(&spec).unwrap()).expect("Could not write to file");
    eprintln!("Wrote to {}", out_fpath);
    Ok(())
}
