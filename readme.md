<p align="center">
    <img src="./icon.png" height=150/>
</p>
<h1> CDD </h1>

CDD is a simple and easy to use tool that allows you to easily move to a different directory without having to type the full path. For example, `cd C:\Users\John\Documents\Photos\2020\Summer\` could become `cdd summer_2020`.
<br>
<br>

## Installation

Download `cdd.bat`, and either place it in `C:\Windows\System32\` or add the folder where it is placed to you `PATH` environment variable. Also place `cddmanage.exe` in the same folder. A file will be created in your AppData folder to store the paths that have been saved.

## Usage
`cdd -a [NAME] [PATH]` will add a new path that can be used with `cdd [NAME]`. This will overwrite any existing path with the same name.

`cdd -d [NAME]` will delete the path with the given name.

`cdd -l` will list all the paths that are currently saved.

`cdd NAME [-r]` will change the current directory to the path with the given name. If a `cddRun.bat` file is present in the directory that is being changed to, it will be executed. This will allow you to automatically start the program when you change directories if desired. If the `-r` flag is used, `cddRun.bat` will not be run.


# Information
This program works by using `cddmanaage.exe` to read the file where the paths are stored and returns it to the `cdd.bat` file. This is because, changing the current directory must be done through a batch file, and file parsing is much easier in rust.
