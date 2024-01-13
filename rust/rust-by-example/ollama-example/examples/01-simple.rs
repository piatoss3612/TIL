use ollama_example::{
    consts::{DEFAULT_SYSTEM_MOCK, MODEL},
    Result,
};
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};

#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default(); // localhost:11434

    let model = MODEL.to_string();

    let prompt = "What is the best programming language? (Be concise)".to_string();

    let gen_req = GenerationRequest::new(model, prompt).system(DEFAULT_SYSTEM_MOCK.to_string());

    let res = ollama.generate(gen_req).await?;
    println!("->> res {}", res.response);

    Ok(())
}
