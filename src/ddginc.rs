extern crate hyper;
extern crate select;
extern crate hyper_native_tls;
extern crate url;

use self::hyper::Client;
use self::hyper::header::Connection;
use std::io::Read;
use self::select::document::Document;
use self::select::predicate::{Class,Name};
use self::select::node::Node;
use self::hyper_native_tls::NativeTlsClient;
use self::hyper::net::HttpsConnector;
use self::url::percent_encoding::percent_decode;
use com;

pub fn read_ddg(res: &str, num: i8) -> String {
	let new = com::replace(" ",&res,"+");
	let ddg_ddg_results = Result::get_ddg_results(&new);
	let mut sendhelp="".to_string();
	let mut count=0;
	for ddg_result in ddg_ddg_results.iter() {
		if count<num{
			let s = percent_decode(&ddg_result.link.as_bytes()).decode_utf8().unwrap();
			let t = com::replace(r"/l/\?kh=-1&uddg=",&s,"");
			if t.contains("yahoo.com") == false {
				if count == 0 {
					sendhelp = format!("• <{}>",t);
					count += 1;
				}
				else
				{
					sendhelp = format!("{}\n• <{}>", sendhelp, t);
					count += 1;
				}
			}
		}
		else {
			return sendhelp.to_string();
		}
	}
	return sendhelp.to_string();
}

fn open_ddg(res: &str) -> String {
	let ssl = NativeTlsClient::new().unwrap();
	let connector = HttpsConnector::new(ssl);
	let client = Client::with_connector(connector);
	let res = format!("https://duckduckgo.com/html/?q={}",res);
	let mut response = client.get(&res).
		header(Connection::close()).send().unwrap();
	let mut body = String::new();
	response.read_to_string(&mut body).unwrap();
	return body;
}

struct Result {
	link:	String
}

impl Result {
	fn get_ddg_results(res: &str) -> Vec<Result> {
	let doc: &str = &open_ddg(res);
		Document::from(doc).find(Class("result__extras__url"))
			.map(|node| Result::new(&node)).collect()
	}
	fn new(node: &Node) -> Result {
		let header = node.find(Name("a")).nth(0).unwrap();
		let link = String::from(header.attr("href").unwrap());
		Result { link: link }
	}
}

