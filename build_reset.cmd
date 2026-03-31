del deploy -Recurse

cd frontend
trunk build --release
xcopy .\dist\ ..\deploy\dist\ /y /s
cd ..
cd backend
cargo build --release

xcopy .\target\release\backend.exe ..\deploy\backend.exe /y
cd ..