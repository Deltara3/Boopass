#include <d3d10.h>
#include "shared.h"

HRESULT WINAPI hk_Present(IDXGISwapChain* pSwapChain, UINT sync, UINT flags) {
    return o_Present(pSwapChain, sync, flags);
}