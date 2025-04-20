@echo off
for /l %%i in (1,1,14) do (
    setlocal enabledelayedexpansion
    set "num=0%%i"
    set "num=!num:~-2!"
    echo. > "!num!.txt"
    endlocal
)