mod get_file;

pub use get_file::OnlineOpen;
pub use get_file::ReturnFile;

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

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

    #[test]
    fn it_works() {
        let cookie = "";
        let source_url = "";
        let mut f = get_file::OnlineOpen::new(cookie);
        let ret = f.get_url(source_url).unwrap();
        let ret = DownloadFile::new(ret);
        ret.download().unwrap();
        //println!("{:?}",ret);
    }
}
