<center>
    <img src="./icon.png" height=150/>
    <h1> CDR </h1>
</center>
CDR is a simple and easy to use tool that allows you to easily move to a different directory without having to type the full path. For example, `cd C:\Users\John\Documents\Photos\2020\Summer\` could become `cdr summer_2020`.
<br>
<br>

## Installation
Download cdr.bat, and either place it in C:\Windows\System32\ or add the folder where it is placed to you PATH environment variable.

## Usage
`cdr -a NAME PATH` will add a new path that can be used with `cdr NAME`. This will overwrite any existing path with the same name.

`crd -d NAME` will delete the path with the given name.

`cdr NAME` will change the current directory to the path with the given name.
