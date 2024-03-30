use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use urlencoding::encode;

#[no_mangle]
pub fn translate(
    text: &str, // 待翻译文本
    from: &str, // 源语言
    to: &str,   // 目标语言
    // (pot会根据info.json 中的 language 字段传入插件需要的语言代码，无需再次转换)
    detect: &str, // 检测到的语言 (若使用 detect, 需要手动转换)
    needs: HashMap<String, String>, // 插件需要的其他参数,由info.json定义
) -> Result<(String, Value), Box<dyn Error>> {
    // 将待翻译文本返回作为结果的一部分
    let translated_text = text.to_string();

    if let Some(result) = parse_result(res) {
        // 返回待翻译文本和翻译结果的元组
        return Ok((translated_text, Value::String(result)));
    } else {
        return Err("Response Parse Error".into());
    }
}

