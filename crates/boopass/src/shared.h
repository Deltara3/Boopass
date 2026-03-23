#pragma once

#include <d3d10.h>

typedef HRESULT (WINAPI* PresentFn)(IDXGISwapChain*, UINT, UINT);

extern PresentFn o_Present;
extern ID3D10Device* g_Device;