use std::io;
use super::*;
use std::fs::File;
use std::io::Read;
use reqwest::Url;
use config::SslConfig;


#[derive(Clone )]
pub struct HttpsSender{
    url: Url,
    client: reqwest::Client,
}
impl HttpsSender {
    pub fn new(url: &str, ssl: Option<SslConfig>)
            -> Result<HttpsSender, io::Error> {
        let client = match ssl {
            Some(ssl) => {
                let mut buf = Vec::new();
                File::open(ssl.client_pem_file)?
                    .read_to_end(&mut buf)?;
                let identity = reqwest::Identity::from_pem(&buf).unwrap();
                let mut buf = Vec::new();
                File::open(ssl.client_ca_chain_file)?
                    .read_to_end(&mut buf)?;
                let ca = reqwest::Certificate::from_pem(&buf).unwrap();
                let client = reqwest::Client::builder()
                    .use_rustls_tls()
                    .add_root_certificate(ca)
                    .identity(identity)
                    .build().unwrap();
                client
            },
            None => {
                let client = reqwest::Client::builder()
                    .build().unwrap();
                client
            }
        };

        let url = Url::parse(url).unwrap();

        Ok(HttpsSender{
            url,
            client,
        })

    }
}
fn new_record(file_name: &str, line: &str, offset: u64)
    -> LogstashLogRecord
{
    let fields = LogstashFields{
        program: "mt4".to_string(),
    };
    let record = LogstashLogRecord {
        message: line.to_owned(),
        source: file_name.to_owned(),
        offset: offset,
        fields: fields,
    };
    return record
}

impl Exporter for HttpsSender {
    fn send(&self, file_name: &str, lines: Vec<(String, u64)>) -> Result<(), String>{
        let f = move || -> Result<(), io::Error>{
            let mut events = Vec::new();
            for (line, offset) in lines{
                let event = new_record(file_name, &line, offset);
                events.push(event);
            }
            let strings: String = events
                .into_iter()
                .map(|x| serde_json::to_string(&x).unwrap())
                .map(|x| x+"\n")
                .fold("".to_string(), |acc, s| acc + &s);

            let res = self.client.post(self.url.to_owned())
                .body(strings)
                .send().unwrap();

            if res.status().as_u16() != 200 {
                panic!("{:?}", res.status());
            }

            Ok(())
        };
        if let Err(e) = f() {
            match e.kind() {
                io::ErrorKind::BrokenPipe => {return Err("BrokenPipe".to_string())},
                _ => panic!(e),
            }
        }
        Ok(())
    }
}


#[derive(Serialize, Clone)]
struct LogstashLogRecord {
    message: String,
    source: String,
    fields: LogstashFields,
    offset: u64,
}
#[derive(Serialize, Clone)]
struct LogstashFields{
   program: String,
}

