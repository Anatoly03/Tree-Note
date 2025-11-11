# Check for Node.js dependencies
if [ ! -d "node_modules" ]; then
    npm install
fi

# Startup Tauri Development Server
(cd app-desktop; cargo tauri dev)
