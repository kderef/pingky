#include <windows.h>
#include <windowsx.h>
#include <wingdi.h>
#include <winuser.h>

const int WINDOW_W = 400;
const int WINDOW_H = 200;

typedef struct {
    HWND hwnd;
    UINT msg;
    WPARAM wParam;
    LPARAM lParam;
} WindowState;

void WindowPaint(const WindowState* ws) {
    static PAINTSTRUCT ps;
    HDC hdc = BeginPaint(ws->hwnd, &ps);
    
    // ... painting
    
    EndPaint(ws->hwnd, &ps);
}

LRESULT CALLBACK WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam) {
    WindowState window_state = {
        .hwnd = hwnd,
        .msg = uMsg,
        .wParam = wParam,
        .lParam = lParam
    };
    
    switch (uMsg) {
        case WM_PAINT:
            WindowPaint(&window_state);
            break;

        case WM_DESTROY:
            PostQuitMessage(0);
            break;

        default: break;
    }
    
    // the rest is handled by default
    return DefWindowProc(hwnd, uMsg, wParam, lParam);
}

int WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nShowCmd) {
    const char* class_name = "pingky";
    const char* window_text = "pingky";

    // create window class

    WNDCLASS wc = {0};

#if 0
    SHSTOCKICONINFO sii = {0};
    sii.cbSize = sizeof(SHSTOCKICONINFO);
    SHGetStockIconInfo(SIID_SOFTWARE, SHGSI_ICON, &sii);
    wc.hIcon = sii.hIcon;
    SHGetStockIconInfo(SIID_SOFTWARE, SHGSI_ICON | SHGSI_SMALLICON, &sii);
    wc.hIconSm = sii.hIcon;    
#endif

    wc.lpfnWndProc = WindowProc;
    wc.hInstance = hInstance;
    wc.lpszClassName = class_name;

    RegisterClass(&wc);
    

    // center window
    RECT window_center_pos;
    GetClientRect(GetDesktopWindow(), &window_center_pos);
    window_center_pos.left = window_center_pos.right/2 - WINDOW_W/2;
    window_center_pos.top = window_center_pos.bottom/2 - WINDOW_H/2;

    // create a window

    long window_style = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX;

    HWND hwnd = CreateWindowEx(
        0,
        class_name,
        window_text,
        window_style,
        window_center_pos.left,
        window_center_pos.top,
        WINDOW_W, 
        WINDOW_H,
        NULL,
        NULL,
        hInstance,
        NULL
    );

    if (hwnd == 0) {
        return 1;
    }


    ShowWindow(hwnd, nShowCmd);
    
    // message loop


    MSG msg;

    while (GetMessage(&msg, NULL, 0, 0) > 0) {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }

    return 0;
}

void WinMainCRTStartup(void) {
    HINSTANCE hInstance = GetModuleHandle(NULL);
    int result = WinMain(hInstance, NULL, NULL, SW_SHOWDEFAULT);
    ExitProcess(result);
}
