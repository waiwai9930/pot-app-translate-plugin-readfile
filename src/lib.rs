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
) -> Result<(String, serde_json::Value), Box<dyn Error>> {
    // 将待翻译文本返回作为结果的一部分
    // 返回用Value包裹的String
    return Ok((text.to_string(), Value::String(text.to_string())));
}
