#include <windows.h>
#include <d3d10.h>
#include <string>
#include <sstream>

#define PATCH_SIZE 5

typedef HRESULT (WINAPI* CreateDXGIFactoryFn)(
    REFIID riid,
    void** ppFactory
);

static CreateDXGIFactoryFn oCreateDXGIFactory = nullptr;

HRESULT WINAPI hk_CreateDXGIFactory(REFIID riid, void** ppFactory) {
    return oCreateDXGIFactory(riid, ppFactory);
}

void ErrorBoxA(HWND window, const std::string& text, UINT errorType) {
    DWORD err = GetLastError();

    LPSTR msg = nullptr;
    FormatMessageA(
        FORMAT_MESSAGE_ALLOCATE_BUFFER |
        FORMAT_MESSAGE_FROM_SYSTEM |
        FORMAT_MESSAGE_IGNORE_INSERTS,
        nullptr,
        err,
        MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT),
        (LPSTR)&msg,
        0,
        nullptr
    );

    std::ostringstream oss;
    oss << text << "\nReason: " << (msg ? msg : "Unknown error.");

    MessageBoxA(window, oss.str().c_str(), "Uh-oh!", errorType);

    if (msg) {
        LocalFree(msg);
    }
}

extern "C" BOOL AttachHook() {
    HMODULE dxgi = GetModuleHandleA("dxgi.dll");

    if (dxgi == nullptr) {
        ErrorBoxA(nullptr, "Failed to retrieve handle for DXGI.", MB_OK | MB_ICONERROR);
        return FALSE;
    }

    void* target = GetProcAddress(dxgi, "CreateDXGIFactory");

    /* Probably don't need to error handle here, but oh well. */
    if (target == nullptr) {
        ErrorBoxA(nullptr, "Failed to get function address for CreateDXGIFactory.", MB_OK | MB_ICONERROR);
        return FALSE;
    }

    BYTE* detour = (BYTE*)VirtualAlloc(nullptr, PATCH_SIZE + 5, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);

    if (detour == nullptr) {
        ErrorBoxA(nullptr, "Failed to allocate memory for trampoline.", MB_OK | MB_ICONERROR);
        return FALSE;
    }

    memcpy(detour, target, PATCH_SIZE);

    uintptr_t returnAddr = (uintptr_t)target + PATCH_SIZE;
    uintptr_t relBack = returnAddr - ((uintptr_t)detour + PATCH_SIZE) - 5;

    detour[PATCH_SIZE] = 0xE9;
    *(uint32_t*)(detour + PATCH_SIZE + 1) = (uint32_t)relBack;

    DWORD oldProtect;
    
    if (!VirtualProtect(target, PATCH_SIZE, PAGE_EXECUTE_READWRITE, &oldProtect)) {
        ErrorBoxA(nullptr, "Failed to enable writing for patch region.", MB_OK | MB_ICONERROR);
        VirtualFree(detour, 0, MEM_RELEASE);
        return FALSE;
    }

    uintptr_t rel = (uintptr_t)hk_CreateDXGIFactory - (uintptr_t)target - 5;

    BYTE patch[5];
    patch[0] = 0xE9;
    *(uint32_t*)(patch + 1) = (uint32_t)rel;

    memcpy(target, patch, PATCH_SIZE);

    if (!VirtualProtect(target, PATCH_SIZE, oldProtect, &oldProtect)) {
        ErrorBoxA(nullptr, "Failed to disable writing for patch region.", MB_OK | MB_ICONWARNING);
    }

    FlushInstructionCache(GetCurrentProcess(), target, PATCH_SIZE);
    oCreateDXGIFactory = (CreateDXGIFactoryFn)detour;

    return TRUE;
}