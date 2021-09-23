fn main() {
    windows::build!(
        Windows::Win32::Foundation::BOOL,
        Windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls,
        Windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH,
        Windows::Win32::Foundation::HINSTANCE,
        Windows::Win32::System::Console::AllocConsole,
        Windows::Win32::System::Console::FreeConsole,
        Windows::Win32::System::LibraryLoader::GetModuleHandleA
    )
}
