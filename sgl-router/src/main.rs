use sglang_router_rs::config::{PolicyConfig, RouterConfig, RoutingMode};
use sglang_router_rs::server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let policy_config = PolicyConfig::CacheAware {
    //     // 设置默认值
    //     cache_threshold: 0.5,
    //     balance_abs_threshold: 32,
    //     balance_rel_threshold: 1.0001,
    //     eviction_interval_secs: 60,
    //     max_tree_size: 2usize.pow(24),
    // };
    let mode = RoutingMode::Regular {
        worker_urls: vec![],
    };
    // let mode = RoutingMode::Regular {
    //     worker_urls: vec![
    //         "http://119.255.238.67:8007".to_string(),
    //     ],
    // };

    let router_config = RouterConfig::new(mode, PolicyConfig::RoundRobin);
    // 解析命令行参数
    // let args = Args::parse();
    // 从配置文件或命令行参数构建 RouterConfig
    // let router_config = RouterConfig::from_cli_args(&args)?;

    server::startup(server::ServerConfig {
        host: "0.0.0.0".to_string(),
        port: 9999,
        router_config,
        // 必须显式设置这两个非Option字段
        max_payload_size: 256 * 1024 * 1024, // 例如10MB
        request_timeout_secs: 30,           // 例如30秒超时
        // 可选字段可以设为None或具体值
        // log_dir: Some("/Users/hhl/Documents/chehaoduo/code/sglang/sgl-router/logs".to_string()),
        log_dir: None,
        log_level: Some("debug".to_string()),
        service_discovery_config: None,
        prometheus_config: None,
        request_id_headers: None,
    }).await?;

    Ok(())
}
