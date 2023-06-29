use std::collections::HashMap;

use actix_web::{
    dev::Server, get, http::StatusCode, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Responder, body,
};
use tera::{Context, Tera};

use crate::AppState;



#[get("/")]
pub async fn index(req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("foo", "123");

    let source = data.js_source.lock().unwrap();

    let isolate = &mut v8::Isolate::new(Default::default());

    let handle_scope = &mut v8::HandleScope::new(isolate);

    let context = v8::Context::new(handle_scope);

    let scope = &mut v8::ContextScope::new(handle_scope, context);

    let code = v8::String::new(scope, &format!({};{}, source, "SSR"))
        .expect("Invalid JS: Strings are needed");

    let script =
        v8::Script::compile(scope, code, None).expect("Invalid JS: There aren't runnable scripts");

    let exports = script
        .run(scope)
        .expect("Invalid JS: Missing entry point. Is the bundle exported as a variable?");

    let object = exports
        .to_object(scope)
        .expect("Invalid JS: There are no objects");
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html: charset=utf-8")
        .streaming(body)
}
