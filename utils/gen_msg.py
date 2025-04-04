import requests
from bs4 import BeautifulSoup
target = "ponsic-winsafe/src/win/gen_by_py/msg_translate.rs"
urls = [
    "https://learn.microsoft.com/zh-cn/windows/win32/inputdev/keyboard-input-notifications",
    "https://learn.microsoft.com/zh-cn/windows/win32/inputdev/keyboard-input-messages",
    "https://learn.microsoft.com/zh-cn/windows/win32/inputdev/mouse-input-notifications",
    "https://learn.microsoft.com/zh-cn/windows/win32/winmsg/window-notifications",
    "https://learn.microsoft.com/zh-cn/windows/win32/winmsg/window-messages",
    "https://learn.microsoft.com/zh-cn/windows/win32/winmsg/timer-notifications",
    "https://learn.microsoft.com/zh-cn/windows/win32/winmsg/hook-notifications",
]
result = []
for url in urls:
    print(f"正在请求 {url}")
    response = requests.get(url)
    assert response.status_code == 200
    response.encoding = "utf-8"

    html = response.text
    soup = BeautifulSoup(html, "html.parser")
    lis = soup.find_all("ul")[1]
    a = lis.find_all("a")
    for i in a:
        d = i.get("href")
        base_url = "/".join(url.split("/")[:-1]) + "/"
        curl = f"{base_url}{d}"
        print(f"正在请求 {curl}")
        response = requests.get(curl)
        assert response.status_code == 200
        response.encoding = "utf-8"
        html = response.text
        soup = BeautifulSoup(html, "html.parser")
        pre = soup.find("pre")
        ls = pre.text.split()[1:]
        result.append((ls[0], ls[1]))

print(f"共查询到 {len(result)} 条事件标记")
print("正在生成代码")
with open(target, "w", encoding="utf-8") as f:
    f.write(f"// 此文件是由 {__file__} 生成的\n")
    f.write(
        """
pub const fn translate_msg(code: u32) -> &'static str {
    match code {
"""
    )
    for name, index in result:
        f.write(f'        {index} => "{name}",\n')
    f.write(
        """\
        _ => "UNDEFINED",
    }
}
"""
    )
