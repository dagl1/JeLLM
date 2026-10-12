// src/main.rs
use tokio;
mod markdown;

// 2. THE APPLICATION LOGIC
fn main() -> Result<(), slint::PlatformError> {
    return Ok(());

    // // Manually initialize the tokio runtime container
    // let rt = tokio::runtime::Runtime::new().unwrap();
    //
    // // Enter the runtime context so tokio::spawn works anywhere inside the thread
    // let _guard = rt.enter();
    //
    // // AppWindow is now fully visible to the code below
    // let ui = AppWindow::new()?;
    // let ui_handle = ui.as_weak();
    //
    // ui.on_submit_prompt(move |prompt| {
    //     // Create a dedicated clone of the handle for THIS specific button click event
    //     let ui_handle_for_click = ui_handle.clone();
    //     let prompt_str = prompt.to_string();
    //
    //     println!("Prompt captured: {}", prompt_str);
    //
    //     // This spawn command hooks directly into the runtime context initialized above
    //     tokio::spawn(async move {
    //         // Clone the click-specific handle for the initial loading state update
    //         let ui_clone_loading = ui_handle_for_click.clone();
    //         let _ = slint::invoke_from_event_loop(move || {
    //             if let Some(ui) = ui_clone_loading.upgrade() {
    //                 ui.set_log_text("Thinking...".into());
    //             }
    //         });
    //
    //         // Simulate the LLM API processing delay
    //         tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    //         let mock_llm_response = format!(
    //             "Received prompt: '{}'\n\nThis is a mock response from your backend harness!",
    //             prompt_str
    //         );
    //
    //         // Use the click-specific handle for the final response update
    //         let ui_clone_final = ui_handle_for_click;
    //         let _ = slint::invoke_from_event_loop(move || {
    //             if let Some(ui) = ui_clone_final.upgrade() {
    //                 ui.set_log_text(mock_llm_response.into());
    //             }
    //         });
    //     });
    // });
    //
    // ui.run()
}
