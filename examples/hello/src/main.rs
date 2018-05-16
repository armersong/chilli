extern crate sharp_pencil;
extern crate typemap;
#[macro_use] extern crate log;
extern crate env_logger;

use std::env;
use std::collections::BTreeMap;
use typemap::Key;
use sharp_pencil::{Pencil, Request, Response, PencilResult};
use sharp_pencil::jsonify;
use sharp_pencil::HTTPError;
use sharp_pencil::{redirect, abort};
use sharp_pencil::method::Get;
use sharp_pencil::Module;


fn hello(_: &mut Request) -> PencilResult {
    Ok(Response::from("Hello World!"))
}

fn user(r: &mut Request) -> PencilResult {
    let user_id = r.view_args.get("user_id").unwrap();
    Ok(format!("user {}", user_id).into())
}

fn app_info(_: &mut Request) -> PencilResult {
    let mut d = BTreeMap::new();
    d.insert("name", "hello");
    d.insert("version", "0.1.0");
    return jsonify(&d);
}

fn page_not_found(_: HTTPError) -> PencilResult {
    let mut response = Response::from("Customized 404 :)");
    response.status_code = 404;
    Ok(response)
}

fn hello_template(request: &mut Request) -> PencilResult {
    let mut context = BTreeMap::new();
    context.insert("name".to_string(), "template".to_string());
    return request.app.render_template("hello.html", &context);
}

struct KeyType;
struct Value(i32);
impl Key for KeyType { type Value = Value; }

fn before_request(request: &mut Request) -> Option<PencilResult> {
    request.extensions_data.insert::<KeyType>(Value(100));
    None
}

fn github(_: &mut Request) -> PencilResult {
    return redirect("https://github.com/", 302);
}

fn login(_: &mut Request) -> PencilResult {
    return abort(401);
}

fn search(request: &mut Request) -> PencilResult {
    let keyword = match request.args().get("q") {
        Some(q) => q as &str,
        None => "",
    };
    Ok(Response::from(format!("You are searching for {}", keyword)))
}

fn hi_module(_: &mut Request) -> PencilResult {
    Ok("Hi module.".into())
}

fn main() {
    let mut app = Pencil::new(env::var("HELLO_ROOT_PATH").unwrap().as_ref());
    app.set_debug(true);
    app.set_log_level();
    env_logger::init();
    app.enable_static_file_handling();
    app.register_template("hello.html");
    app.before_request(before_request);

    app.httperrorhandler(404, page_not_found);

    app.get("/", "hello", hello);
    app.get("/user/<user_id:int>", "user", user);
    app.get("/info", "app_info", app_info);
    app.get("/hello_template", "hello_template", hello_template);
    app.get("/github", "github", github);
    app.get("/login", "login", login);
    app.get("/search/", "search", search);

    let mut demo_module = Module::new("demo", "/web/hello/demo");
    demo_module.route("/demo/hi", &[Get], "hi", hi_module);
    app.register_module(demo_module);

    debug!("* Running on http://localhost:5000/");
    app.run("127.0.0.1:5000");
}
