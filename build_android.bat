@echo off
setlocal enabledelayedexpansion

echo ============================================
echo Kokoro G2P - Android Build Script
echo ============================================
echo.

:: Check for Android SDK
set "SDK_PATH=%LOCALAPPDATA%\Android\Sdk"
if not exist "%SDK_PATH%" (
    echo ERROR: Android SDK not found at %SDK_PATH%
    echo Please install Android Studio or set ANDROID_HOME manually.
    goto :error
)

:: Find NDK - check common locations
set "NDK_FOUND="
if exist "%SDK_PATH%\ndk" (
    for /d %%D in ("%SDK_PATH%\ndk\*") do (
        set "ANDROID_NDK_HOME=%%D"
        set "NDK_FOUND=1"
    )
)

if not defined NDK_FOUND (
    if exist "%SDK_PATH%\ndk-bundle" (
        set "ANDROID_NDK_HOME=%SDK_PATH%\ndk-bundle"
        set "NDK_FOUND=1"
    )
)

if not defined NDK_FOUND (
    echo ERROR: Android NDK not found!
    echo.
    echo Please install NDK using one of these methods:
    echo.
    echo 1. Android Studio:
    echo    - Open Android Studio
    echo    - Go to: Tools ^> SDK Manager ^> SDK Tools
    echo    - Check "NDK (Side by side)" and click Apply
    echo.
    echo 2. Command line:
    echo    sdkmanager "ndk;26.1.10909125"
    echo.
    echo 3. Manual download:
    echo    https://developer.android.com/ndk/downloads
    echo.
    goto :error
)

echo Found NDK: %ANDROID_NDK_HOME%
echo.

:: Check cargo-ndk is installed
where cargo-ndk >nul 2>&1
if errorlevel 1 (
    echo Installing cargo-ndk...
    cargo install cargo-ndk
    if errorlevel 1 goto :error
)

:: Check Rust Android targets
echo Checking Rust targets...
rustup target list --installed | findstr /C:"aarch64-linux-android" >nul
if errorlevel 1 (
    echo Installing Android targets...
    rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
)

:: Build
echo.
echo Building for Android (arm64-v8a)...
echo.
cargo ndk -t arm64-v8a -o jniLibs build --release --features "jni english"

if errorlevel 1 (
    echo.
    echo Build failed!
    goto :error
)

echo.
echo ============================================
echo Build successful!
echo ============================================
echo.
echo Output files:
dir /b jniLibs\arm64-v8a\*.so 2>nul
if errorlevel 1 (
    echo WARNING: No .so files found in jniLibs\arm64-v8a
) else (
    echo.
    echo Copy jniLibs folder to your Android project's app/src/main/
)
echo.
pause
exit /b 0

:error
echo.
echo Build failed. See errors above.
pause
exit /b 1
