use std::collections::HashMap;
use reqwest::blocking::{Client,RequestBuilder};
use anyhow::{Result, anyhow};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ReturnFile{
    pub url: String,
    pub filename: String,
}

pub struct OnlineOpen{
    cookie: String,
    url: String,
    id: String,
}
/// 获取 cookie 方法：
/// 打开任意一个在线文档，按F12并且输入以下JavaScript：
/// function get_tag(tag){var key=tag+"=";var str=document.cookie.slice(document.cookie.indexOf(key)+key.length);return str.slice(0,str.indexOf(";"))}"uid="+get_tag("uid")+";uid_key="+get_tag("uid_key");
/// 然后控制台输出就是 cookie 了
impl OnlineOpen{
    pub fn new(cookie: &str)->Self{
        Self{cookie:cookie.to_string().replace(['\'','\"'],""),url:"".to_string(),id:"".to_string()}
    }
    pub fn renew(&mut self, cookie:&str){
        self.cookie = cookie.to_string().replace(['\'','\"'],"");
    }
    pub fn get_url(&mut self, url: &str)->Result<ReturnFile>{
        self.url = url.to_string();
        let left = match url.rfind('/'){
            Some(x)=>{x+1}
            None=>{0}
        };
        let right = url.rfind('?').unwrap_or(url.len()-1);
        let id = &url[left..right];
        self.id = id.to_string();
        let docid = self.get_docid()?;
        let operationid = self.get_operationid(&docid)?;
        //println!("{}",&operationid);
        self.get_exact_url(&operationid)
    }
}

impl OnlineOpen{
    fn get_docid(&self)-> Result<String>{
        let url = format!("https://docs.qq.com/dop-api/opendoc?id={}&normal=1&outformat=1&startrow=0&endrow=60&wb=1&nowb=0&callback=clientVarsCallback&xsrf=&t={}"
            ,&self.id,get_time::time_x());
        let ret = self.get_header(&url).send()?.text()?;
        let ret = self.read_callback(ret)?;
        Ok(ret["clientVars"]["globalPadId"].to_string().replace('\"',""))
    }
    fn get_operationid(&self, docid: &String)->Result<String>{
        let url = "https://docs.qq.com/v1/export/export_office";
        let mut json = HashMap::new();
        json.insert("exportType", "0");
        json.insert("switches", "%7B%22embedFonts%22%3Afalse%7D");
        json.insert("exportSource", "client");
        json.insert("docId", docid);
        json.insert("version", "2");

        //let json = json!({"exportType": 0, "switches": {"embedFonts": "false"}, "exportSource": "client", "docId": &docid, "version": 2});
        let ret = self.post_header(url).form(&json).send()?.text()?;
        let ret:Value = serde_json::from_str(&ret)?;
        if ret["ret"] == 0{
            Ok(ret["operationId"].to_string().replace('\"',""))
        }
        else if ret["ret"] == 403{
            Err(anyhow!("{}",ret["msg"].to_string()))
        }
        else{
            Err(anyhow!("未知的状态码 {}",ret.to_string()))
        }
    }
    fn get_exact_url(&self, operationid: &String)->Result<ReturnFile>{
        let url = format!("https://docs.qq.com/v1/export/query_progress?operationId={}",&operationid);
        loop{
            let ret = self.get_header(&url).send()?.text()?;
            let ret:Value = serde_json::from_str(&ret)?;
            if ret["ret"] == 0{
                if ret["status"] == "Done"{//其他还有 "Processing"
                    break Ok(ReturnFile{url: ret["file_url"].to_string().replace('\"',""), filename: ret["file_name"].to_string().replace('\"',"")});
                }
            }
            else{
                break Err(anyhow!("未知的状态码 {}",ret.to_string()));
            }
        }
    }
    fn read_callback(&self, text: String)->Result<Value>{
        let left = "clientVarsCallback(\"".len();
        let right = text.rfind("\")").unwrap_or(text.len()-1);
        let content = &text[left..right];
        let json = content.replace("&#34;", "\"")
        .replace("\\\\\"","\\\\\'");
        Ok(serde_json::from_str(&json)?)
    }
    fn get_header(&self, url: &str)->RequestBuilder{
        let client = Client::new();
        client.get(url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36")
            .header("Referer", self.url.clone())
            .header("cookie", self.cookie.clone())
    }
    fn post_header(&self, url: &str)->RequestBuilder{
        let client = Client::new();
        client.post(url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36")
            .header("Referer", self.url.clone())
            .header("cookie", self.cookie.clone())
    }
}

mod get_time{
    use chrono::prelude::Local;

    pub fn time_x()-> i64{
        let now = Local::now();
        now.timestamp_millis()
    }
}