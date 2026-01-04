mod tools;

use reqwest::blocking::Client;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result as RlResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use tools::ToolRegistry;

// ============== API 请求/响应结构 ==============

#[derive(Serialize, Clone, Debug)]
struct Message {
    role: String,
    content: MessageContent,
}

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
enum MessageContent {
    Text(String),
    Blocks(Vec<Value>),
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
    tools: Vec<Value>,
}

#[derive(Deserialize, Debug)]
struct AnthropicResponse {
    content: Vec<Value>,
    #[allow(dead_code)]
    stop_reason: Option<String>,
}

// ============== 配置结构 ==============

#[derive(Deserialize)]
struct Settings {
    env: Env,
}

#[derive(Deserialize)]
struct Env {
    #[serde(rename = "ANTHROPIC_AUTH_TOKEN")]
    api_key: String,
    #[serde(rename = "ANTHROPIC_BASE_URL")]
    base_url: String,
    #[serde(rename = "HTTPS_PROXY")]
    https_proxy: Option<String>,
}

// ============== Content Block 处理 ==============

/// 从 Value 中提取 content block 类型和数据
fn parse_content_block(block: &Value) -> Option<(&str, &Value)> {
    let block_type = block.get("type")?.as_str()?;
    Some((block_type, block))
}

/// 创建 tool_result block
fn create_tool_result(tool_use_id: &str, content: &str) -> Value {
    serde_json::json!({
        "type": "tool_result",
        "tool_use_id": tool_use_id,
        "content": content
    })
}

// ============== Chat Client ==============

struct ChatClient {
    client: Client,
    url: String,
    api_key: String,
    tool_registry: ToolRegistry,
    messages: Vec<Message>,
    model: String,
}

impl ChatClient {
    fn new(settings: &Settings) -> Result<Self, Box<dyn std::error::Error>> {
        let mut client_builder = Client::builder();
        if let Some(proxy_url) = &settings.env.https_proxy {
            let proxy = reqwest::Proxy::all(proxy_url)?;
            client_builder = client_builder.proxy(proxy);
        }
        let client = client_builder.build()?;

        Ok(Self {
            client,
            url: format!("{}/v1/messages", settings.env.base_url),
            api_key: settings.env.api_key.clone(),
            tool_registry: ToolRegistry::with_builtins(),
            messages: Vec::new(),
            model: "claude-opus-4-5-20251101".to_string(),
        })
    }

    fn send_message(&mut self, user_input: &str) -> Result<(), Box<dyn std::error::Error>> {
        // 添加用户消息
        self.messages.push(Message {
            role: "user".to_string(),
            content: MessageContent::Text(user_input.to_string()),
        });

        // Tool Use 循环
        loop {
            let request_body = AnthropicRequest {
                model: self.model.clone(),
                max_tokens: 4096,
                messages: self.messages.clone(),
                tools: self.tool_registry.definitions(),
            };

            let response = self
                .client
                .post(&self.url)
                .header("x-api-key", &self.api_key)
                .header("anthropic-version", "2023-06-01")
                .header("content-type", "application/json")
                .json(&request_body)
                .send()?;

            if !response.status().is_success() {
                let status = response.status();
                let error_text = response.text()?;
                eprintln!("❌ API Error [{}]: {}", status, error_text);
                self.messages.pop();
                return Ok(());
            }

            // 先获取原始文本，便于调试
            let response_text = response.text()?;
            let result: AnthropicResponse = match serde_json::from_str(&response_text) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("❌ JSON 解析错误: {}", e);
                    eprintln!("📄 原始响应 (前 500 字符): {}", &response_text[..response_text.len().min(500)]);
                    self.messages.pop();
                    return Ok(());
                }
            };

            // 处理响应内容
            let mut tool_results: Vec<Value> = Vec::new();
            let mut has_tool_use = false;

