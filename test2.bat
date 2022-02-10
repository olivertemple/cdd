@echo off
if %1 == -l (
    for /F %%B in ('^""C:\Users\ollie\Documents\cdr\target\release\cdr.exe" "%1"^"') do (
        echo %%B
)
) else (for /F %%A in ('^""C:\Users\ollie\Documents\cdr\target\release\cdr.exe" "%1"^"') do (
    echo %%A
    cd %%A
))