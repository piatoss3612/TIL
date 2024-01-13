use ollama_example::{consts::MODEL, gen::gen_stream_print, Result};
use ollama_rs::{
    generation::completion::{request::GenerationRequest, GenerationContext},
    Ollama,
};
use simple_fs::{ensure_file_dir, save_json};

#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default(); // localhost:11434

    let model = MODEL.to_string();

    let prompts = &[
        "Why the sky is red? (be concise)",
        "What was my first question?",
    ];

    let mut last_ctx: Option<GenerationContext> = None;

    for prompt in prompts {
        println!("\nPrompt: {}", prompt);

        let mut req = GenerationRequest::new(model.to_string(), prompt.to_string());

        if let Some(last_ctx) = last_ctx.take() {
            req = req.context(last_ctx);
        }

        let final_data = gen_stream_print(&ollama, req).await?;

        if let Some(final_data) = final_data {
            last_ctx = Some(final_data.context);

            let ctx_file_path = "data/ctx.json";
            ensure_file_dir(ctx_file_path)?;
            save_json(ctx_file_path, &last_ctx)?;
        }
    }

    Ok(())
}
