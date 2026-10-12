
slint::slint! {
    import { Button, LineEdit, TextEdit } from "std-widgets.slint";

    export component AppWindow inherits Window {
        title: "LLM Harness UI";
        min-width: 600px;
        min-height: 500px;

        in-out property <string> log-text: "System: Ready for prompts...";
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
