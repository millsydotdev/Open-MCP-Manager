const { execSync } = require('node:child_process');

function checkDx() {
    console.log('🔍 Checking for Dioxus CLI...');
    try {
        execSync('dx --version', { stdio: 'ignore' });
        console.log('✅ Dioxus CLI found.');
    } catch {
        console.log('⚠️ Dioxus CLI (dx) not found.');
        console.log('📦 Installing dioxus-cli via cargo...');
        try {
            execSync('cargo install dioxus-cli', { stdio: 'inherit' });
            console.log('✅ Dioxus CLI installed successfully.');
        } catch (error) {
            console.error('❌ Failed to install dioxus-cli:', error);
            console.error('❌ Failed to install dioxus-cli. Please install it manually: cargo install dioxus-cli');
            process.exit(1);
        }
    }
}

checkDx();
