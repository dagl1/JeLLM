// src/main.rs
use tokio; 

// 1. THE UI DECLARATION (Must be at the top of the file)
slint::slint! {
    import { Button, LineEdit, TextEdit } from "std-widgets.slint";

    export component AppWindow inherits Window {
        title: "LLM Harness UI";
        min-width: 600px;
        min-height: 500px;

        // Custom property that Rust can mutate dynamically
        in-out property <string> log-text: "System: Ready for prompts...";

        // Callbacks to emit interactions back to Rust
        callback submit-prompt(string);

        VerticalLayout {
            padding: 20px;
            spacing: 15px;

            Text {
                text: "LLM Output Log:";
                font-size: 14px;
            }

            // Displays your model's responses
            TextEdit {
                text: root.log-text;
                read-only: true;
            }

            Text {
                text: "Enter Prompt:";
                font-size: 14px;
            }

            // Input field for user prompts
            input_field := LineEdit {
                placeholder-text: "Type your query here...";
            }

            // Submit Button
            Button {
                text: "Send to Harness";
                primary: true;
                clicked => {
                    root.submit-prompt(input_field.text);
                    input_field.text = ""; // Clear input box after click
                }
            }
        }
    }
}

// 2. THE APPLICATION LOGIC
fn main() -> Result<(), slint::PlatformError> {
    // Manually initialize the tokio runtime container
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    // Enter the runtime context so tokio::spawn works anywhere inside the thread
    let _guard = rt.enter();

    // AppWindow is now fully visible to the code below
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_submit_prompt(move |prompt| {
        // Create a dedicated clone of the handle for THIS specific button click event
        let ui_handle_for_click = ui_handle.clone(); 
        let prompt_str = prompt.to_string();

        println!("Prompt captured: {}", prompt_str);

        // This spawn command hooks directly into the runtime context initialized above
        tokio::spawn(async move {
            // Clone the click-specific handle for the initial loading state update
            let ui_clone_loading = ui_handle_for_click.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui_clone_loading.upgrade() {
                    ui.set_log_text("Thinking...".into());
                }
            });

            // Simulate the LLM API processing delay
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            let mock_llm_response = format!(
                "Received prompt: '{}'\n\nThis is a mock response from your backend harness!", 
                prompt_str
            );

            // Use the click-specific handle for the final response update
            let ui_clone_final = ui_handle_for_click;
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui_clone_final.upgrade() {
                    ui.set_log_text(mock_llm_response.into());
                }
            });
        });
    });

    ui.run()
}
