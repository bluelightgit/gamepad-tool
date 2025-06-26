<script setup lang="ts">
import GamepadTestPage from './views/RefactoredGamepadTestPage.vue'
</script>

<template>
  <div id="app-container">
    <header class="app-header">
      <h1 class="app-title">Gamepad Tool</h1>
      <p class="app-subtitle">Real-time Gamepad Monitoring & Testing</p>
    </header>
    
    <main class="main-content">
      <GamepadTestPage />
    </main>
    
    <footer class="app-footer">
      <p>Built with Tauri 2.0 & Vue 3</p>
    </footer>
  </div>
</template>

<style>
/* Global Styles */
:root {
  --primary-color: #42b983;
  --background-color: #f0f2f5;
  --container-bg-color: #ffffff;
  --text-color: #333;
  --border-color: #e0e0e0;
  --shadow-color: rgba(0, 0, 0, 0.1);
}

body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  background-color: var(--background-color);
  color: var(--text-color);
}

#app-container {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

.app-header {
  padding: 24px;
  background: var(--primary-color);
  color: white;
  text-align: center;
  box-shadow: 0 2px 8px var(--shadow-color);
}

.app-title {
  margin: 0;
  font-size: 2em;
  font-weight: 700;
}

.app-subtitle {
  margin: 4px 0 0;
  font-size: 1em;
  opacity: 0.9;
}

.main-content {
  flex: 1;
  padding: 24px;
  min-width: 0; /* 防止内容溢出 */
  box-sizing: border-box;
}

.app-footer {
  padding: 16px;
  text-align: center;
  font-size: 0.9em;
  color: #666;
  background: #f8f9fa;
  border-top: 1px solid var(--border-color);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .app-header {
    padding: 16px 12px;
  }
  
  .app-title {
    font-size: 1.6em;
  }
  
  .app-subtitle {
    font-size: 0.9em;
  }
  
  .main-content {
    padding: 16px 12px;
  }
  
  .app-footer {
    padding: 12px;
    font-size: 0.8em;
  }
}

@media (max-width: 480px) {
  .app-header {
    padding: 12px 8px;
  }
  
  .app-title {
    font-size: 1.4em;
  }
  
  .app-subtitle {
    font-size: 0.85em;
  }
  
  .main-content {
    padding: 12px 8px;
  }
  
  .app-footer {
    padding: 8px;
    font-size: 0.75em;
  }
}

@media (max-width: 360px) {
  .app-header {
    padding: 8px 6px;
  }
  
  .app-title {
    font-size: 1.2em;
  }
  
  .app-subtitle {
    font-size: 0.8em;
  }
  
  .main-content {
    padding: 8px 6px;
  }
}
</style>