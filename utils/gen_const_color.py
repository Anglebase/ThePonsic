import requests
from bs4 import BeautifulSoup

url = "https://developer.mozilla.org/en-US/docs/Web/CSS/named-color"

print(f"正在请求 {url}")
response = requests.get(url)

assert response.status_code == 200
response.encoding = "utf-8"

soup = BeautifulSoup(response.text, "html.parser")
tbody = soup.find_all("tbody")
tr = tbody[0].find_all("tr")
tr.extend(tbody[1].find_all("tr"))

print(f"统计到{len(tr)}个颜色预定义值")

result = []
for t in tr:
    code = t.find_all("code")
    if len(code) != 2:
        continue
    name = code[0].text.strip()
    hex_code = code[1].text.strip()
    result.append((name, hex_code[1:]))

const_file = r"ponsic-color\src\const_color.rs"
with open(const_file, "w", encoding="utf-8") as f:
    f.write(f"// 此文件由 {__file__.split('\\')[-1]} 生成的\n")
    f.write("use crate::Color;\n\n")
    f.write("impl Color {\n")
    for name, hex_code in result:
        f.write(
            f"    pub const {name.upper()}: Self = Self::new(0x{hex_code[0:2]}, 0x{hex_code[2:4]}, 0x{hex_code[4:]});\n"
        )
    f.write("}\n")
