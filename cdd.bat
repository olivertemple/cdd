@echo off

if %1== -h (
    echo "Usage: cdd.bat [-a (add)|-d (delete)] [name] [path]"
) else (
    if %1 == -a (
        setx %2 %3
    ) else (
        if %1 == -d (
            reg delete "HKCU\Environment" /v %2 /f
        ) else (
            FOR /F "usebackq tokens=3*" %%A IN (`reg query "HKCU\Environment" /v %1`) DO (
                cd %%A %%B
            )
        )
    )
)

