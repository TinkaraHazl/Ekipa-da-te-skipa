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

use crate::sequence::Sequence;


use sequence::arithmetic::Arithmetic;
use sequence::catalan::Catalan;
use sequence::constant::Constant;
use sequence::aliquot::Aliquot;
use sequence::base::Base;
use sequence::drop::Drop;
use sequence::geometric::Geometric;
use sequence::lah::Lah;
use sequence::mix::Mix;
use sequence::product::Product;
use sequence::sum::Sum;
use sequence::tribonacci::Tribonacci;



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

fn sequences() -> Vec<SequenceInfo> {
    let mut sequences = Vec::new();
    
    sequences.push(SequenceInfo {
        name: "Arithmetic".to_string(),
        description: "Arithmetic sequence".to_string(),
        parameters: 2,
        sequences: 0,
    });
    
    sequences.push(SequenceInfo {
        name: "Geometric".to_string(),
        description: "Geometric sequence".to_string(),
        parameters: 2,
        sequences: 0,
    });
    
    sequences.push(SequenceInfo {
        name: "Tribonacci".to_string(),
        description: "Tribonacci sequence".to_string(),
        parameters: 3,
        sequences: 0,
    });
    
    sequences.push(SequenceInfo {
        name: "Sum".to_string(),
        description: "Sum of two sequences by term".to_string(),
        parameters: 0,
        sequences: 2,
    });
    
    sequences.push(SequenceInfo {
        name: "Product".to_string(),
        description: "Product of two sequences by term".to_string(),
        parameters: 0,
        sequences: 2,
    });
    
    sequences.push(SequenceInfo {
        name: "Mix".to_string(),
        description: "Generates a new sequence by alternating elements from two 
        provided sequences. The 'step' parameter determines how many elements
        from each sequence are included before switching and is rounded to the 
        greatest integer less than or equal to the input. If 'step' is less
        than one, the function panics.".to_string(),
        parameters: 2,
        sequences: 2,
    });
    
    sequences.push(SequenceInfo {
        name: "Drop".to_string(),
        description: "Dropping the first k terms of a sequence".to_string(),
        parameters: 1,
        sequences: 1,
    });
    
    sequences.push(SequenceInfo {
        name: "Lah".to_string(),
        description: "Generates a sequence of the number of ways a set can be  paritioned into k linearly ordered 
                    terms, where set size increases by term and k is the parameter chosen.".to_string(),
        parameters: 1,
        sequences: 0,
    });
    
    sequences.push(SequenceInfo {
        name: "Catalan".to_string(),
        description: "Sequence of Catalan numbers".to_string(),
        parameters: 0,
        sequences: 0,
    });
    
    sequences.push(SequenceInfo {
        name: "Base".to_string(),
        description: "Changes the sequence from one number system to another. 
                    Only works for bases from 2 to 10.".to_string(),
        parameters: 2,
        sequences: 1,
    });
    
    sequences.push(SequenceInfo {
        name: "Aliquot".to_string(),
        description: "A sequence of positive integers in 
        which each term is the sum of proper divisors of the previous term. ".to_string(),
        parameters: 1,
        sequences: 0,
    });
    
    sequences.push(SequenceInfo {
        name: "Constant".to_string(),
        description: "A sequence that repeats the same value".to_string(),
        parameters: 1,
        sequences: 0,
    });
    
    sequences
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

fn create_sequence(name: &str, parameters: Vec<f64>, sequences: Vec<Box<dyn Sequence>>) -> Option<Box<dyn Sequence>> {
    match name {
        "Constant" => {
            if parameters.len() == 1 && sequences.is_empty() {
                Some(Constant::new(parameters[0]))
            } else {
                None
            }
                }
        "Arithmetic" => {
            if parameters.len() == 2 && sequences.is_empty() {
                Some(Box::new(Arithmetic::new(parameters[0], parameters[1])))
            } else {
                None
            }
        }
        "Geometric" => {
            if parameters.len() == 2 && sequences.is_empty() {
                Some(Geometric::new(parameters[0], parameters[1]))
            } else {
                None
            }
        }
        "Tribonacci" => {
            if parameters.len() == 3 && sequences.is_empty() {
                Some(Box::new(Tribonacci::new(parameters[0], parameters[1], parameters[2])))
            } else {
                None
            }
        }
        "Sum" => {
            if parameters.is_empty() && sequences.len() == 2 {
                let mut iter = sequences.into_iter();
                let seq1 = iter.next()?; // Extract the first element
                let seq2 = iter.next()?; // Extract the second element
                Some(Sum::new(seq1, seq2)) // Pass ownership to `Sum::new`
            } else {
                None
            }
        }

        "Product" => {
            if parameters.is_empty() && sequences.len() == 2 {
                let mut iter = sequences.into_iter();
                let seq1 = iter.next()?; // Extract the first element
                let seq2 = iter.next()?; // Extract the second element
                Some(Product::new(seq1, seq2)) // Pass ownership to `Sum::new`
            } else {
                None
            }
        }
        "Mix" => {
            if parameters.len() == 2 && sequences.len() == 2 {
                let mut iter = sequences.into_iter();
                let seq1 = iter.next()?; // First element
                let seq2 = iter.next()?; // Second element
                Some(Mix::new(seq1, seq2, parameters[0], parameters[1])) // Pass ownership to `SomeSequence::new`
            } else {
                None
            }
        }

        "Drop" => {
            if parameters.len() == 1 && sequences.len() == 1 {
                let mut iter = sequences.into_iter();
                let seq1 = iter.next()?; // Extract the first element
                Some(Drop::new(seq1, parameters[0] as usize))
            } else {
                None
            }
        }
        "Lah" => {
            if parameters.len() == 1 && sequences.is_empty() {
                Some(Lah::new(parameters[0]))
            } else {
                None
            }
        }
        "Catalan" => {
            if parameters.is_empty() && sequences.is_empty() {
                Some(Catalan::new())
            } else {
                None
            }
        }
        "Base" => {
            if parameters.len() == 2 && sequences.len() == 1 {
                let mut iter = sequences.into_iter();
                let seq1 = iter.next()?; // Extract the first element
                Some(Base::new(seq1, parameters[0] as usize, parameters[1] as usize))
            } else {
                None
            }
        }
        "Aliquot" => {
            if parameters.len() == 1 && sequences.is_empty() {
                Some(Box::new(Aliquot::new(parameters[0] as usize)))
            } else {
                None
            }
        }
        _ => None,
    }
}


async fn handle_sequence_request(
    req: Request<Incoming>, 
    sequence_info: &SequenceInfo
) -> Result<Response<BoxBody<Bytes, Error>>, hyper::Error> {
    let body = collect_body(req).await?;
    let request: SequenceRequest = match serde_json::from_str(&body) {
        Ok(req) => req,
        Err(e) => {
            eprintln!("Failed to parse request: {}", e);
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(full(format!("Invalid request: {}", e)))
                .unwrap());
        }
    };

    let range = request.range;

    let sub_sequences: Option<Vec<Box<dyn Sequence>>> = request
        .sequences
        .into_iter()
        .map(|syntax| {
            create_sequence(
                &syntax.name,
                syntax.parameters.clone(),
                syntax.sequences
                    .into_iter()
                    .filter_map(|sub_syntax| {
                        create_sequence(
                            &sub_syntax.name,
                            sub_syntax.parameters.clone(),
                            vec![],
                        )
                    })
                    .collect::<Vec<Box<dyn Sequence>>>(),
            )
        })
        .collect::<Option<Vec<Box<dyn Sequence>>>>();

    if sub_sequences.is_none() {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(full("Invalid sub-sequences in request".to_string()))
            .unwrap());
    }

    let sequence = create_sequence(
        &sequence_info.name,
        request.parameters,
        sub_sequences.unwrap(),
    );

    match sequence {
        Some(seq) => {
            let result = serde_json::to_string(&seq.range(range)).unwrap();
            Ok(Response::new(full(result)))
        }
        None => {
            eprintln!("Failed to create sequence: {}", sequence_info.name);
            Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(full("Failed to create sequence".to_string()))
                .unwrap())
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = ([0, 0, 0, 0], PORT).into();

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
            let method = req.method().clone();
            let path = req.uri().path().to_string();

            async move {
                match (method, path.as_str()) {
                    // Handle both /ping and /ping/ endpoints
                    (Method::GET, "/ping") | (Method::GET, "/ping/") => {
                        let project = get_project();
                        Ok::<_, Error>(Response::new(full(
                            serde_json::to_string(&project).unwrap(),
                        )))
                    },

                    (Method::GET, "/sequence") => {
                        let sequences = sequences();
                        
                        match send_get("http://other-generator:port/sequence".to_string()).await {
                            Ok(other_sequences) => {
                                println!("Got sequences from other generator: {}", other_sequences);
                                // You could combine sequences here if needed
                            },
                            Err(e) => println!("Could not get sequences from other generator: {}", e),
                        }
                        
                        Ok(Response::new(full(
                            serde_json::to_string(&sequences).unwrap(),
                        )))
                    },
                    (Method::POST, path) if path.starts_with("/sequence/") => {
                        let seq_name = &path["/sequence/".len()..];
                        
                        if seq_name.len() > 256 || !seq_name.chars().all(|c| c.is_alphanumeric()) {
                            return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(full("Invalid sequence name".to_string()))
                                .unwrap());
                        }
                        
                        let seq_info = sequences()
                            .into_iter()
                            .find(|info| info.name == seq_name);
                
                        match seq_info {
                            Some(info) => handle_sequence_request(req, &info).await,
                            None => {
                                // Try to get sequence from another generator
                                match send_post(
                                    format!("http://other-generator:port/sequence/{}", seq_name),
                                    collect_body(req).await?.to_string()
                                ).await {
                                    Ok(response) => Ok(Response::new(full(response))),
                                    Err(_) => create_404(),
                                }
                            },
                        }
                    },
                    _ => create_404(),
                }
            }
        });

        if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
            println!("Error serving connection: {:?}", err);
        }
    }
}

