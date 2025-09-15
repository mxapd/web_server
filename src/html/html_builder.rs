pub struct HtmlBuilder {
    head: String,
    body: String,
}

impl HtmlBuilder {
    pub fn new() -> Self {
        Self {
            head: String::new(),
            body: String::new(),
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.head
            .push_str(format!("<title>{}</title>\n", title).as_str());
        self
    }

    pub fn add_h1(mut self, heading: &str) -> Self {
        self.body.push_str(format!("<h1>{}</h1>", heading).as_str());
        self
    }

    pub fn add_h2(mut self, heading: &str) -> Self {
        self.body.push_str(format!("<h2>{}</h2>", heading).as_str());
        self
    }

    pub fn add_h3(mut self, heading: &str) -> Self {
        self.body.push_str(format!("<h3>{}</h3>", heading).as_str());
        self
    }

    pub fn add_h4(mut self, heading: &str) -> Self {
        self.body.push_str(format!("<h4>{}</h4>", heading).as_str());
        self
    }

    pub fn add_h5(mut self, heading: &str) -> Self {
        self.body.push_str(format!("<h5>{}</h5>", heading).as_str());
        self
    }

    pub fn add_h6(mut self, heading: &str) -> Self {
        self.body.push_str(format!("<h6>{}</h6>", heading).as_str());
        self
    }

    pub fn add_p(mut self, p: &str) -> Self {
        self.body.push_str(format!("<p>{}</p>", p).as_str());
        self
    }

    pub fn add_hr(mut self) -> Self {
        self.body.push_str("<hr>");
        self
    }

    pub fn add_br(mut self) -> Self {
        self.body.push_str("<br>");
        self
    }

    pub fn build(self) -> String {
        format!(
            "<!DOCTYPE html>\n
                <html>\n
                <head>\n
                    {}\n
                </head>\n
                <body>\n
                    {}\n
                </body>\n
                </html>",
            self.head, self.body
        )
    }
}
