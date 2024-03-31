use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use urlencoding::encode;
use std::io::Write;
use std::fs::File;
use std::fs::read_to_string;
use std::fs::remove_file;
use std::thread::sleep;
use core::time::Duration;


#[no_mangle]
pub fn translate(
    text: &str, // 待翻译文本
    from: &str, // 源语言
    to: &str,   // 目标语言
    // (pot会根据info.json 中的 language 字段传入插件需要的语言代码，无需再次转换)
    detect: &str, // 检测到的语言 (若使用 detect, 需要手动转换)
    needs: HashMap<String, String>, // 插件需要的其他参数,由info.json定义
) -> Result<Value, Box<dyn Error>> {

     // 读入翻译后中文zh.txt
     let path_zh = match needs.get("path_zh") {
        Some(path) => path.to_string(),
        None => return Err("读取zh文件错误".into()),
    };
    // 清除残留文本
    if std::path::Path::new(&path_zh).exists() {
        remove_file(&path_zh)?;
    }


    // 写出英语文本到text.txt
     let file_path = match needs.get("path_text") {
        Some(path) => path.to_string(),
        None => return Err("写出text文件错误".into()),
    };
    // 写出操作
    let mut file = File::create(file_path)?;
    file.write_all(text.as_bytes())?;
    
   
    
    // 检测文本、读取文本、清除文本
    let mut loops = 150; // 计数器变量
    loop {
        if std::path::Path::new(&path_zh).exists() {
            content = read_to_string(&path_zh)?;
            break;
        }
        sleep(Duration::from_millis(100));
        loops += 1;
        if loops >= max_loops { // 达到指定次数后跳出循环
            return Err("超过最大循环次数".into());
        }
    }
    
    Ok(Value::String(content))
}
