@echo off
setlocal
cd /d "%~dp0"

echo ===================================================
echo  Codex Babel / The Symbiotic Truth Engine Launcher
echo ===================================================

echo [1/3] Verifying and building Rust Truth Engine CLI...
cargo build -p signal-protocol --bin truth_engine_cli
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Failed to compile Rust Truth Engine.
    pause
    exit /b %ERRORLEVEL%
)

echo [2/3] Setting up Python SMT / Z3 Backend...
cd backend
if not exist .venv (
    echo Creating virtual environment...
    python -m venv .venv
)
call .venv\Scripts\activate.bat
pip install -r requirements.txt
start "Truth Engine SMT Backend" cmd /k "uvicorn app.main:app --reload --port 8000"

echo [3/3] Setting up Vector Lens Frontend...
cd ..\frontend
call pnpm install
start "Vector Lens Web Frontend" cmd /k "pnpm dev"

echo.
echo ===================================================
echo  Systems Active:
echo   - Web UI:    http://localhost:3004
echo   - SMT API:   http://localhost:8000/docs
echo   - Rust CLI:  target\debug\truth_engine_cli.exe
echo ===================================================
pause
