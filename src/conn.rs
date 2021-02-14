use crate::models::*;
use anyhow::Result;
use hyper::{
    service::{make_service_fn, service_fn},
    {Body, Method, Request, Response, Server, StatusCode},
};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

pub async fn server(
    json_map: Arc<Mutex<HashMap<String, CmdResultJSON>>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8931));

    #[rustfmt::skip]
    let service 
        = make_service_fn( move |_conn| {
                let cln = json_map.clone();
                async {
                    Ok::<_, anyhow::Error> ( 
                        service_fn( 
                            move |req| {
                                handler(req, cln.clone())
                            }
                        )
                    )
                }
            }
        );

    #[rustfmt::skip]
    let server 
        = Server::bind(&addr).serve(service);

    server.await?;

    Ok(())
}

async fn handler(
    req: Request<Body>,
    json_map: Arc<Mutex<HashMap<String, CmdResultJSON>>>,
) -> Result<Response<Body>, anyhow::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/cmd-result") => {
            let (_, body) = req.into_parts();
            let body_bytes = hyper::body::to_bytes(body).await?;

            let body = body_bytes
                .to_vec()
                .iter()
                .map(|&c| c as char)
                .collect::<String>();

            println!("{}", &body);

            let res = serde_json::from_str(&body);
            if let Err(e) = res {
                return Ok(hyper::Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from(format!("400: Bad Request\n{}", e)))
                    .unwrap());
            }

            let cmd_result: CmdResultJSON = res.unwrap();

            json_map
                .lock()
                .unwrap()
                .insert(cmd_result.session_id.clone(), cmd_result.clone());

            Ok(Response::new(Body::from("ok")))
        }
        _ => {
            let resp = hyper::Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("404: Not Found"))
                .unwrap();
            Ok(resp)
        }
    }
}



/* 
#[cfg(test)]
mod tests {
    use hyper::{Body, Client, Method, Request};

    use crate::models::*;
    use crate::server::server;
    use std::sync::Arc;
    use std::{collections::HashMap, sync::Mutex};

    #[tokio::test(flavor = "multi_thread")]
    async fn server_test() -> Result<(), anyhow::Error> {
        let hashmap: HashMap<String, CmdResultJSON> = HashMap::new();
        let sp = Arc::new(Mutex::new(hashmap));
        let sp_ = sp.clone();

        tokio::spawn(async {
            let res = server(sp_).await;
            if let Err(e) = res {
                eprintln!("{:?}", e);
            }
        });

        let payload = r#"
            {
                "session_id": "test",
                "time": 10,
                "result": true,
                "message": "this is test",
                "mem_usage": 8931,
                "stdout_size": 314,
                "timeout": false,
                "testcase_result": {
                    "submit_id": 1,
                    "testcase_id": 1,
                    "status": "AC",
                    "execution_time": 10,
                    "execution_memory": 128
                }
            }
        "#;
        let json_struct: CmdResultJSON = serde_json::from_str(payload).unwrap();

        let request = Request::builder()
            .method(Method::POST)
            .uri("http://127.0.0.1:8931/cmd-result")
            .body(Body::from(payload))
            .unwrap();

        let cli = Client::new();
        let _ = cli.request(request).await?;

        let sp_ = sp.clone();
        let map = sp_.lock().unwrap();
        let x = map.get("test").unwrap();

        assert_eq!(&json_struct, x);

        Ok(())
    }
}
*/