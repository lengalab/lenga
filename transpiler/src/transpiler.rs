use language::language::{Language, c::C};

pub enum SuportedLanguage {
    C(C),
    // Rust(Rust),
}

impl SuportedLanguage {
    #[allow(dead_code)]
    fn file_extension(&self) -> String {
        match self {
            SuportedLanguage::C(c) => c.file_extension(),
        }
    }

    #[allow(dead_code)]
    fn name(&self) -> String {
        match self {
            SuportedLanguage::C(c) => c.name(),
        }
    }

    pub fn text_to_nodes(&self, content: &str) -> Result<Vec<u8>, String> {
        match self {
            SuportedLanguage::C(language) => text_to_nodes(language, content),
        }
    }

    pub fn text_to_text(&self, content: &str) -> Result<String, String> {
        match self {
            SuportedLanguage::C(language) => text_to_text(language, content),
        }
    }

    pub fn nodes_to_text(&self, nodes: Vec<u8>) -> Result<String, String> {
        match self {
            SuportedLanguage::C(language) => nodes_to_text(language, nodes),
        }
    }
}

fn text_to_text<T: Language>(language: &T, content: &str) -> Result<String, String> {
    let nodes = language.parse_text(content)?;
    let output = language.write_to_text(nodes).unwrap();
    Ok(output)
}

fn text_to_nodes<T: Language>(language: &T, content: &str) -> Result<Vec<u8>, String> {
    let code_objects = language.parse_text(content)?;
    let output = language.write_to_nodes(code_objects).unwrap();
    Ok(output)
}

pub fn nodes_to_text<T: Language>(language: &T, nodes: Vec<u8>) -> Result<String, String> {
    let code_objects = language.parse_nodes(nodes)?;
    let output = language.write_to_text(code_objects)?;
    Ok(output)
}

pub struct Transpiler {}

impl Default for Transpiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Transpiler {
    pub fn new() -> Self {
        Self {}
    }

    fn get_language(&self, file_extension: &str) -> Option<SuportedLanguage> {
        let cext = C::new().file_extension();
        let lang: Option<SuportedLanguage> = match file_extension {
            ext if ext == cext => Some(SuportedLanguage::C(C::new())),
            _ => None,
        };
        lang
    }

    pub fn text_to_nodes(&self, content: &str, file_extension: &str) -> Result<Vec<u8>, String> {
        let language = self
            .get_language(file_extension)
            .ok_or_else(|| format!("Language with extension '{file_extension}' not registered"))
            .unwrap();
        let nodes = language.text_to_nodes(content)?;
        Ok(nodes)
    }

    pub fn text_to_text(&self, content: &str, file_extension: &str) -> Result<String, String> {
        let language = self
            .get_language(file_extension)
            .ok_or_else(|| format!("Language with extension '{file_extension}' not registered"))
            .unwrap();
        let output = language.text_to_text(content).unwrap();
        Ok(output)
    }

    pub fn nodes_to_text(&self, nodes: Vec<u8>, file_extension: &str) -> Result<String, String> {
        let language = self
            .get_language(file_extension)
            .ok_or_else(|| format!("Language with extension '{file_extension}' not registered"))
            .unwrap();
        let output = language.nodes_to_text(nodes).unwrap();
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpile_c_text_to_nodes_and_back() {
        let c_code = r#"
        #include <stdio.h>

int first(int a, int b, int c)
{
    int first_parameter = a;
    return first_parameter;
}

int first(int a, int b, int c);

int main()
{
    int a = 1;
    int b = 2;
    int c = 3;
    {
        int a = b + c;
        printf("inner a: %d\n", a);
    }

    int result = first(a, b, c);
    printf("result: %d\n", result);
}
        "#;

        let transpiler = Transpiler::new();
        let nodes = transpiler
            .text_to_nodes(c_code, "c")
            .expect("Failed to convert C code to nodes");
        let output_code = transpiler
            .nodes_to_text(nodes, "c")
            .expect("Failed to convert nodes back to C code");

        assert_eq!(
            c_code.replace(" ", "").replace("\n", ""),
            output_code.replace(" ", "").replace("\n", "")
        );
    }
}
