//! Prompt building functions for script generation

use orchestrator_domain::Project;

/// Build the prompt for generating a project script
pub fn build_project_script_prompt(project: &Project, user_prompt: &str) -> String {
    format!(
        r#"You are a professional video script writer. Generate a detailed video script for the following project.

Project: {} ({})
User Request: {}

Return your response as a JSON array of scenes. Each scene should have:
- title: A short title for the scene
- description: A brief description of what happens
- script_text: The actual script/narration for the scene
- sort_order: The order of the scene (starting from 1)

Return ONLY valid JSON, no markdown code blocks or other text.

Example format:
[
  {{"title": "Introduction", "description": "Opening hook", "script_text": "Welcome to...", "sort_order": 1}},
  {{"title": "Main Content", "description": "Core explanation", "script_text": "Let's explore...", "sort_order": 2}}
]"#,
        project.title, project.slug, user_prompt
    )
}

/// Build the prompt for refining a scene script
pub fn build_scene_script_prompt(scene_title: &str, current_script: Option<&str>, user_prompt: &str) -> String {
    let current = current_script.unwrap_or("(no existing script)");
    format!(
        r#"You are a professional video script writer. Refine or create the script for this scene.

Scene: {}
Current Script: {}
User Request: {}

Return ONLY the refined script text, no JSON or other formatting."#,
        scene_title, current, user_prompt
    )
}
