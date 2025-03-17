# Gamepad Tester

A Windows application for testing and analyzing gamepad/joystick inputs, built with Tauri 2.0 and Vue.js.

![Screenshot](public/image.png)

## Features

- Real-time display of gamepad input data
- Support for up to 4 controllers via XInput
- Polling rate measurement and analysis
- Adjustable frame rate display
- Configurable log size for data collection
- Joystick movement visualization
- Button state monitoring

## Technology Stack

- Frontend: Vue.js with TypeScript
- Backend: Rust with Tauri 2.0
- Input API: Windows XInput via windows-rs

## Development

### Prerequisites

- Node.js
- Rust
- Tauri CLI

### Setup

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.