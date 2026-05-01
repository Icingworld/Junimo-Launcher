//! 游戏启动命令模块

use tauri::command;


/// 启动游戏
/// 
/// # 参数
/// 
/// - `game_path`: 游戏路径
/// 
/// # 返回
/// 
/// - `Result<(), String>`: 启动结果
/// 
/// # 错误
/// 
/// - `String`: 错误信息
/// 
/// # 示例
/// 
/// ```rust
/// let result = start_game("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Stardew Valley".to_string());
/// assert!(result.is_ok());
/// ```
/// 
#[command]
pub async fn start_game() -> Result<(), String> {
    let url = "steam://run/413150";
    // 使用 opener 打开游戏
    opener::open(url).map_err(|e| e.to_string())?;

    Ok(())
}
