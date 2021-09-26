pub use Windows::Win32::{
    // For toy aroms
    Foundation::BOOL,
    Foundation::HINSTANCE,
    // For toy-arms-test
    System::Console::AllocConsole,
    System::Console::FreeConsole,
    System::Diagnostics::Debug::IMAGE_OPTIONAL_HEADER32,
    System::Diagnostics::Debug::IMAGE_OPTIONAL_HEADER64,
    System::LibraryLoader::DisableThreadLibraryCalls,
    System::LibraryLoader::GetModuleHandleA,
    System::SystemServices::DLL_PROCESS_ATTACH,
    System::SystemServices::IMAGE_DOS_HEADER,
    System::SystemServices::IMAGE_IMPORT_DESCRIPTOR,
    UI::KeyboardAndMouseInput::GetAsyncKeyState,
};
windows::include_bindings!();
