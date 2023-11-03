use ntex::web;
use rlimit::{getrlimit, setrlimit, Resource};

#[ntex::main]
async fn main() {
    let (soft, hard) = getrlimit(Resource::NOFILE).unwrap();
    println!("Current rlimit(NOFILE) is soft:{soft}, hard:{hard}\nSetting to 1024/1024");
    setrlimit(Resource::NOFILE, 1024, 1024).unwrap();

    for i in 0.. {
        println!("i={i}");

        let srv = web::server(|| {
            web::App::new().default_service(web::to(|| async { web::HttpResponse::Ok() }))
        })
        .bind("127.0.0.1:8989")
        .unwrap()
        .stop_runtime()
        .disable_signals()
        .run();

        srv.stop(false).await;
    }
}
