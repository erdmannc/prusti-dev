#[derive(Debug, Clone)]
pub struct ClassName {
    full_class_name_dot: String,
    full_class_name_slash: String,
    class_name: String,
}

impl ClassName {
    pub fn new(full_class_name: &str) -> Self {
        let full_class_name_dot = full_class_name.to_string().replace("/", ".");
        let full_class_name_slash = full_class_name_dot.replace(".", "/");
        let class_name = full_class_name_slash.split("/").last().unwrap().to_string();

        ClassName {
            full_class_name_dot,
            full_class_name_slash,
            class_name,
        }
    }

    pub fn name(&self) -> String {
        self.class_name.clone()
    }

    pub fn rust_name(&self) -> String {
        self.class_name.replace("$", "_object")
    }

    pub fn path(&self) -> String {
        self.full_class_name_slash.clone()
    }

    pub fn full_components(&self) -> Vec<&str> {
        self.full_class_name_slash.split("/").collect()
    }

    pub fn full_name(&self) -> String {
        self.full_class_name_dot.clone()
    }
}
