fn main() {
    windows::build!(
        // For toy-arms
        Windows::Win32::Foundation::BOOL,
        Windows::Win32::Foundation::HINSTANCE,
        Windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls,
        Windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH,
        Windows::Win32::UI::KeyboardAndMouseInput::GetAsyncKeyState,
        Windows::Win32::System::SystemServices::IMAGE_IMPORT_DESCRIPTOR,
        Windows::Win32::System::SystemServices::IMAGE_DOS_HEADER,
        Windows::Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS32,
        Windows::Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS64,
        Windows::Win32::System::Diagnostics::Debug::IMAGE_OPTIONAL_HEADER32,
        Windows::Win32::System::Diagnostics::Debug::IMAGE_OPTIONAL_HEADER64,
        // For toy-arms-test
        Windows::Win32::System::Console::AllocConsole,
        Windows::Win32::System::Console::FreeConsole,
        Windows::Win32::System::LibraryLoader::GetModuleHandleA,
    );
}
