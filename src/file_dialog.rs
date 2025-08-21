use anyhow::Result;
use std::path::PathBuf;

pub struct FileDialog;

impl FileDialog {
    pub fn open_file() -> Result<Option<PathBuf>> {
        #[cfg(target_os = "macos")]
        {
            Self::open_file_macos()
        }
        #[cfg(target_os = "linux")]
        {
            Self::open_file_linux()
        }
        #[cfg(target_os = "windows")]
        {
            Self::open_file_windows()
        }
        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        {
            Ok(None)
        }
    }
    
    #[cfg(target_os = "macos")]
    fn open_file_macos() -> Result<Option<PathBuf>> {
        use std::process::Command;
        
        let output = Command::new("osascript")
            .arg("-e")
            .arg(r#"
                set theFile to choose file with prompt "Select a MIDI file" of type {"mid", "midi", "MID", "MIDI"}
                POSIX path of theFile
            "#)
            .output()?;
        
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout);
            let trimmed = path_str.trim();
            if !trimmed.is_empty() {
                return Ok(Some(PathBuf::from(trimmed)));
            }
        }
        
        Ok(None)
    }
    
    #[cfg(target_os = "linux")]
    fn open_file_linux() -> Result<Option<PathBuf>> {
        use std::process::Command;
        
        let output = Command::new("zenity")
            .arg("--file-selection")
            .arg("--title=Select a MIDI file")
            .arg("--file-filter=MIDI files (*.mid *.midi) | *.mid *.midi")
            .output()?;
        
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout);
            let trimmed = path_str.trim();
            if !trimmed.is_empty() {
                return Ok(Some(PathBuf::from(trimmed)));
            }
        }
        
        Ok(None)
    }
    
    #[cfg(target_os = "windows")]
    fn open_file_windows() -> Result<Option<PathBuf>> {
        use std::process::Command;
        
        let output = Command::new("powershell")
            .arg("-Command")
            .arg(r#"
                Add-Type -AssemblyName System.Windows.Forms;
                $openFileDialog = New-Object System.Windows.Forms.OpenFileDialog;
                $openFileDialog.Filter = "MIDI files (*.mid;*.midi)|*.mid;*.midi";
                $openFileDialog.Title = "Select a MIDI file";
                if ($openFileDialog.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) {
                    $openFileDialog.FileName
                }
            "#)
            .output()?;
        
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim();
            if !path_str.is_empty() && path_str != "null" {
                return Ok(Some(PathBuf::from(path_str)));
            }
        }
        
        Ok(None)
    }
}