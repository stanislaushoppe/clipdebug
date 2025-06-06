use std::time::Duration;
use tokio::time::sleep;
use arboard::Clipboard;
use std::process::Command;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let debug_mode = args.contains(&"-debug".to_string());
    
    if debug_mode {
        println!("🔍 Clipboard Debugger für Wayland/Hyprland gestartet");
        println!("Überwache Clipboard-Änderungen... (Strg+C zum Beenden)\n");
    }

    let mut last_content = String::new();
    let mut iteration = 0;

    loop {
        match get_clipboard_content().await {
            Ok(current_content) => {
                if current_content != last_content && !current_content.is_empty() {
                    iteration += 1;
                    
                    if debug_mode {
                        println!("📋 Clipboard-Änderung #{} erkannt:", iteration);
                        println!("=== Text-Inhalt ===");
                        
                        // Text ausgeben (mit Zeilenbegrenzung für bessere Lesbarkeit)
                        let lines: Vec<&str> = current_content.lines().collect();
                        for (i, line) in lines.iter().enumerate() {
                            if i > 10 {
                                println!("... ({} weitere Zeilen)", lines.len() - 10);
                                break;
                            }
                            println!("{}", line);
                        }
                        
                        println!("\n=== Hex-Repräsentation ===");
                        
                        let bytes = current_content.as_bytes();
                        println!("Länge: {} Bytes", bytes.len());
                        
                        // Hex-Ausgabe in 16-Byte-Blöcken
                        for (offset, chunk) in bytes.chunks(16).enumerate() {
                            let hex_string: String = chunk.iter()
                                .map(|b| format!("{:02x}", b))
                                .collect::<Vec<String>>()
                                .join(" ");
                            
                            let ascii_string: String = chunk.iter()
                                .map(|&b| if b.is_ascii_graphic() || b == b' ' { 
                                    b as char 
                                } else { 
                                    '.' 
                                })
                                .collect();
                            
                            println!("{:08x}: {:48} [{}]", 
                                    offset * 16, 
                                    hex_string, 
                                    ascii_string);
                        }
                    }
                    
                    // Inhalt an wl-copy weitergeben
                    match send_to_wl_copy(&current_content).await {
                        Ok(_) => {
                            if debug_mode {
                                println!("✅ Inhalt erfolgreich an wl-copy übertragen");
                            }
                        },
                        Err(e) => {
                            if debug_mode {
                                println!("⚠️  Fehler beim Übertragen an wl-copy: {}", e);
                            }
                        }
                    }
                    
                    if debug_mode {
                        println!("==========================================\n");
                    }
                    
                    last_content = current_content;
                }
            }
            Err(_) => {
                // Fehler beim Lesen des Clipboards werden stumm ignoriert
            }
        }
        
        // Kurze Pause zwischen den Überprüfungen
        sleep(Duration::from_millis(500)).await;
    }
}

async fn get_clipboard_content() -> Result<String, Box<dyn std::error::Error>> {
    // Verwende arboard für plattformübergreifende Clipboard-Unterstützung
    let mut clipboard = Clipboard::new()?;
    let content = clipboard.get_text()?;
    Ok(content)
}

async fn send_to_wl_copy(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new("wl-copy")
        .stdin(std::process::Stdio::piped())
        .spawn()?;
    
    if let Some(stdin) = child.stdin.as_mut() {
        use std::io::Write;
        stdin.write_all(content.as_bytes())?;
    }
    
    child.wait()?;
    Ok(())
}