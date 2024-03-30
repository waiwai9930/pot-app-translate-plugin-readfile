use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::thread;
use std::time::Duration;

#[no_mangle]
pub fn translate(
    text: &str, // 待翻译文本
    from: &str, // 源语言
    to: &str,   // 目标语言
    detect: &str, // 检测到的语言 (若使用 detect, 需要手动转换)
    needs: HashMap<String, String>, // 插件需要的其他参数,由info.json定义
) -> Result<Value, Box<dyn Error>> {
    let folder_path = Path::new(r"C:\Users\yy\ollama\t5_translate");
    let text_path = folder_path.join("text.txt");
    let zh_path = folder_path.join("zh.txt");

    let mut file = fs::File::create(text_path)?;
    file.write_all(text.as_bytes())?;
    drop(file);

    loop {
        if zh_path.exists() {
            let zh_text = fs::read_to_string(zh_path)?;
            fs::remove_file(zh_path)?;
            return Ok(Value::String(zh_text));
        }
        thread::sleep(Duration::from_millis(200));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let mut needs = HashMap::new();
        needs.insert("requestPath".to_string(), "lingva.pot-app.com".to_string());
        let result = translate("你好 世界！", "auto", "en", "zh_cn", needs).unwrap();
        println!("{result}");
    }
}
