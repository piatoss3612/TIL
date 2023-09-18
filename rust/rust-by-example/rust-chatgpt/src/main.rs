use dotenv::dotenv;
use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env;
use std::io::{stdin, stdout, Write};

// a struct to work with the API response
#[derive(Serialize, Deserialize, Debug)]
struct OAIResponse {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<OAIChoices>,
}

// a struct for the choices
#[derive(Serialize, Deserialize, Debug)]
struct OAIChoices {
    text: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
}

// a struct for the request you will make to the API
#[derive(Serialize, Deserialize, Debug)]
struct OAIRequest {
    prompt: String,
    max_tokens: u16,
    // temperature: f32,
}

// tokio async main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // load .env file
    dotenv().ok();

    // create a HttpsConnector, hyper
    let https = HttpsConnector::new();

    // create a client
    let client = Client::builder().build(https);

    let uri = "https://api.openai.com/v1/engines/text-davinci-001/completions"; // URI to which we will make the request
    let preamble = "Generate a Sql code for the given statement."; // preamble for the prompt

    let oai_token: String = env::var("OAI_TOKEN").unwrap(); // get the token from the .env file
    let auth_header_val = format!("Bearer {}", oai_token); // create the auth header value

    loop {
        print!("> ");
        stdout().flush().unwrap(); // flush the stdout
        let mut user_text = String::new(); // create a mutable string

        stdin()
            .read_line(&mut user_text)
            .expect("Did not enter a correct string"); // read the user input

        println!("You entered: {}", &user_text); // print the user input

        let sp = Spinner::new(&Spinners::Dots12, "\t\tOpenAI is Thinking...".into()); // create a spinner

        let oai_request = OAIRequest {
            prompt: format!("{} {}", preamble, user_text), // create the prompt
            max_tokens: 1000,                              // max tokens
        };

        let body = Body::from(serde_json::to_vec(&oai_request)?); // create the body

        let req = Request::post(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::AUTHORIZATION, &auth_header_val)
            .body(body)
            .unwrap(); // create the request

        let res = client.request(req).await?; // make the request

        let body = hyper::body::aggregate(res).await?;

        // let json: OAIResponse = serde_json::from_reader(body.reader())?; // read the response

        sp.stop();

        println!("");

        // println!("{}", json.choices[0].text); // print the response

        println!("{}", std::str::from_utf8(body.chunk()).unwrap());
    }
}
