use crate::domain::{Project, Scene};

/// Build prompt for generating a full project script with multiple scenes
pub fn build_project_script_prompt(project: &Project, user_prompt: &str) -> String {
    let description = project.description.as_deref().unwrap_or("No description");

    format!(
        r#"You are a video tutorial script writer. Generate a script for:

Project: {title}
Description: {description}

User Request: {user_prompt}

Generate a JSON response with this exact structure:
{{
  "scenes": [
    {{
      "order": 1,
      "title": "Scene title",
      "script_text": "Narrator dialog for this scene...",
      "description": "Visual description of what happens...",
      "overlay_text": "On-screen text if any"
    }}
  ]
}}

Guidelines:
- Create 3-7 scenes for a 5-minute video
- Each scene should have 30-90 seconds of dialog
- Include clear visual descriptions
- Suggest relevant overlay text for key points
- Return ONLY valid JSON, no markdown code blocks"#,
        title = project.title,
        description = description,
        user_prompt = user_prompt
    )
}

/// Build prompt for generating/refining a single scene script
pub fn build_scene_script_prompt(scene: &Scene, project: &Project, user_prompt: &str) -> String {
    let current_script = scene.script_text.as_deref().unwrap_or("No existing script");

    format!(
        r#"Refine or generate a scene script:

Project: {project_title}
Scene: {scene_title}
Current Script: {current_script}

User Request: {user_prompt}

Return ONLY the refined script text. No JSON wrapper, no explanation.
Just the narrator dialog that should be spoken in this scene."#,
        project_title = project.title,
        scene_title = scene.title,
        current_script = current_script,
        user_prompt = user_prompt
    )
}
