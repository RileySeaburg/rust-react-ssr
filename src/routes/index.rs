use actix_web::{get, web, HttpResponse, Responder};
use deno_core::anyhow::{bail, Error};
use deno_core::{
    resolve_import, resolve_path, FastString, JsRuntime, ModuleLoader, ModuleSource,
    ModuleSourceFuture, ModuleSpecifier, ModuleType, ResolutionKind, RuntimeOptions,
};
use futures::FutureExt;
use std::cell::RefCell;
use std::env;
use std::path::{self, Path};
use std::pin::Pin;
use std::rc::Rc;

pub struct SimpleModuleLoader;

impl ModuleLoader for SimpleModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _is_main: ResolutionKind,
    ) -> Result<ModuleSpecifier, Error> {
        Ok(resolve_import(specifier, referrer)?)
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<&ModuleSpecifier>,
        _is_dyn_import: bool,
    ) -> Pin<Box<ModuleSourceFuture>> {
        let module_specifier = module_specifier.clone();
        let string_specifier = module_specifier.to_string();

        async move {
            let mut module_url_found = string_specifier.clone();
            let bytes = match module_specifier.scheme() {
                "http" | "https" => {
                    let res = reqwest::get(module_specifier).await?;
                    let res = res.error_for_status()?;
                    module_url_found = res.url().to_string();
                    res.bytes().await?.to_vec()
                }
                "file" => {
                    let path = match module_specifier.to_file_path() {
                        Ok(path) => path,
                        Err(_) => bail!("Invalid file URL."),
                    };
                    tokio::fs::read(path).await?
                }
                "data" => {
                    let url = match reqwest::Url::parse(&string_specifier) {
                        Ok(url) => url,
                        Err(_) => bail!("Not a valid data URL."),
                    };
                    let data: Vec<u8> = url.path().as_bytes().to_vec();
                    data
                }
                              schema => bail!("Invalid schema {}", schema),
            };

            let code = String::from_utf8(bytes).unwrap();
            let module_type = ModuleType::JavaScript;
            let module_url_specified = resolve_path(&string_specifier, &env::current_dir()?)?;
            let module_url_found = resolve_path(&module_url_found, &env::current_dir()?)?;

            Ok(ModuleSource::new_with_redirect(
                module_type,
                code.into(),
                &module_url_specified,
                &module_url_found,
            ))
        }
        .boxed_local()
    }
}

#[get("/")]
async fn index() -> HttpResponse {
    let props = r##"{
        "params": [
            "hello",
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
    <script type="module" src="./vite/client/dist/client/entry-server.mjs"></script>
  </body>
</html>
"##
    );

    let options = RuntimeOptions {
        module_loader: Some(Rc::new(SimpleModuleLoader)),
        ..Default::default()
    };
    let mut runtime = JsRuntime::new(options);

    let current_dir = std::env::current_dir().expect("Failed to get current directory");
   let specifier = deno_core::ModuleSpecifier::resolve(url.as_str())
    .expect("Failed to create module specifier");



    runtime
        .load_main_module(&specifier, None)
        .await
        .expect("Failed to load module");

    runtime
        .run_event_loop(true)
        .await
        .expect("Failed to run event loop");

    HttpResponse::Ok().body(html)
}
