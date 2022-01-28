@echo off
IF ("%TA_TOOLS_PATH%") NEQ ("") (
    MKLINK "%TA_TOOLS_PATH%\\bin\\RemoteLogger.dll" "%~dp0\\target\\x86_64-pc-windows-msvc\\release-minsize\\t7_tool_ext.dll"
    GOTO END
)

@ECHO Environment variable 'TA_TOOLS_PATH' not found. You must run the mod tools at least once, or manually set this value

:END