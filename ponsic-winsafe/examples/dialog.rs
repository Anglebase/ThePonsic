#[cfg(target_os = "windows")]
use ponsic_winsafe::{SystemError, dialog::*};

#[cfg(not(target_os = "windows"))]
fn main() {}

#[cfg(target_os = "windows")]
fn main() -> Result<(), SystemError> {
    let res = Dialog::new(DialogType::Information)
        .title("Ponsic 对话框示例")
        .message("这是 Ponsic 对话框的内容，可以是用于提示用户的任何信息！")
        .buttons(Button::OkCancel)
        .block();

    match res {
        DialogResult::Ok => println!("用户点击了确定按钮！"),
        DialogResult::Cancel => println!("用户点击了取消按钮！"),
        _ => unreachable!(),
    }

    Ok(())
}
