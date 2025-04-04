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
    "https://learn.microsoft.com/zh-cn/windows/win32/dataxchg/clipboard-messages",
    "https://learn.microsoft.com/zh-cn/windows/win32/dataxchg/clipboard-notifications",
    "https://learn.microsoft.com/zh-cn/windows/win32/dataxchg/data-copy-reference",
    "https://learn.microsoft.com/zh-cn/windows/win32/dataxchg/dynamic-data-exchange-messages",
    "https://learn.microsoft.com/zh-cn/windows/win32/dataxchg/dynamic-data-exchange-notifications",
    "https://learn.microsoft.com/zh-cn/windows/win32/menurc/keyboard-accelerator-notifications",
    "https://learn.microsoft.com/zh-cn/windows/win32/menurc/menu-notifications",
    "https://learn.microsoft.com/zh-cn/windows/win32/dlgbox/dialog-box-notifications",
    "https://learn.microsoft.com/zh-cn/windows/win32/menurc/keyboard-accelerator-messages",
    "https://learn.microsoft.com/zh-cn/windows/win32/menurc/cursor-notifications",
]


def get_common():
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
            if d[0] == "/":
                continue
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
    return list(set(result))


def write_code(result):
    with open(target, "w", encoding="utf-8") as f:
        f.write(f"// 此文件是由 {__file__} 生成的\n")
        f.write("\nuse winapi::um::winuser::*;\n\n")
        f.write(
            """
pub const fn translate_msg(code: u32) -> &'static str {
    match code {
"""
        )
        for name, index in result:
            if name.startswith("WM_"):
                f.write(f'        {index} => "{name}",\n')
        f.write(
            """\
        _ => "UNDEFINED",
    }
}
"""
        )


ime_url = (
    "https://learn.microsoft.com/zh-cn/windows/win32/intl/input-method-manager-messages"
)
result = []


def get_ime():
    print(f"正在请求 {ime_url}")
    response = requests.get(ime_url)
    assert response.status_code == 200
    response.encoding = "utf-8"

    html = response.text
    soup = BeautifulSoup(html, "html.parser")
    lis = soup.find_all("dl")[0]
    strong = lis.find_all("strong")
    return [s.text for s in strong]


dwm_url = "https://learn.microsoft.com/zh-cn/windows/win32/dwm/dwm-messages"


def get_dwm():
    print(f"正在请求 {dwm_url}")
    response = requests.get(dwm_url)
    assert response.status_code == 200
    response.encoding = "utf-8"

    html = response.text
    soup = BeautifulSoup(html, "html.parser")
    tbody = soup.find_all("tbody")
    a = tbody[0].find_all("a")
    strong = []
    for aa in a:
        aa.find("strong")
        strong.append(aa.text.strip())
    return strong


ourl = (
    "https://learn.microsoft.com/zh-cn/windows/win32/gdi/painting-and-drawing-messages"
)


def get_other():
    print(f"正在请求 {ourl}")
    response = requests.get(ourl)
    assert response.status_code == 200
    response.encoding = "utf-8"

    html = response.text
    soup = BeautifulSoup(html, "html.parser")
    lis = soup.find_all("ul")[1]
    a = lis.find_all("a")
    result = []
    for aa in a:
        s = aa.find("strong")
        result.append(s.text)
    return result


if __name__ == "__main__":
    result = get_common()
    result.extend([(i, i) for i in get_ime()])
    result.extend([(i, i) for i in get_dwm()])
    result.extend([(i, i) for i in get_other()])
    cache = {}
    for v, k in result:
        cache[k] = v
    result = [(cache[k], k) for k in cache]

    print(f"共查询到 {len(result)} 条事件标记")
    print("正在生成代码")

    write_code(result)
