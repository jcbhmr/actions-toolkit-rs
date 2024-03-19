use std::{
    cell::{OnceCell, RefCell},
    collections::HashMap,
    env,
    error::Error,
    fs,
    io::Write,
};

pub const SUMMARY_ENV_VAR: &str = "GITHUB_STEP_SUMMARY";
pub const SUMMARY_DOCS_URL: &str = "https://docs.github.com/actions/using-workflows/workflow-commands-for-github-actions#adding-a-job-summary";

pub enum SummaryTableRowItem {
    SummaryTableCell(SummaryTableCell),
    String(String),
}
pub type SummaryTableRow = Vec<SummaryTableRowItem>;

pub struct SummaryTableCell {
    pub data: String,
    pub header: Option<bool>,
    pub colspan: Option<String>,
    pub rowspan: Option<String>,
}

pub struct SummaryImageOptions {
    pub width: Option<String>,
    pub height: Option<String>,
}

pub struct SummaryWriteOptions {
    pub overwrite: Option<bool>,
}

struct Summary {
    buffer: String,
    file_path: Option<String>,
}

impl Summary {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            file_path: None,
        }
    }

    fn file_path(&mut self) -> Result<String, Box<dyn Error>> {
        if let Some(file_path) = self.file_path {
            if !file_path.is_empty() {
                return Ok(file_path.into());
            }
        }

        let path_from_env = env::var(SUMMARY_ENV_VAR).unwrap_or_default();
        if path_from_env.is_empty() {
            return Err("GITHUB_STEP_SUMMARY not set".into());
        }

        let result = path_from_env.clone();
        self.file_path = Some(path_from_env);
        Ok(result)
    }

    fn wrap(
        &self,
        tag: &str,
        content: Option<String>,
        attrs: Option<HashMap<String, String>>,
    ) -> String {
        let mut result = format!("<{tag}");
        if let Some(attrs) = attrs {
            for (key, value) in attrs {
                result.push_str(&format!(" {key}=\"{value}\""));
            }
        }
        result.push('>');
        if let Some(content) = content {
            result.push_str(&content);
        }
        result.push_str(&format!("</{tag}>"));
        result
    }

    pub fn write(
        &mut self,
        options: Option<SummaryWriteOptions>,
    ) -> Result<&mut Self, Box<dyn Error>> {
        let overwrite = options
            .map(|o| o.overwrite.unwrap_or_default())
            .unwrap_or_default();
        let file_path = self.file_path()?;
        if overwrite {
            fs::write(file_path, self.buffer.clone())?;
        } else {
            let mut file = fs::OpenOptions::new().append(true).open(file_path)?;
            file.write_all(self.buffer.as_bytes())?;
        }
        self.empty_buffer();
        Ok(self)
    }

    pub fn clear(&mut self) -> Result<&mut Self, Box<dyn Error>> {
        self.empty_buffer();
        self.write(Some(SummaryWriteOptions {
            overwrite: Some(true),
        }))?;
        Ok(self)
    }

    pub fn stringify(&self) -> &str {
        self.buffer.as_str()
    }

    pub fn is_empty_buffer(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn empty_buffer(&mut self) -> &mut Self {
        self.buffer = String::new();
        self
    }

    pub fn add_raw(&mut self, text: &str, add_eol: Option<bool>) -> &mut Self {
        self.buffer.push_str(text);
        if add_eol.unwrap_or_default() {
            self.add_eol();
        }
        self
    }

    pub fn add_eol(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        let eol = "\r\n";
        #[cfg(not(target_os = "windows"))]
        let eol = "\n";
        self.buffer.push_str(eol);
        self
    }

    pub fn add_code_block(&mut self, code: &str, lang: Option<String>) -> &mut Self {
        let attrs = lang.map(|lang| HashMap::from([("lang".into(), lang)]));
        let element = self.wrap(
            "pre",
            Some(self.wrap("code", Some(code.into()), None)),
            attrs,
        );
        self.add_raw(&element, Some(true));
        self
    }

    pub fn add_list(&mut self, items: &[String], ordered: Option<bool>) -> &mut Self {
        let tag = if ordered.unwrap_or_default() {
            "ol"
        } else {
            "ul"
        };
        let mut list = String::new();
        for item in items {
            list.push_str(&format!("<li>{item}</li>"));
        }
        self.add_raw(&self.wrap(tag, Some(list), None), Some(true));
        self
    }

    pub fn add_table(&mut self, rows: Vec<SummaryTableRow>) -> &mut Self {
        let mut table = String::new();
        for row in rows {
            let mut tr = String::new();
            for item in row {
                let td = match item {
                    SummaryTableRowItem::SummaryTableCell(cell) => {
                        let mut attrs: HashMap<String, String> = HashMap::new();
                        if let Some(header) = cell.header {
                            attrs.insert(
                                "th".into(),
                                if header {
                                    "true".into()
                                } else {
                                    "false".into()
                                },
                            );
                        }
                        if let Some(colspan) = cell.colspan {
                            attrs.insert("colspan".into(), colspan);
                        }
                        if let Some(rowspan) = cell.rowspan {
                            attrs.insert("rowspan".into(), rowspan);
                        }
                        self.wrap("td", Some(cell.data), Some(attrs))
                    }
                    SummaryTableRowItem::String(data) => self.wrap("td", Some(data), None),
                };
                tr.push_str(&td);
            }
            table.push_str(&self.wrap("tr", Some(tr), None));
        }
        self.add_raw(&self.wrap("table", Some(table), None), Some(true));
        self
    }

    pub fn add_details(&mut self, label: &str, content: &str) -> &mut Self {
        let element = self.wrap(
            "details",
            Some(self.wrap("summary", Some(label.into()), None) + content),
            None,
        );
        self.add_raw(&element, Some(true));
        self
    }

    pub fn add_image(
        &mut self,
        src: &str,
        alt: &str,
        options: Option<SummaryImageOptions>,
    ) -> &mut Self {
        let mut attrs = HashMap::from([
            ("src".to_string(), src.to_string()),
            ("alt".to_string(), alt.to_string()),
        ]);
        if let Some(options) = options {
            if let Some(width) = options.width {
                attrs.insert("width".into(), width.into());
            }
            if let Some(height) = options.height {
                attrs.insert("height".into(), height.into());
            }
        }
        let element = self.wrap("img", None, Some(attrs));
        self.add_raw(&element, Some(true));
        self
    }

    pub fn add_heading(&mut self, text: &str, level: Option<u8>) -> &mut Self {
        let tag = match level {
            Some(level) => format!("h{level}"),
            None => "h1".to_string(),
        };
        let tag = match tag.as_str() {
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => &tag,
            _ => "h1",
        };
        let element = self.wrap(tag, Some(text.into()), None);
        self.add_raw(&element, Some(true));
        self
    }

    pub fn add_separator(&mut self) -> &mut Self {
        let element = self.wrap("hr", None, None);
        self.add_raw(&element, Some(true));
        self
    }

    pub fn add_break(&mut self) -> &mut Self {
        let element = self.wrap("br", None, None);
        self.add_raw(&element, Some(true));
        self
    }

    pub fn add_quote(&mut self, text: &str, cite: Option<&str>) -> &mut Self {
        let attrs = cite.map(|cite| HashMap::from([("cite".into(), cite.into())]));
        let element = self.wrap("blockquote", Some(text.into()), attrs);
        self.add_raw(&element, Some(true));
        self
    }

    pub fn add_link(&mut self, text: &str, href: &str) -> &mut Self {
        let element = self.wrap(
            "a",
            Some(text.into()),
            Some(HashMap::from([("href".into(), href.into())])),
        );
        self.add_raw(&element, Some(true));
        self
    }
}

thread_local! {
    pub static SUMMARY: RefCell<Summary> = RefCell::new(Summary::new());
    pub static MARKDOWN_SUMMARY: () = ();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_instance() {
        SUMMARY.with(|s| {
            let mut summary = s.borrow_mut();
            summary.add_raw("test", Some(true));
            assert_eq!(summary.stringify(), "test\n");
        });
    }
}
