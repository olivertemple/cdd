@echo off

if %1== -h (
    echo Usage: cdd.bat
    echo        [name] - change directory to name
    echo        [name] -r - don't run run.bat in the new directory
    echo        -a [name] [path] - add a new directory
    echo        -d [name] - delete a directory
    echo        -h - show this help dialog
) else (
    if %1 == -a (
        setx %2 %3
    ) else (
        if %1 == -d (
            reg delete "HKCU\Environment" /v %2 /f
        ) else (
            FOR /F "usebackq tokens=3*" %%A IN (`reg query "HKCU\Environment" /v %1`) DO (
                cd %%A %%B
                if not "%~2"=="-r" (
                    if exist run.bat (
                        run.bat
                    )
                )
            )
        )
    )
)

