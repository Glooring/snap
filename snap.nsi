; snap.nsi — NSIS installer script for Snap

!include "MUI2.nsh"
RequestExecutionLevel admin

!define APP_NAME    "snap"
!define APP_VER     "7.2.0"
!define APP_EXE     "snap.exe"

Name "${APP_NAME} ${APP_VER}"
OutFile "${APP_NAME}-setup.exe"
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
