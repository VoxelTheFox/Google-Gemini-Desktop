!macro NSIS_HOOK_PREINSTALL
    DetailPrint "Checking if Google Gemini is running..."
    
    loop:
    # Check if the production executable exists in the installation directory and is locked
    IfFileExists "$INSTDIR\Google Gemini.exe" check_prod_locked check_dev
    
    check_prod_locked:
    ClearErrors
    FileOpen $0 "$INSTDIR\Google Gemini.exe" "a"
    IfErrors running
    FileClose $0
    
    check_dev:
    # Check if the development executable exists in the installation directory and is locked
    IfFileExists "$INSTDIR\gemini-desktop-tauri.exe" check_dev_locked not_running
    
    check_dev_locked:
    ClearErrors
    FileOpen $0 "$INSTDIR\gemini-desktop-tauri.exe" "a"
    IfErrors running
    FileClose $0
    Goto not_running
    
    running:
    MessageBox MB_RETRYCANCEL|MB_ICONEXCLAMATION "Google Gemini is currently running. Please close the app and click Retry to continue, or Cancel to abort the installation." /SD IDCANCEL IDRETRY loop IDCANCEL abort_install
        
    abort_install:
    Abort "Installation aborted."
    
    not_running:
!macroend
