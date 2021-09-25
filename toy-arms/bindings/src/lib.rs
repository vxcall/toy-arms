pub use Windows::Win32::{
    // For toy aroms
    Foundation::BOOL,
    Foundation::HINSTANCE,
    System::LibraryLoader::DisableThreadLibraryCalls,
    System::SystemServices::DLL_PROCESS_ATTACH,
    UI::KeyboardAndMouseInput::GetAsyncKeyState,
    // For toy-arms-test
    System::Console::AllocConsole,
    System::Console::FreeConsole,
    System::LibraryLoader::GetModuleHandleA,
};
windows::include_bindings!();
