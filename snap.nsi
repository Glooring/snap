; snap.nsi — NSIS installer script for Snap

!include "MUI2.nsh"
RequestExecutionLevel admin

!ifndef APP_NAME
!define APP_NAME "snap"
!endif
!ifndef APP_VER
!define APP_VER "7.2.0"
!endif
!ifndef APP_EXE
!define APP_EXE "snap.exe"
!endif
!ifndef OUT_FILE
!define OUT_FILE "${APP_NAME}-setup.exe"
!endif

Name "${APP_NAME} ${APP_VER}"
OutFile "${OUT_FILE}"
InstallDir "$PROGRAMFILES\${APP_NAME}"
InstallDirRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" ""

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "wix\License.rtf"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_LANGUAGE "English"

Section "Install"
    SetOutPath "$INSTDIR"
    File /oname=${APP_EXE} "target\release\snap.exe"

    ; Create the uninstaller EXE so users can remove Snap cleanly
    WriteUninstaller "$INSTDIR\Uninstall.exe"

    ; add uninstall entry
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" \
      "DisplayName" "${APP_NAME} ${APP_VER}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" \
      "UninstallString" '"$INSTDIR\Uninstall.exe"'

    ; Start Menu shortcut
    CreateDirectory "$SMPROGRAMS\${APP_NAME}"
    CreateShortCut "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk" "$INSTDIR\${APP_EXE}"
SectionEnd

Section "Uninstall"
    Delete "$INSTDIR\${APP_EXE}"
    RMDir "$INSTDIR"
    Delete "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk"
    RMDir "$SMPROGRAMS\${APP_NAME}"
    DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}"
SectionEnd
