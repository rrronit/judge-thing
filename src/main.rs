use axum::{response::Response, routing::post, Router};
use std::process::Command;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(run_code));

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");
    
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to start server");
}

async fn run_code() -> Response<String> {
    // Code to execute
    let code = "print('HellodsljfldskafWorld')";

    // Initialize the isolate
    match Command::new("isolate").arg("--init").output() {
        Ok(v) => {
            println!(
                "Isolate initialized: {}",
                String::from_utf8(v.stdout).unwrap()
            );
        }
        Err(e) => {
            return Response::builder()
                .status(500)
                .body(format!("Failed to initialize isolate: {}", e))
                .unwrap();
        }
    }

    // Execute the code
    let output = match Command::new("isolate")
        .args(["--run", "--", "/usr/bin/gcc", "-c", code])
        .output()
    {
        Ok(output) => output.stdout,
        Err(e) => {
            return Response::builder()
                .status(500)
                .body(format!("Failed to execute code: {}", e))
                .unwrap()
        }
    };

    // Clean up the isolate
    if let Err(e) = Command::new("isolate").arg("--cleanup").output() {
        println!("Failed to cleanup isolate: {}", e);
        return Response::builder()
            .status(500)
            .body(format!("Failed to cleanup isolate: {}", e))
            .unwrap();
    }

    println!("Output: {}", String::from_utf8(output.clone()).unwrap());
    Response::builder()
        .status(200)
        .body(String::from_utf8(output).unwrap())
        .unwrap()
}
