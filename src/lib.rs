use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use urlencoding::encode;

#[no_mangle]
pub fn translate(
    text: &str, // 待翻译文本
    from: &str, // 源语言代码
    to: &str,   // 目标语言代码
    detect: &str, // 检测到的语言代码(未转换)
    needs: HashMap<String, String>, // 插件需要的其他配置信息,由info.json定义
) -> Result<(String, Value), Box<dyn Error>> {
    // 将待翻译文本返回作为结果的一部分
    let translated_text = text.to_string();

    // 调用翻译函数获取翻译结果
    let translation_result = translate_function(text, from, to, detect, needs)?;

    // 解析翻译结果
    if let Some(result) = parse_result(&translation_result) {
        // 返回待翻译文本和翻译结果的元组
        return Ok((translated_text, Value::String(result)));
    } else {
        return Err("Response Parse Error".into());
    }
}

