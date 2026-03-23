#include <windows.h>
#include <d3d10.h>
#include <string>
#include <sstream>

#define PATCH_SIZE 5

typedef HRESULT (WINAPI* CreateDXGIFactoryFn)(REFIID riid, void** ppFactory);
typedef HRESULT (WINAPI* CreateSwapChainFn)(IDXGIFactory*, IUnknown*, DXGI_SWAP_CHAIN_DESC*, IDXGISwapChain**);

static CreateDXGIFactoryFn o_CreateDXGIFactory = nullptr;
static CreateSwapChainFn o_CreateSwapChain = nullptr;

void ErrorBoxA(const std::string& text, UINT errorType);

HRESULT WINAPI hk_CreateSwapChain(IDXGIFactory* factory, IUnknown* device, DXGI_SWAP_CHAIN_DESC* desc, IDXGISwapChain** ppSwapChain) {
    return o_CreateSwapChain(factory, device, desc, ppSwapChain);
}

HRESULT WINAPI hk_CreateDXGIFactory(REFIID riid, void** ppFactory) {
    HRESULT hr = o_CreateDXGIFactory(riid, ppFactory);

    if (SUCCEEDED(hr) && ppFactory && *ppFactory) {
        IDXGIFactory* factory = (IDXGIFactory*)(*ppFactory);
        void** vtable = *reinterpret_cast<void***>(factory);

        DWORD oldProtect;

        if (!VirtualProtect(&vtable[10], sizeof(void*), PAGE_EXECUTE_READWRITE, &oldProtect)) {
            ErrorBoxA("Failed to enable writing for vtable, continuing without Boopass.", MB_ICONWARNING);
            goto exit;
        }

        o_CreateSwapChain = (CreateSwapChainFn)vtable[10];
        vtable[10] = (void*)&hk_CreateSwapChain;

        if (!VirtualProtect(&vtable[10], sizeof(void*), oldProtect, &oldProtect)) {
            ErrorBoxA("Failed to disable writing for vtable.", MB_ICONWARNING);
        }
    }

    exit:
    return hr;
}

extern "C" BOOL AttachHook() {
    HMODULE dxgi = GetModuleHandleA("dxgi.dll");

    if (dxgi == nullptr) {
        ErrorBoxA("Failed to retrieve handle for DXGI, continuing without Boopass.", MB_ICONWARNING);
        return FALSE;
    }

    void* target = GetProcAddress(dxgi, "CreateDXGIFactory");

    /* Probably don't need to error handle here, but oh well. */
    if (target == nullptr) {
        ErrorBoxA("Failed to get function address for CreateDXGIFactory, continuing without Boopass.", MB_ICONWARNING);
        return FALSE;
    }

    BYTE* detour = (BYTE*)VirtualAlloc(nullptr, PATCH_SIZE + 5, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);

    if (detour == nullptr) {
        ErrorBoxA("Failed to allocate memory for trampoline, continuing without Boopass.", MB_ICONWARNING);
        return FALSE;
    }

    memcpy(detour, target, PATCH_SIZE);

    uintptr_t returnAddr = (uintptr_t)target + PATCH_SIZE;
    uintptr_t relBack = returnAddr - ((uintptr_t)detour + PATCH_SIZE) - 5;

    detour[PATCH_SIZE] = 0xE9;
    *(uint32_t*)(detour + PATCH_SIZE + 1) = (uint32_t)relBack;

    DWORD oldProtect;

    if (!VirtualProtect(target, PATCH_SIZE, PAGE_EXECUTE_READWRITE, &oldProtect)) {
        ErrorBoxA("Failed to enable writing for patch region, continuing without Boopass.", MB_ICONWARNING);
        VirtualFree(detour, 0, MEM_RELEASE);
        return FALSE;
    }

    uintptr_t rel = (uintptr_t)hk_CreateDXGIFactory - (uintptr_t)target - 5;

    BYTE patch[5];
    patch[0] = 0xE9;
    *(uint32_t*)(patch + 1) = (uint32_t)rel;

    memcpy(target, patch, PATCH_SIZE);

    if (!VirtualProtect(target, PATCH_SIZE, oldProtect, &oldProtect)) {
        ErrorBoxA("Failed to disable writing for patch region.", MB_ICONWARNING);
    }

    FlushInstructionCache(GetCurrentProcess(), target, PATCH_SIZE);
    o_CreateDXGIFactory = (CreateDXGIFactoryFn)detour;

    return TRUE;
}

void ErrorBoxA(const std::string& text, UINT errorType) {
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

    MessageBoxA(nullptr, oss.str().c_str(), "Uh-oh!", MB_OK | errorType);

    if (msg) {
        LocalFree(msg);
    }
}