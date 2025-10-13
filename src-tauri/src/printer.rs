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
    
    // Generate ESC/POS data
    let print_data = generate_escpos_data(&request);
    
    // Try Windows USB printer first (for USB-001, etc.)
    #[cfg(target_os = "windows")]
    {
        log::info!("Trying Windows USB printer...");
        match try_windows_usb_printers(&print_data) {
            Ok(printer_name) => {
                log::info!("Successfully printed to Windows printer: {}", printer_name);
                return Ok(format!("Berhasil mencetak ke {}", printer_name));
            }
            Err(e) => {
                log::warn!("Windows USB printer not available: {}", e);
            }
        }
    }
    
    // Fallback to serial port scan (for COM ports)
    log::info!("Trying serial ports...");
    match find_and_print_thermal(&request) {
        Ok(_) => {
            log::info!("Successfully printed to serial thermal printer");
            Ok("Berhasil mencetak ke printer thermal".to_string())
        }
        Err(e) => {
            log::warn!("Thermal printer not available: {}. Falling back to standard print.", e);
            // Return error to trigger fallback to window.print()
            Err(format!("Printer thermal tidak ditemukan: {}. Gunakan print dialog.", e))
        }
    }
}

// Windows-specific USB printer support
#[cfg(target_os = "windows")]
fn try_windows_usb_printers(data: &[u8]) -> Result<String, String> {
    use std::ffi::CString;
    use std::ptr;
    use winapi::um::winspool::{
        ClosePrinter, EnumPrintersA, OpenPrinterA, StartDocPrinterA, 
        EndDocPrinter, StartPagePrinter, EndPagePrinter, WritePrinter,
        PRINTER_ENUM_LOCAL, PRINTER_INFO_2A, DOC_INFO_1A
    };
    use winapi::shared::minwindef::DWORD;
    
    // Common thermal printer names to try
    let printer_names = vec![
        "USB-001",
        "USB001", 
        "USB-002",
        "USB002",
        "POS-80",
        "TP806",
        "TP860",
        "Thermal Printer",
        "XP-80C",
        "ZJ-80",
    ];
    
    // Try each printer name
    for printer_name in &printer_names {
        log::info!("Trying printer: {}", printer_name);
        match print_to_windows_printer(printer_name, data) {
            Ok(_) => return Ok(printer_name.to_string()),
            Err(e) => {
                log::debug!("Failed to print to {}: {}", printer_name, e);
                continue;
            }
        }
    }
    
    // If none of the common names worked, try to enumerate all printers
    log::info!("Trying to enumerate all local printers...");
    unsafe {
        let mut needed: DWORD = 0;
        let mut returned: DWORD = 0;
        
        // First call to get buffer size
        EnumPrintersA(
            PRINTER_ENUM_LOCAL,
            ptr::null_mut(),
            2,
            ptr::null_mut(),
            0,
            &mut needed,
            &mut returned,
        );
        
        if needed > 0 {
            let mut buffer: Vec<u8> = vec![0; needed as usize];
            
            // Second call to get actual data
            if EnumPrintersA(
                PRINTER_ENUM_LOCAL,
                ptr::null_mut(),
                2,
                buffer.as_mut_ptr(),
                needed,
                &mut needed,
                &mut returned,
            ) != 0 {
                let printers = buffer.as_ptr() as *const PRINTER_INFO_2A;
                
                for i in 0..returned {
                    let printer = printers.offset(i as isize);
                    if !(*printer).pPrinterName.is_null() {
                        let name = std::ffi::CStr::from_ptr((*printer).pPrinterName)
                            .to_string_lossy()
                            .to_string();
                        
                        log::info!("Found printer: {}", name);
                        
                        // Try to print to this printer
                        if let Ok(_) = print_to_windows_printer(&name, data) {
                            return Ok(name);
                        }
                    }
                }
            }
        }
    }
    
    Err("No Windows USB printer found".to_string())
}

