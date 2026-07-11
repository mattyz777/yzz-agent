use arboard::Clipboard;

pub fn copy_to_clipboard(text: &str) -> Result<(), String> {
    let mut clipboard = Clipboard::new()
        .map_err(|_| "cannot access clipboard")?;
    
    clipboard.set_text(text)
        .map_err(|e| format!("failed to copy: {}", e))?;

    Ok(())
}