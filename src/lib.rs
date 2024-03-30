use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use urlencoding::encode;
use std::io::Write;
use std::fs::File;

#[no_mangle]
pub fn translate(
    text: &str, // 待翻译文本
    from: &str, // 源语言
    to: &str,   // 目标语言
    // (pot会根据info.json 中的 language 字段传入插件需要的语言代码，无需再次转换)
    detect: &str, // 检测到的语言 (若使用 detect, 需要手动转换)
    needs: HashMap<String, String>, // 插件需要的其他参数,由info.json定义
) -> Result<Value, Box<dyn Error>> {
     let file_path = match needs.get("path") {
        Some(path) => path.to_string(),
        None => return Err("文件路径错误".into()),
    };

    let mut file = File::create(file_path)?;
    file.write_all(text.as_bytes())?;
    
    Ok(Value::String(text.to_string()))
}
