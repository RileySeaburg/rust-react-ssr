use actix_web::{get, web, HttpResponse, Responder};
use std::{env, fs::read_to_string};
use v8::*;

#[get("/")]
async fn index() -> HttpResponse {
    // initialize the platform
    let platform = new_default_platform(4, false);
    V8::initialize_platform(platform.into());
    V8::initialize();
    // initialize the isolate
    let isolate = &mut Isolate::new(Default::default());

    // Define the exports object
    let js_shim = r#"
        var exports = {};
    "#;

    let props = r##"{
        "params": [
            "hello",W
            "ciao",
            "こんにちは"
        ]
    }"##;

    let html = format!(
        r##"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Vite + React + TS</title>
    <!--app-head-->
  </head>
  <body>
    <div id="root"><!--app-html--></div>
    <script type="module" src="./vite/client/dist/client/entry-server.js"></script>
  </body>
</html>

"##
    );

    let current_dir = match env::current_dir() {
        Ok(current_dir) => current_dir.display().to_string(),
        Err(err) => format!("Failed to get the current working directory: {}", err),
    };

    let source = read_to_string("../client/dist/server/bundle.js").unwrap_or_else(|_| {
        panic!(
            "Error reading file {}, CWD: {}",
            "../client/dist/server/bundle.js", current_dir
        )
    });

    // Combine the shim and the server bundle
    let js_code = format!("{}{}", js_shim, source);

    // createa scope for the execution
    let scope = &mut HandleScope::new(isolate);

    // Create a new context for the execution
    // Create a new context without using a snapshot
    let context = Context::new(scope);

    // Enter the context for compiling and
    // running the hello world script

    let scope = &mut ContextScope::new(scope, context);

    // Compile the code
    let code = js_code;

    let code = v8::String::new(scope, &code).unwrap();

    let script = v8::Script::compile(scope, code, None).unwrap();

    // Run the script
    let result = script.run(scope).unwrap();

    // Convert the result to a string and print it
    let result = result.to_string(scope).unwrap();

    println!("{}", result.to_rust_string_lossy(scope));

    // Create a new object template

    HttpResponse::Ok().body(html)
}