#[cfg(target_os = "windows")]
fn print_to_windows_printer(printer_name: &str, data: &[u8]) -> Result<(), String> {
    use std::ffi::CString;
    use std::ptr;
    use winapi::um::winspool::{
        ClosePrinter, OpenPrinterA, StartDocPrinterA, EndDocPrinter,
        StartPagePrinter, EndPagePrinter, WritePrinter, DOC_INFO_1A
    };
    
    unsafe {
        let printer_cstr = CString::new(printer_name).map_err(|e| e.to_string())?;
        let mut printer_handle = ptr::null_mut();
        
        // Open printer
        if OpenPrinterA(
            printer_cstr.as_ptr() as *mut i8,
            &mut printer_handle,
            ptr::null_mut()
        ) == 0 {
            return Err(format!("Failed to open printer: {}", printer_name));
        }
        
        // Start document
        let doc_name = CString::new("Antrian Ticket").unwrap();
        let mut doc_info = DOC_INFO_1A {
            pDocName: doc_name.as_ptr() as *mut i8,
            pOutputFile: ptr::null_mut(),
            pDatatype: ptr::null_mut(),
        };
        
        if StartDocPrinterA(printer_handle, 1, &mut doc_info as *mut _ as *mut u8) == 0 {
            ClosePrinter(printer_handle);
            return Err("Failed to start document".to_string());
        }
        
        // Start page
        if StartPagePrinter(printer_handle) == 0 {
            EndDocPrinter(printer_handle);
            ClosePrinter(printer_handle);
            return Err("Failed to start page".to_string());
        }
        
        // Write data
        let mut bytes_written = 0u32;
        if WritePrinter(
            printer_handle,
            data.as_ptr() as *mut _,
            data.len() as u32,
            &mut bytes_written,
        ) == 0 {
            EndPagePrinter(printer_handle);
            EndDocPrinter(printer_handle);
            ClosePrinter(printer_handle);
            return Err("Failed to write to printer".to_string());
        }
        
        // End page and document
        EndPagePrinter(printer_handle);
        EndDocPrinter(printer_handle);
        ClosePrinter(printer_handle);
        
        log::info!("Successfully wrote {} bytes to {}", bytes_written, printer_name);
    }
    
    Ok(())
}

fn generate_escpos_data(request: &PrintTicketRequest) -> Vec<u8> {
    let mut data = Vec::new();
    
    // ESC/POS commands
    let esc = 0x1B;
    let gs = 0x1D;
    
    // Initialize printer
    data.extend_from_slice(&[esc, b'@']);
    
    // Set alignment to center
    data.extend_from_slice(&[esc, b'a', 1]);
    
    // Set text size to double (width and height)
    data.extend_from_slice(&[gs, b'!', 0x11]);
    
    // Print header
    data.extend_from_slice(b"PUSKESMAS MREBET\n");
    
    // Reset text size
    data.extend_from_slice(&[gs, b'!', 0x00]);
    
    data.extend_from_slice(b"Nomor Antrian\n\n");
    
    // Set text size to triple for queue number
    data.extend_from_slice(&[gs, b'!', 0x22]);
    
    // Print queue code (large)
    data.extend_from_slice(request.queue_code.as_bytes());
    data.extend_from_slice(b"\n\n");
    
    // Reset text size
    data.extend_from_slice(&[gs, b'!', 0x00]);
    
    // Set alignment to left
    data.extend_from_slice(&[esc, b'a', 0]);
    
    // Print details
    let loket_line = format!("Loket: {}\n", request.loket_type);
    data.extend_from_slice(loket_line.as_bytes());
    
    let patient_line = format!("Jenis: {}\n", request.patient_type);
    data.extend_from_slice(patient_line.as_bytes());
    
    let time_line = format!("Waktu: {}\n\n", request.created_at);
    data.extend_from_slice(time_line.as_bytes());
    
    // Set alignment to center
    data.extend_from_slice(&[esc, b'a', 1]);
    
    data.extend_from_slice(b"Terima Kasih\n");
    data.extend_from_slice(b"Harap Menunggu\n\n\n");
    
    // Feed paper
    data.extend_from_slice(&[esc, b'd', 3]);
    
    // Cut paper (full cut)
    data.extend_from_slice(&[gs, b'V', 0]);
    
    data
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
    
    // Generate and send ESC/POS data
    let data = generate_escpos_data(request);
    port.write_all(&data)
        .map_err(|e| format!("Gagal menulis ke port: {}", e))?;
    
    // Flush to ensure all data is sent
    port.flush()
        .map_err(|e| format!("Gagal flush data: {}", e))?;
    
    Ok(())
}
