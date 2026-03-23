#pragma once

#include <d3d10.h>

typedef HRESULT (WINAPI* PresentFn)(IDXGISwapChain*, UINT, UINT);
extern PresentFn o_Present;