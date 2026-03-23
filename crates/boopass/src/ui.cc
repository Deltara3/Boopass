#include <d3d10.h>
#include "shared.h"
#include "imgui.h"
#include "imgui_impl_dx10.h"
#include "imgui_impl_win32.h"

LRESULT CALLBACK WndProc(HWND hWnd, UINT uMsg, WPARAM wParam, LPARAM lParam);
extern LRESULT ImGui_ImplWin32_WndProcHandler(HWND hWnd, UINT msg, WPARAM wParam, LPARAM lParam);

static bool g_Initialized = false;
static bool g_MenuShown = false;
static ID3D10Device* g_Device = nullptr;
static ID3D10RenderTargetView* g_RenderTarget = nullptr;
static WNDPROC o_WndProc = nullptr;

HRESULT WINAPI hk_Present(IDXGISwapChain* pSwapChain, UINT sync, UINT flags) {
    if (!g_Initialized) {
        pSwapChain->GetDevice(__uuidof(ID3D10Device), (void**)&g_Device);

        DXGI_SWAP_CHAIN_DESC desc;
        pSwapChain->GetDesc(&desc);

        ID3D10Texture2D* pBackBuffer;
        pSwapChain->GetBuffer(0, __uuidof(ID3D10Texture2D), (void**)&pBackBuffer);
        g_Device->CreateRenderTargetView(pBackBuffer, nullptr, &g_RenderTarget);
        pBackBuffer->Release();

        o_WndProc = (WNDPROC)SetWindowLongPtr(desc.OutputWindow, GWLP_WNDPROC, (intptr_t)WndProc);

        ImGui::CreateContext();
        ImGui_ImplWin32_Init(desc.OutputWindow);
        ImGui_ImplDX10_Init(g_Device);
        
        g_Initialized = true;
    }

    if (!g_Initialized) {
        return o_Present(pSwapChain, sync, flags);
    }

    ImGui_ImplDX10_NewFrame();
    ImGui_ImplWin32_NewFrame();
    ImGui::NewFrame();

    if (g_MenuShown) {
        while (ShowCursor(TRUE) < 0);

        ImGui::Begin("Boopass");

        // TODO: Everything between this.

        ImGui::End();
    } else {
        while (ShowCursor(FALSE) >= 0);
    }

    ImGui::EndFrame();
    ImGui::Render();

    g_Device->OMSetRenderTargets(1, &g_RenderTarget, nullptr);
    ImGui_ImplDX10_RenderDrawData(ImGui::GetDrawData());

    return o_Present(pSwapChain, sync, flags);
}

LRESULT CALLBACK WndProc(HWND hWnd, UINT uMsg, WPARAM wParam, LPARAM lParam) {
    if (uMsg == WM_KEYDOWN && wParam == VK_F12) {
        g_MenuShown = !g_MenuShown;
    }

    ImGui_ImplWin32_WndProcHandler(hWnd, uMsg, wParam, lParam);
    return CallWindowProc(o_WndProc, hWnd, uMsg, wParam, lParam);
}