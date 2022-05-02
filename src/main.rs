use std::{collections::HashMap, process::Command, thread};

use warp::Filter;

use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;


#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    // POST /employees/:rate  {"name":"Sean","rate":2}
    let promote = warp::post()
        .and(warp::path("git_webhook"))
        .map(git_webhook_handler);
        //.map(|req: HashMap<String, String>| format!("git_webhook, req ==> {:#?}", req));

    warp::serve(hello.or(promote))
        .run(([127, 0, 0, 1], 8080))
        .await;
}

fn git_webhook_handler() -> String{
    let mut map = HashMap::new();
    map.insert("code", "0");
    map.insert("message", "success");

    thread::spawn(run_shell);
    
    //return format!("out ==> {}\nerr ==> {}", output_str, err_str);

    return format!("{:#?}", map);
}

fn run_shell() {
    let output = Command::new("sh").arg("-c").arg("/root/shell/git_webhook.sh").output().expect("sh exec error!");
    let output_str = String::from_utf8_lossy(&output.stdout);
    let err_str = String::from_utf8_lossy(&output.stderr);

    let fmt = "%Y-%m-%d %H:%M:%S";
    let str_date = Local::now().format(fmt).to_string();

    let mut file = OpenOptions::new().append(true).open("/root/git_webhook_log.txt").expect(
        "cannot open file");
    
    file.write_all((str_date.to_string() + " output_str ==> ").as_bytes()).expect("write failed");
    file.write_all(output_str.as_bytes()).expect("write failed");
    
    file.write_all("\n".as_bytes()).expect("write failed");

    file.write_all((str_date.to_string() + " err_str ==> ").as_bytes()).expect("write failed");
    file.write_all(err_str.as_bytes()).expect("write failed");
}