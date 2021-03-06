@ECHO off

if %1== -a (
    goto :add
)
if %1== --add (
    goto :add
)
if %1== -d (
    goto :delete
)
if %1== --delete (
    goto :delete
)
if %1== -l (
    goto :list
)
if %1== --list (
    goto :list
)
if %1== -h (
    goto :help
)
if %1== --help (
    goto :help
)

for /f "delims=" %%A in ('cddmanage.exe %1') DO (
    cd %%A
    if not "%~2"=="-r" (
    if exist cddRun.bat (
        cddRun.bat
    )
)
)
goto :end

:add
for /f "delims=" %%A in ('cddmanage -a %2 %3') DO (
    echo %%A
)
goto :end

:delete
for /f "delims=" %%A in ('cddmanage -d %2') DO (
    echo %%A
)
goto :end

:list
for /f "delims=" %%A in ('cddmanage -l') DO (
    echo %%A
)
goto :end

:help
echo Usage: cdd.bat
echo        [name] - change directory to name
echo        [name] -r - don't run cddRun.bat in the new directory
echo        [name/child] - change directory to name/child
echo        -a(--add) [name] [path] - add a new directory
echo        -d(--delete) [name] - delete a directory
echo        -l(--list) - list all directories
echo        -h(--help) - show this help dialog
goto :end

:end