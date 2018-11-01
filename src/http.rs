//    use std::fs::File;
//    use std::io::Read;
//    use std::collections::HashMap;
//    let mut buf = Vec::new();
//    File::open("certs/identity.pfx")?
//        .read_to_end(&mut buf)?;
//
//    let pkcs12 = reqwest::Identity::from_pkcs12_der(&buf, "123").unwrap();
//
//
//    let mut buf = Vec::new();
//    File::open("certs/ca2.crt")?
//        .read_to_end(&mut buf)?;
//    let cert = reqwest::Certificate::from_pem(&buf).unwrap();
//    let client = reqwest::Client::builder()
//        .add_root_certificate(cert)
//        .identity(pkcs12)
//        .danger_accept_invalid_hostnames(true)
//        .build().unwrap();
//
//    let fields = LogstashFields{
//        program: "mt4".to_string(),
//    };
//    let record = LogstashLogRecord {
//        message: "Test message".to_string(),
//        source: "testfile.tst".to_string(),
//        offset: 123,
//        fields: fields,
//    };
//    let serialized = serde_json::to_string(&record).unwrap();
//    let res = client.post("https://logstash.fortfs.net:5048")
//        .body(serialized)
//        .send().unwrap();
//
//
