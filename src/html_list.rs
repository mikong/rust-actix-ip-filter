use actix_ip_filter::models::IpAddress;

pub struct HtmlList;

impl HtmlList {
    pub fn new(results: Vec<IpAddress>) -> String {
        let mut list = String::from("<ul>\n");
        for ip_address in results {
            let item = format!("<li>{}</li>\n", ip_address.ip);
            list.push_str(&item);
        }
        list.push_str("</ul>");

        format!("<html>\n<body>\n{}\n</body>\n</html>", list)
    }
}
