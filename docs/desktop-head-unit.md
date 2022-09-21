# Using the `Desktop Head Unit (DHU)` by Google

### Installation

Follow the instructions on [this website](https://developer.android.com/training/cars/testing#install_dhu_version_20) to install the desktop-head-unit emulator.

### Steps

1. Make sure developer mode is enabled on your phone.
2. Install Android Auto on your phone.
3. Enable developer mode by tapping on the title in the app several times quickly.
4. Once in developer mode, tap on the 3 dot menu on the top corner and click "Start head unit server".
5. Connect your device to the computer.
6. On your computer now, enable tcp forwarding via `adb forward tcp:5277 tcp:5277` (if this fails, you may need to call `adb kill-server`). You will need to perform this step any time you disconnect then reconnect your phone to the computer.
7. Start the DHU by calling `desktop-head-unit.exe` located in `$android_sdk/extras/google/auto`

Source: Edited from [StackOverflow](https://stackoverflow.com/a/34023421/13990026)
