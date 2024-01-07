use std::{env, fs};
use std::fs::File;
use serde_yaml::Value;
use openapiv3::{OpenAPI, Schema, RefOr};

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

    let make_required = [
        "Thread",
        "Message",
        "MessagePart",
    ];
    for name in make_required {
        let mut schema = spec.schemas.index_mut2(name);
        let keys = schema.properties().keys().cloned().collect::<Vec<_>>();
        *schema.required_mut() = keys;
    }
    let message = spec.schemas.index_mut2("Message");
    message.remove_required("raw");
    let thread = spec.schemas.index_mut2("Thread");
    thread.remove_required("snippet");

    let props = spec.schemas.index_mut2("MessagePart").properties_mut();
    props.insert("headers", Schema::new_array(RefOr::schema_ref("Header")));
    spec.schemas.insert("Header", {
        let mut s = Schema::new_object();
        let p = s.properties_mut();
        p.insert("name".to_string(), Schema::new_string());
        p.insert("value".to_string(), Schema::new_string());
        let req = s.required_mut();
        req.push("name".to_string());
        req.push("value".to_string());
        s
    });
    *spec.schemas.index_mut2("ListThreadsResponse").required_mut() = vec![
        "resultSizeEstimate".to_string(),
        "threads".to_string(),
    ];
    fs::write(&out_fpath, serde_yaml::to_string(&spec).unwrap()).expect("Could not write to file");
    eprintln!("Wrote to {}", out_fpath);
    Ok(())
}
