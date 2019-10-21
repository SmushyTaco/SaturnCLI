## 📝 Table of Contents
- [About](#about)
- [Getting Started](#getting_started)
- [How to Build](#building)
- [How to Install](#installing)
- [Usage](#usage)
- [Built Using](#built_using)
- [Authors](#authors)

## 🧐 About <a name = "about"></a>
SaturnCLI is made to make using APT less annoying by providing an easy to use set of commands with syntax highlighting that never require you to ever have to type sudo or worry about case sensitivity!

## 🏁 Getting Started <a name = "getting_started"></a>
Getting started for contributing to SaturnCLI is very easy, you just need to follow the instructions below if you wish to build from source.

### Prerequisites
In order to start building SaturnCLI, you are going to need to have rustup and cargo installed (Installing rustup automatically installs cargo so just worry about rustup). You can get rustup [here](https://www.rust-lang.org/tools/install/).

Without [APT](https://launchpad.net/ubuntu/+source/apt/) (Linux) it's useless though so be sure to install what fits for your system!

## Building

* cd into the stn directory (You can easily do so by opening the terminal typing ```cd``` and dragging and dropping the stn folder into the terminal window and clicking the enter key).
* Run ```cargo build --release``` in the terminal.
* Make sure your in the stn directory and from there go into the target directory and then go into the release directory (stn --> target --> release).
* You should see a executable file named ```stn```.
* Congratulations! You have successfully built SaturnCLI (stn)!

## Installing
### Linux and macOS
* If you want to build from source [click here](#Building). If you just want a precompiled binary [click here](https://github.com/realSmushyTaco/SaturnCLI/releases)
* Navigate to ```/usr/local/bin/```.
* Drag the stn binary you got from compiling (or downloading) saturn in the window that just popped up.
* Congratulations! You have successfully installed SaturnCLI (stn)! Without [APT](https://launchpad.net/ubuntu/+source/apt/) (Linux) it's useless though so be sure to install apt if you don't have it!
* Whenever a new SaturnCLI update is out just [rebuild](#Building) or download the latest precompiled binary by [clicking here](https://github.com/realSmushyTaco/SaturnCLI/releases), navigate to ```/usr/local/bin/``` and replace the old ```stn``` binary with the new ```stn``` binary you just rebuilt.

### Windows 10
* If you want to build from source [click here](#Building). If you just want a precompiled binary [click here](https://github.com/realSmushyTaco/SaturnCLI/releases)
* Navigate to your ```C:``` directory (This should be your very last directory shown in the file explorer besides ```This PC```) and create a new folder named ```bin```.
* Put the ```stn``` binary in the ```bin``` folder.
* In the ```Type here to search``` box search ```Edit the system environment variables``` and open the control center program.
* Click ```Envirnment Variables..```.
* Under ```System variables``` (The bottom box with the list of variables) look for the variable named ```path``` and click on it then click ```Edit...``` (A window named ```Edit envirnment variable``` should pop up).
* Click ```New``` and input ```C:\bin``` for the new entry. After you do that click ```OK``` for all the windows related to ```Edit envirnment variable```.
* In the ```Type here to search``` box search ```Registry Editor``` and open the Registry Editor app (If it asks if you want to allow the app to make changes to your device choose ```Yes```).
* Click the drop down arrow for ```HKEY_CURRENT_USER``` and select the folder named ```Console```.
* In the box where it says ```Name```, ```Type```, and ```Data``` look for the name ```VirtualTerminalLevel``` (Under ```Name``` of course). If you don't see ```VirtualTerminalLevel``` right click in the box, hover over ```New``` and click ```DWORD (32-bit) Value``` and give it the name ```VirtualTerminalLevel```.
* Right click on ```VirtualTerminalLevel``` and click ```Modify...``` (A window named ```Edit DWORD (32-bit) Value``` should pop up).
* Make sure the ```Value name:``` is ```VirtualTerminalLevel```, make sure the ```Value data:``` is ```1```, and make sure the ```Base``` has ```Hexadecimal``` selected. After all that information has been properly set click ```OK``` and exit the Registry Editor app (This registry option has to be set to ```1``` so ANSI mode is enabled by default so text coloring properly works with SaturnCLI, if you want to revert this back just set the ```Value data:``` to ```0``` but if you do choose to revert back coloring in SaturnCLI will break and the text will be hard to read because the escape sequences will be shown).
* Congratulations! You have successfully installed SaturnCLI (stn)! Without [WSL](https://docs.microsoft.com/en-us/windows/wsl/install-win10) (The Linux subsystem for Windows) SaturnCLI will be useless so make sure you install WSL and make sure the Linux distribution you use with WSL has [APT](https://launchpad.net/ubuntu/+source/apt/) installed.
* Whenever a new SaturnCLI update is out just [rebuild](#Building) or download the latest precompiled binary by [clicking here](https://github.com/realSmushyTaco/SaturnCLI/releases), navigate to ```C:\bin``` and replace the old ```stn``` binary with the new ```stn``` binary you just rebuilt.

## 🎈 Usage <a name="usage"></a>
SaturnCLI commands:

* ```stn search <query>``` - Searches for specified query
* ```stn list [--flag(s)] <package(s)>```  - Lists specified packages
* ```stn info [package(s)]``` - Display info on specified package(s)
* ```stn install [package(s)]``` - Installs specified package(s)
* ```stn reinstall [package(s)]``` - Reinstalls specified package(s)
* ```stn remove [package(s)]``` - Removes specified package(s)
* ```stn add-key <filepath>``` - Adds a key to the list of trusted keys
* ```stn edit-sources``` - Opens the APT repo editor
* ```stn add-repos [--flag(s)] <repo(s)>``` - Adds new repo(s)
* ```stn add-source``` - Adds a new repository
* ```stn autoremove``` - Removes unneeded packages (orphans)
* ```stn update``` - Updates the repository lists
* ```stn upgrade [package(s)]``` - Upgrades specified packages
* ```stn full-upgrade <package(s)>``` - Upgrades the system (or package(s)) (Linux Only)
* ```stn version``` - Show APT, DPKG, and SaturnCLI versions
* ```stn clean``` - Clear the download cache
* ```stn help <command>``` - Opens help menu for specified commands
* ```stn about``` - View legal information and credits

## ⛏️ Built Using <a name = "built_using"></a>
- [RustLang](https://www.rust-lang.org/) - The only used programming language.

## ✍️ Authors <a name = "authors"></a>
- [@realSmushyTaco](https://github.com/realSmushyTaco)
