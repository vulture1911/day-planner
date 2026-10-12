# Day Planner (Windows)
Build on Windows: install Node 20+, Rust (rustup) and Visual Studio C++ Build Tools, then:
    npm install
    npm run tauri build      (installer: src-tauri/target/release/bundle/nsis/)
    npm run dev              (run without installing)
No Windows machine? Push this folder to a GitHub repo and run the "Build Windows installer" workflow; download the .exe from the run's Artifacts.
Shortcuts: Ctrl+Alt+W shows/hides the corner widget. The tray icon opens the app, toggles the widget or quits. Closing the main window keeps the app running in the tray so reminders still fire.
