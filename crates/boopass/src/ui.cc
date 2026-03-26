#include <d3d10.h>
#include "imgui.h"
#include "imgui_impl_dx10.h"
#include "imgui_impl_win32.h"

static ID3D10Device* g_Device = nullptr;
static ID3D10RenderTargetView* g_Target = nullptr;

extern LRESULT ImGui_ImplWin32_WndProcHandler(HWND hWnd, UINT msg, WPARAM wParam, LPARAM lParam);

extern "C" void ImGui_Init(HWND hwnd, ID3D10Device* device, ID3D10RenderTargetView* target) {
    g_Device = device;
    g_Target = target;

    ImGui::CreateContext();
    ImGui_ImplWin32_Init(hwnd);
    ImGui_ImplDX10_Init(g_Device);
}

extern "C" void ImGui_Draw() {
    ImGui_ImplDX10_NewFrame();
    ImGui_ImplWin32_NewFrame();
    ImGui::NewFrame();

    {
        ImGui::Begin("Boopass");

        // TODO: Everything in between.

        ImGui::End();
    }

    ImGui::EndFrame();
    ImGui::Render();

    g_Device->OMSetRenderTargets(1, &g_Target, nullptr);
    ImGui_ImplDX10_RenderDrawData(ImGui::GetDrawData());
}

extern "C" LRESULT ImGui_WndProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam) {
    return ImGui_ImplWin32_WndProcHandler(hwnd, msg, wParam, lParam);
}