            for block in &result.content {
                if let Some((block_type, data)) = parse_content_block(block) {
                    match block_type {
                        "text" => {
                            if let Some(text) = data.get("text").and_then(|t| t.as_str()) {
                                println!("\n🤖 {}\n", text);
                            }
                        }
                        "thinking" => {
                            if let Some(thinking) = data.get("thinking").and_then(|t| t.as_str()) {
                                // 截取前 200 字符显示
                                let display = if thinking.len() > 200 {
                                    format!("{}...", &thinking[..200])
                                } else {
                                    thinking.to_string()
                                };
                                println!("\n💭 [思考中...] {}\n", display);
                            }
                        }
                        "tool_use" => {
                            has_tool_use = true;
                            let id = data.get("id").and_then(|v| v.as_str()).unwrap_or("");
                            let name = data.get("name").and_then(|v| v.as_str()).unwrap_or("");
                            let input = data.get("input").unwrap_or(&Value::Null);

                            println!("  🔧 [{}] {}", name, serde_json::to_string(input)?);

                            let tool_output = self.tool_registry.execute(name, input);
                            tool_results.push(create_tool_result(id, &tool_output));
                        }
                        _ => {
                            // 忽略其他未知类型
                        }
                    }
                }
            }

            // 添加 assistant 消息（保留原始 content）
            self.messages.push(Message {
                role: "assistant".to_string(),
                content: MessageContent::Blocks(result.content.clone()),
            });

            // 检查是否需要继续循环
            if !has_tool_use {
                break;
            }

            // 添加 tool_result 消息
            self.messages.push(Message {
                role: "user".to_string(),
                content: MessageContent::Blocks(tool_results),
            });
        }

        Ok(())
    }

    fn clear_history(&mut self) {
        self.messages.clear();
        println!("📝 对话历史已清除\n");
    }

    fn tool_count(&self) -> usize {
        self.tool_registry.len()
    }
}

// ============== REPL 命令处理 ==============

fn handle_command(cmd: &str, client: &mut ChatClient) -> bool {
    match cmd.trim() {
        "/exit" | "/quit" | "/q" => {
            println!("👋 再见！");
            return true;
        }
        "/clear" | "/c" => {
            client.clear_history();
        }
        "/tools" | "/t" => {
            println!("\n🔧 已注册的工具 ({}):", client.tool_count());
            for name in client.tool_registry.tool_names() {
                println!("  - {}", name);
            }
            println!();
        }
        "/help" | "/h" | "/?" => {
            println!(
                r#"
📚 可用命令:
  /exit, /quit, /q  - 退出程序
  /clear, /c        - 清除对话历史
  /tools, /t        - 显示已注册的工具
  /help, /h, /?     - 显示此帮助

💡 提示:
  - 直接输入问题即可与 AI 对话
  - AI 可以使用已注册的工具操作本地文件
  - 按 Ctrl+C 中断当前请求
  - 按 Ctrl+D 退出
"#
            );
        }
        _ => {
            println!("❓ 未知命令: {}，输入 /help 查看帮助", cmd);
        }
    }
    false
}

// ============== 主函数 ==============

fn main() -> RlResult<()> {
    // 读取配置
    let settings_path = ".mentat/settings.json";
    let settings_content = fs::read_to_string(settings_path).expect("无法读取配置文件");
    let settings: Settings = serde_json::from_str(&settings_content).expect("配置文件格式错误");

    // 创建 ChatClient
    let mut client = ChatClient::new(&settings).expect("创建客户端失败");

    // 创建 REPL 编辑器
    let mut rl = DefaultEditor::new()?;

    // 加载历史记录
    let history_path = ".mentat/history.txt";
    let _ = rl.load_history(history_path);

    println!(
        r#"
╔══════════════════════════════════════════════════════════╗
║                  🧠 Mentat Code v0.1.0                   ║
║                                                          ║
║  输入问题与 AI 对话，输入 /help 查看帮助                 ║
║  已加载 {} 个工具                                         ║
╚══════════════════════════════════════════════════════════╝
"#,
        client.tool_count()
    );

    loop {
        let readline = rl.readline("❯ ");
        match readline {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }

                // 添加到历史
                let _ = rl.add_history_entry(input);

                // 处理命令
                if input.starts_with('/') {
                    if handle_command(input, &mut client) {
                        break;
                    }
                    continue;
                }

                // 发送消息
                if let Err(e) = client.send_message(input) {
                    eprintln!("❌ 错误: {}", e);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("👋 再见！");
                break;
            }
            Err(err) => {
                eprintln!("❌ 读取错误: {:?}", err);
                break;
            }
        }
    }

    // 保存历史记录
    let _ = fs::create_dir_all(".mentat");
    let _ = rl.save_history(history_path);

    Ok(())
}
