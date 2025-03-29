use crate::analyzer::EngineData;
use anyhow::{Result, anyhow};
use std::fs;
use std::path::Path;
use tera::{Context, Tera};

pub struct TemplateEngine {
    pub engine_data: EngineData,
    tera: Tera,
    output_dir: String,
}

impl TemplateEngine {
    pub fn new(engine_data: EngineData, output_dir: &str) -> Result<Self> {
        // Initialize Tera with templates
        let mut tera = Tera::default();

        // Load templates
        tera.add_raw_template(
            "entity_class.html",
            include_str!("../../templates/entity_class.html"),
        )
        .map_err(|e| anyhow!("Failed to load entity template: {}", e))?;

        // Create output directory if it doesn't exist
        let output_path = Path::new(output_dir);
        if !output_path.exists() {
            fs::create_dir_all(output_path)
                .map_err(|e| anyhow!("Failed to create output directory: {}", e))?;
        }

        Ok(Self {
            engine_data,
            tera,
            output_dir: output_dir.to_string(),
        })
    }

    pub fn generate_templates(&self) -> Result<()> {
        self.generate_entity_classes()?;
        // In the future, add more generation methods:
        // self.generate_queries_class()?;

        Ok(())
    }

    fn generate_entity_classes(&self) -> Result<()> {
        for table in &self.engine_data.tables {
            let mut context = Context::new();

            // Convert table name to PascalCase for class name
            let class_name = self.to_pascal_case(&table.name);
            context.insert("class_name", &class_name);

            // Add columns to context
            context.insert("columns", &table.columns);

            // Render the template
            let rendered = self
                .tera
                .render("entity_class.html", &context)
                .map_err(|e| {
                    anyhow!(
                        "Failed to render entity template for {}: {:?}",
                        table.name,
                        e
                    )
                })?;

            // Write the rendered template to a file
            let file_path = Path::new(&self.output_dir).join(format!("{}.php", class_name));
            fs::write(&file_path, rendered).map_err(|e| {
                anyhow!("Failed to write entity file {}: {}", file_path.display(), e)
            })?;

            println!(
                "Generated entity class for table {} at {}",
                table.name,
                file_path.display()
            );
        }

        Ok(())
    }

    // Helper to convert snake_case to PascalCase
    fn to_pascal_case(&self, s: &str) -> String {
        let mut result = String::new();
        let mut capitalize = true;

        for c in s.chars() {
            if c == '_' {
                capitalize = true;
            } else if capitalize {
                result.push(c.to_ascii_uppercase());
                capitalize = false;
            } else {
                result.push(c);
            }
        }

        result
    }
}
