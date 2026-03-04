@echo off
echo Очистка кэша иконок Windows...
taskkill /F /IM explorer.exe 2>nul
del /A "%localappdata%\IconCache.db" 2>nul
del /A "%localappdata%\Microsoft\Windows\Explorer\iconcache*" 2>nul
start explorer.exe
echo Готово! Проводник перезапущен.
pause
