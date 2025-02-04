# Universal WIFI Driver  
A Rust-based tool to simplify WiFi driver management on Windows. This project automatically identifies your WiFi hardware (adapters, chipsets, routers) and installs the correct drivers. Designed for 64-bit Windows 10 and newer systems.  

## Download  
1. Visit the [Releases](https://github.com/muzondie/universal-wifi-driver/releases) tab on GitHub.  
2. Download the latest `.zip` file for Windows.  
3. Unzip the file and run `UniversalWifiDriver.exe`.  

## Usage  
1. **Run the application** after installation.  
2. The GUI will launch and **automatically scan** for WiFi hardware.  
3. Follow prompts to install or update drivers. No further input is typically needed.  

## Features  
- **Automatic detection** of WiFi adapters, chipsets, routers, etc.  
- Supports hardware from **all major and niche manufacturers** (Intel, Qualcomm, Broadcom, etc).  
- GUI interface for easy navigation.  
- Background updates for driver databases.  
- Offline mode (uses cached drivers if no internet is available).  
- Repair or reinstall existing drivers.  
- Lightweight, with minimal system resource usage.  
- Automatic rollback on installation errors.  
- Logging for troubleshooting (saves to `logs/` directory).  

## Build from Source  
1. Install [Rust](https://www.rust-lang.org/tools/install).  
2. Clone the repository:  
   ```bash  
   git clone https://github.com/muzondie/universal-wifi-driver.git  
   ```  
3. Build the project:  
   ```bash  
   cd universal-wifi-driver  
   cargo build --release  
   ```  
4. Find the executable in `target/release/`.  

## Contributing  
Contributions, bug reports, and feature requests are **currently not being accepted** due to maintainer capacity. Thank you for understanding.  

## License  
MIT License. See [LICENSE](LICENSE) for details.