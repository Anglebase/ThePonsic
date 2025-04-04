# 此文件用于生成Rust生成错误代码信息的代码

import requests
from bs4 import BeautifulSoup

lang = "zh-cn"
target = "ponsic-winsafe/src/win/gen_by_py/error_translate.rs"
urls = [
    f"https://learn.microsoft.com/{lang}/windows/win32/debug/system-error-codes--0-499-",
    f"https://learn.microsoft.com/{lang}/windows/win32/debug/system-error-codes--500-999-",
    f"https://learn.microsoft.com/{lang}/windows/win32/debug/system-error-codes--1000-1299-",
    f"https://learn.microsoft.com/{lang}/windows/win32/debug/system-error-codes--1300-1699-",
    f"https://learn.microsoft.com/{lang}/windows/win32/debug/system-error-codes--1700-3999-",
    f"https://learn.microsoft.com/{lang}/windows/win32/debug/system-error-codes--4000-5999-",
    f"https://learn.microsoft.com/{lang}/windows/win32/debug/system-error-codes--6000-8199-",
    f"https://learn.microsoft.com/{lang}/windows/win32/debug/system-error-codes--8200-8999-",
    f"https://learn.microsoft.com/{lang}/windows/win32/debug/system-error-codes--9000-11999-",
    f"https://learn.microsoft.com/{lang}/windows/win32/debug/system-error-codes--12000-15999-",
]
result = []

for url in urls:
    print(f"正在请求 {url}")
    response = requests.get(url)
    assert response.status_code == 200
    # print(response.encoding)
    response.encoding = "utf-8"
    html = response.text

    soup = BeautifulSoup(html, "html.parser")
    dl = soup.find("dl")
    dds = dl.find_all("dd")

    for dd in dds:
        dts = dd.find_all("dt")
        result.append((dts[0].text.strip().split()[0], dts[1].text.strip()))

print("查询完毕")
print("正在生成 Rust 代码")

with open(target, "w", encoding="utf-8") as f:
    f.write(f"// 此文件是由 {__file__} 生成的\n")
    f.write(
        """\
pub const fn translate_error(code: u32) -> &'static str {
    match code {
"""
    )
    for code, msg in result:
        f.write(f"        {code} => \"{msg.split('\n')[0]}\",\n")
    f.write(
        """\
        _ => "未知错误",
    }
}
            """
    )

print("生成完毕")