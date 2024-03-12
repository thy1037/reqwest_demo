mod platform;

use std::sync::Arc;
use platform::{
    invoke::Invoke, outcome::OutCome, baidu::Baidu, qq::Qq
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let input: String = "input".to_string();

    let mut backends: Vec::<Arc<dyn Invoke>> = Vec::new();
    backends.push(Arc::new(Baidu::new("baidu")));
    backends.push(Arc::new(Qq::new("qq")));


    let mut handles = Vec::with_capacity(backends.len());
    for backend in backends {
        handles.push(tokio::spawn(process(backend, input)))
    }

    for handle in handles {
        let m = handle.await;
        if let Ok(m) = m {
            if let Ok(m) = m {
                println!("{}", m.msg());
                println!("{}", m.tip());
            }
        }
    }

    Ok(())
}

async fn process(backend: Arc<dyn Invoke + 'static>, msg: String) -> Result<OutCome, &'static str> {
    let rsp = backend.to_owned()
        .request_builder(&msg)
        .send()
        .await;

    if let Ok(rsp) = rsp {
        if !rsp.status().is_success() {
            // println!("Server returned an error: {}", resp.status());
            Err("response status error.")
        } else {
            let out = rsp.text().await;
            if let Ok(out) = out {
                Ok(backend.outcome(&out))
            } else {
                Err("text error.")
            }
        }
    } else {
        Err("request error.")
    }
}
