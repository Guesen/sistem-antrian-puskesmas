use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintTicketRequest {
    pub queue_code: String,
    pub loket_type: String,
    pub patient_type: String,
    pub created_at: String,
}

#[tauri::command]
pub async fn print_thermal_ticket(request: PrintTicketRequest) -> Result<String, String> {
    log::info!("Attempting to print thermal ticket: {}", request.queue_code);
    
    // Try to find and print to thermal printer
    match find_and_print_thermal(&request) {
        Ok(_) => {
            log::info!("Successfully printed to thermal printer");
            Ok("Berhasil mencetak ke printer thermal".to_string())
        }
        Err(e) => {
            log::warn!("Thermal printer not available: {}. Falling back to standard print.", e);
            // Return error to trigger fallback to window.print()
            Err(format!("Printer thermal tidak ditemukan: {}. Gunakan print dialog.", e))
        }
    }
}

fn find_and_print_thermal(request: &PrintTicketRequest) -> Result<(), String> {
    // Get available serial ports
    let ports = serialport::available_ports()
        .map_err(|e| format!("Gagal mencari port: {}", e))?;
    
    log::info!("Found {} serial ports", ports.len());
    
    // Try to find thermal printer
    for port_info in ports {
        log::info!("Checking port: {}", port_info.port_name);
        
        // Try common thermal printer patterns
        let port_name = &port_info.port_name;
        if is_likely_thermal_printer(port_name) {
            log::info!("Attempting to connect to potential thermal printer: {}", port_name);
            
            match print_to_port(port_name, request) {
                Ok(_) => {
                    log::info!("Successfully printed to {}", port_name);
                    return Ok(());
                }
                Err(e) => {
                    log::warn!("Failed to print to {}: {}", port_name, e);
                    continue;
                }
            }
        }
    }
    
    Err("Tidak ada printer thermal yang ditemukan".to_string())
}

fn is_likely_thermal_printer(port_name: &str) -> bool {
    // Common patterns for thermal printers on different platforms
    let patterns = [
        "USB",           // Windows: COM ports with USB
        "ttyUSB",        // Linux: USB serial
        "ttyACM",        // Linux: ACM devices
        "cu.usbserial",  // macOS: USB serial
        "cu.usbmodem",   // macOS: USB modem
        "POS",           // Generic POS printer
        "Printer",       // Generic printer
    ];
    
    patterns.iter().any(|pattern| port_name.contains(pattern))
}

fn print_to_port(port_name: &str, request: &PrintTicketRequest) -> Result<(), String> {
    // Open serial port with common thermal printer settings
    let mut port = serialport::new(port_name, 9600)
        .timeout(std::time::Duration::from_millis(2000))
        .open()
        .map_err(|e| format!("Gagal membuka port: {}", e))?;
    
    // ESC/POS commands
    let esc = 0x1B;
    let gs = 0x1D;
    
    // Initialize printer
    port.write_all(&[esc, b'@'])
        .map_err(|e| format!("Gagal initialize printer: {}", e))?;
    
    // Set alignment to center
    port.write_all(&[esc, b'a', 1])
        .map_err(|e| format!("Gagal set alignment: {}", e))?;
    
    // Set text size to double (width and height)
    port.write_all(&[gs, b'!', 0x11])
        .map_err(|e| format!("Gagal set text size: {}", e))?;
    
    // Print header
    port.write_all(b"PUSKESMAS MREBET\n")
        .map_err(|e| format!("Gagal print header: {}", e))?;
    
    // Reset text size
    port.write_all(&[gs, b'!', 0x00])
        .map_err(|e| format!("Gagal reset text size: {}", e))?;
    
    port.write_all(b"Nomor Antrian\n\n")
        .map_err(|e| format!("Gagal print subtitle: {}", e))?;
    
    // Set text size to triple for queue number
    port.write_all(&[gs, b'!', 0x22])
        .map_err(|e| format!("Gagal set large text: {}", e))?;
    
    // Print queue code (large)
    port.write_all(request.queue_code.as_bytes())
        .map_err(|e| format!("Gagal print queue code: {}", e))?;
    port.write_all(b"\n\n")
        .map_err(|e| format!("Gagal print newline: {}", e))?;
    
    // Reset text size
    port.write_all(&[gs, b'!', 0x00])
        .map_err(|e| format!("Gagal reset text size: {}", e))?;
    
    // Set alignment to left
    port.write_all(&[esc, b'a', 0])
        .map_err(|e| format!("Gagal set left alignment: {}", e))?;
    
    // Print details
    let loket_line = format!("Loket: {}\n", request.loket_type);
    port.write_all(loket_line.as_bytes())
        .map_err(|e| format!("Gagal print loket: {}", e))?;
    
    let patient_line = format!("Jenis: {}\n", request.patient_type);
    port.write_all(patient_line.as_bytes())
        .map_err(|e| format!("Gagal print patient type: {}", e))?;
    
    let time_line = format!("Waktu: {}\n\n", request.created_at);
    port.write_all(time_line.as_bytes())
        .map_err(|e| format!("Gagal print time: {}", e))?;
    
    // Set alignment to center
    port.write_all(&[esc, b'a', 1])
        .map_err(|e| format!("Gagal set center alignment: {}", e))?;
    
    port.write_all(b"Terima Kasih\n")
        .map_err(|e| format!("Gagal print thank you: {}", e))?;
    port.write_all(b"Harap Menunggu\n\n\n")
        .map_err(|e| format!("Gagal print wait message: {}", e))?;
    
    // Feed paper
    port.write_all(&[esc, b'd', 3])
        .map_err(|e| format!("Gagal feed paper: {}", e))?;
    
    // Cut paper (full cut)
    port.write_all(&[gs, b'V', 0])
        .map_err(|e| format!("Gagal cut paper: {}", e))?;
    
    // Flush to ensure all data is sent
    port.flush()
        .map_err(|e| format!("Gagal flush data: {}", e))?;
    
    Ok(())
}

