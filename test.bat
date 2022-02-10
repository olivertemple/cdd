@echo off
FOR /F "usebackq tokens=3*" %%A IN (`reg query "HKCU\Environment"`) DO (
    ECHO %%A %%B
    SET myVar=%%A
    ECHO %myVar:~1%
)
@REM SET test=123456789abcdef0
@REM ECHO %test:~-3%