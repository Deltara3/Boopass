#include <d3d10.h>
#include "shared.h"
#include "imgui.h"
#include "imgui_impl_dx10.h"
#include "imgui_impl_win32.h"

static bool g_Initialized = false;
static ID3D10Device* g_Device = nullptr;
static ID3D10RenderTargetView* g_RenderTarget = nullptr;

HRESULT WINAPI hk_Present(IDXGISwapChain* pSwapChain, UINT sync, UINT flags) {
    if (!g_Initialized) {
        pSwapChain->GetDevice(__uuidof(ID3D10Device), (void**)&g_Device);

        DXGI_SWAP_CHAIN_DESC desc;
        pSwapChain->GetDesc(&desc);

        ID3D10Texture2D* pBackBuffer;
        pSwapChain->GetBuffer(0, __uuidof(ID3D10Texture2D), (void**)&pBackBuffer);
        g_Device->CreateRenderTargetView(pBackBuffer, nullptr, &g_RenderTarget);
        pBackBuffer->Release();

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

    ImGui::ShowDemoWindow();

    ImGui::EndFrame();
    ImGui::Render();

    g_Device->OMSetRenderTargets(1, &g_RenderTarget, nullptr);
    ImGui_ImplDX10_RenderDrawData(ImGui::GetDrawData());

    return o_Present(pSwapChain, sync, flags);
}