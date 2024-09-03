use std::net::SocketAddr;

use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Error;
use hyper::{body::Body, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use serde::{Deserialize, Serialize};

const PORT: u16 = 12345;

pub mod sequence;

use sequence::geometric::Geometric;
use sequence::arithmetic::Arithmetic;
use sequence::constant::Constant;
use sequence::sum::Sum;
use sequence::product::Produkt;
use sequence::drop::Drop;

fn a() {
    let k = Geometric::new(1.1,2.);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub ip: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Range {
    pub from: u64,
    pub to: u64,
    pub step: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SequenceSyntax {
    pub name: String,
    pub parameters: Vec<f64>,
    pub sequences: Vec<Box<SequenceSyntax>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SequenceRequest {
    pub range: Range,
    pub parameters: Vec<f64>,
    pub sequences: Vec<Box<SequenceSyntax>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SequenceInfo {
    name: String,
    description: String,
    parameters: u32,
    sequences: u32,
}

// pub struct Arithmetic {
//     start: f64,
//     step: f64,
// }
// 
// pub struct Constant {
//     
// }
// 
// 
// 
// impl Arithmetic {
//     pub fn new(start: f64, step: f64) -> Box<Arithmetic> {
//         Box::new(Arithmetic { start, step })
//     }
// 
//     pub fn k_th(&self, k: usize) -> f64 {
//         self.start + (k as f64) * self.step
//     }
// 
//     pub fn range(&self, range: Range) -> Vec<f64> {
//         let mut result = Vec::new();
//         let mut k = range.from;
//         while k <= range.to {
//             result.push(self.k_th(k as usize));
//             k += range.step;
//         }
//         result
//     }
// }
// 
// pub struct Constant {
//     value: f64,
// }
// 
// impl Constant {
//     pub fn new(value: f64) -> Box<Constant> {
//         Box::new(Constant { value })
//     }
// 
//     pub fn k_th(&self, k: usize) -> f64 {
//         self.value
//     }
// 
//     pub fn range(&self, range: Range) -> Vec<f64> {
//         let mut result = Vec::new();
//         let mut k = range.from;
//         while k <= range.to {
//             result.push(self.value);
//         }
//         result
//     }
// }
// 
// pub struct Geometric {
//     start: f64,
//     quot: f64,
// }
// 
// impl Geometric {
//     pub fn new(start: f64, quot: f64) -> Box<Geometric> {
//         Box::new(Geometric { start, quot })
//     }
// 
//     pub fn k_th(&self, k: usize) -> f64 {
//         self.start * self.quot.powi(k as i32)
//     }
// 
//     pub fn range(&self, range: Range) -> Vec<f64> {
//         let mut result = Vec::new();
//         let mut k = range.from;
//         while k <= range.to {
//             result.push(self.k_th(k as usize));
//             k = k * range.quot;
//         }
//         result
//     }
// }
// 
// 
// 
fn sequences() -> Vec<SequenceInfo> {
    let mut sequences = Vec::new();
    sequences.push(SequenceInfo {
        name: "Arithmetic".to_string(),
        description: "Arithmetic sequence".to_string(),
        parameters: 2,
        sequences: 0,
    });
    let mut sequences = Vec::new();
    sequences.push(SequenceInfo {
        name: "Geometric".to_string(),
        description: "Geometric sequence".to_string(),
        parameters: 2,
        sequences: 0,
    });
    let mut sequences = Vec::new();
    sequences.push(SequenceInfo {
        name: "Tribonacci".to_string(),
        description: "Tribonacci sequence".to_string(),
        parameters: 3,
        sequences: 0,
    });
    let mut sequences = Vec::new();
    sequences.push(SequenceInfo {
        name: "Sum".to_string(),
        description: "Sum of two sequences by term".to_string(),
        parameters: 0,
        sequences: 2,
    });
    let mut sequences = Vec::new();
    sequences.push(SequenceInfo {
        name: "Product".to_string(),
        description: "Product of two sequences by term".to_string(),
        parameters: 0,
        sequences: 2,
    });
    let mut sequences = Vec::new();
    sequences.push(SequenceInfo {
        name: "Drop".to_string(),
        description: "Dropping the first k terms of a sequence".to_string(),
        parameters: 1,
        sequences: 1,
    });
    let mut sequences = Vec::new();
    sequences.push(SequenceInfo {
        name: "Lah".to_string(),
        description: "Generates a sequence of the number of ways a set can be  paritioned into k linearly ordered 
                    terms, where set size increases by term and k is the parameter chosen.".to_string(),
        parameters: 1,
        sequences: 0,
    });
    let mut sequences = Vec::new();
    sequences.push(SequenceInfo {
        name: "Catalan".to_string(),
        description: "Sequence of Catalan numbers".to_string(),
        parameters: 0,
        sequences: 0,
    });
    let mut sequences = Vec::new();
    sequences.push(SequenceInfo {
        name: "Base".to_string(),
        description: "Changes the sequence from one number system to another. Only works for bases from 2 to 10.".to_string(),
        parameters: 2,
        sequences: 1,
    });
    let mut sequences = Vec::new();
    sequences.push(SequenceInfo {
        name: "Aliquot".to_string(),
        description: "A sequence of positive integers in which each term is the sum of proper divisors of the previous term. ".to_string(),
        parameters: 1,
        sequences: 0,
    });
    //sequences.push(SequenceInfo {
    //    name: "Lin Comb".to_string(),
    //    description: "".to_string(),
    //    parameters: 3, 
    //    sequences: 2,
    //});
    //sequences
}

fn get_project() -> Project {
    return Project {
        name: "Jaka & Tinkara".to_string(),
        ip: "0.0.0.0".to_string(),
        port: PORT,
    };
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
async fn collect_body(req: Request<Incoming>) -> Result<String, hyper::Error> {
    let max = req.body().size_hint().upper().unwrap_or(u64::MAX);
    if max > 1024 * 64 {
        panic!("Body too big");
    }

    let whole_body = req.collect().await?.to_bytes();
    let whole_body = std::str::from_utf8(&whole_body).unwrap().to_string();
    return Ok(whole_body);
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

async fn send_post(url: String, body: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.post(url).body(body).send().await?.text().await?;
    return Ok(res);
}

async fn send_get(url: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?.text().await?;
    return Ok(res);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = ([0, 0, 0, 0], PORT).into();

    let b = send_get("http://0.0.0.0:7878/project".to_string()).await?;
    println!("HERE {}", b);

    let b = send_post(
        "http://0.0.0.0:7878/project".to_string(),
        serde_json::to_string(&get_project()).unwrap(),
    )
    .await?;
    println!("HERE {}", b);

    let b = send_get("http://0.0.0.0:7878".to_string()).await?;
    println!("HERE {}", b);

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    let create_404 = || {
        let mut not_found = Response::new(empty());
        *not_found.status_mut() = StatusCode::NOT_FOUND;
        Ok(not_found)
    };

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        let service = service_fn(move |req| {
            async move {
                match (req.method(), req.uri().path()) {
                    (&Method::GET, "/ping") => Ok::<_, Error>(Response::new(full(
                        serde_json::to_string(&get_project()).unwrap(),
                    ))),
                    (&Method::GET, "/sequence") => {
                        //
                        let sequences = sequences();
                        Ok(Response::new(full(
                            serde_json::to_string(&sequences).unwrap(),
                        )))
                    }
                    (&Method::POST, r) => {
                        let seqs = sequences();
                        let sequences = seqs
                            // .iter()
                            .find(|&x| ("/sequence/".to_string() + &x.name) == r);
                        match sequences {
                            None => create_404(),
                            Some(s) if *s.name == "Arithmetic".to_string() => {
                                let body = collect_body(req).await?;
                                let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                                let range = request.range;
                                let seq =
                                    Arithmetic::new(request.parameters[0], request.parameters[1]);
                                Ok(Response::new(full(
                                    serde_json::to_string(&seq.range(range)).unwrap(),
                                )))
                            }
                            _ => panic!("Not implemented"),
                        }
                    }

                    _ => create_404(),
                }
            }
        });

        if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
            println!("Error serving connection: {:?}", err);
        }
    }
}
