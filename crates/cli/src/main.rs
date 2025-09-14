use tracing_subscriber::{EnvFilter, fmt::Subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    println!("rust-agent CLI (stub)");
    println!("core ping: {}", rust_agent_core::ping());
    println!("llm ping: {}", rust_agent_llm::ping());
    Ok(())
}
