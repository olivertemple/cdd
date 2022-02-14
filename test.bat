@echo off
SET myVar=cddPath?C:\Users\ollie\
for /f "tokens=2 delims=?" %%A in ("%myVar%") Do @Echo %%A
