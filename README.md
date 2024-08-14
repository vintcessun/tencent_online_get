# 一个封装好的库用于获取腾讯在线文档的下载地址

## cookie获取方法

先打开腾讯文档并登录后打开F12并输入

```javascript
function get_tag(tag){var key=tag+"=";var str=document.cookie.slice(document.cookie.indexOf(key)+key.length);return str.slice(0,str.indexOf(";"))}"uid="+get_tag("uid")+";uid_key="+get_tag("uid_key");
```

然后复制输出内容

![1723622466877](.\console.png)

这个就是cookie了，删不删除两边的引号无所谓（内部做了替换）

## 调用方法

```rust
use tencent_online_get::{OnlineOpen,ReturnFile}
let cookie = ""
let source_url = ""
let mut f:OnlineOpen = tencent_online_get::OnlineOpen::new(cookie);
let ret = f.get_url(source_url).unwrap();//这里得到了ReturnFile格式的地址
let ret = DownloadFile::new(ret);
ret.download().unwrap();
```

对于结果的解析可以定义一个这个

```rust
struct DownloadFile{
	url: String,
	filename: String,
}

impl DownloadFile{
	fn new(f: ReturnFile)->Self{
		Self{url:f.url,filename:f.filename}
	}
	fn download(&self)->Result<()>{
		println!("下载 {:?} => {:?}",&self.url,&self.filename);
		Ok(())
	}
}
```

DownloadFile是参照ReturnFile定义的

```rust
#[derive(Debug, Clone)]
pub struct ReturnFile{
    pub url: String,
    pub filename: String,
}
```

