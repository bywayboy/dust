#![allow(non_snake_case)]
#![allow(unused_assignments)]

use libc::{c_int,c_uint, uint32_t,c_void};

use super::wnd::DWnd;

pub type LPARAM = * const c_void;
pub type WPARAM = * const c_void;


pub type WndProc =extern "stdcall" fn (DWnd, u32, WPARAM, LPARAM)->c_int;
pub type WindowHookfn = extern "stdcall" fn(int,* const c_void, * const c_void)->c_int;

#[repr(C)]
pub struct __WIN_HANDLER{
  unused:int
}

pub type HMENU = * const __WIN_HANDLER;
pub type HINSTANCE = * const __WIN_HANDLER;
pub type HICON = * const __WIN_HANDLER;
pub type HBRUSH = * const __WIN_HANDLER;
pub type HCURSOR = * const __WIN_HANDLER;

#[repr(C)]
pub struct WNDCLASSEXW{
  pub cbSize:uint32_t,
  pub style:uint32_t,
  pub lpfnWndProc:WndProc,
  pub cbClsExtra:c_int,
  pub cbWndExtra:c_int,
  pub hInstance:HINSTANCE,
  pub hIcon:HICON,
  pub hCursor:HCURSOR,
  pub hbrBackground:HBRUSH,
  pub lpszMenuName:* const u16,
  pub lpszClassName:* const u16,
  pub hIconSm:HCURSOR,
}

#[repr(C)]
pub struct POINT{
  pub x:c_int,pub y:c_int
}

#[repr(C)]
pub struct MSG{
  pub handle: DWnd,
  pub msg: c_uint,
  pub wparam:c_int,
  pub lparam:c_int,
  pub time:uint32_t,
  pub pt:POINT
}

extern "stdcall"{
//Messages.
pub fn PostMessageW(hWnd:DWnd, msg:u32, wparam:WPARAM, lparam:LPARAM)->c_int;
pub fn PostQuitMessage(exitCode:c_int)->c_int;
fn GetMessageW(lpMsg:* mut MSG, hWnd:DWnd, wMsgFilterMin:u32, wMsgFilterMax:u32)->bool;
fn TranslateMessage(lpMsg:* const MSG)->c_int;
fn DispatchMessageW(lpMsg:* const MSG)->c_int;
pub fn IsDialogMessage(hWnd:DWnd, lpMsg:* const MSG)->bool;
}

impl MSG{
  pub fn GetMessage(&mut self,hWin:DWnd,wMsgFilterMin:u32, wMsgFilterMax:u32)->bool{
    unsafe{GetMessageW(self, hWin, wMsgFilterMin, wMsgFilterMax)}
  }
  pub fn TranslateMessage(&self)->int{
    unsafe{
      TranslateMessage(self) as int
    }
  }
  pub fn DispatchMessage(&self)->int{
    unsafe{
      DispatchMessageW(self)as int
    }
  }
  pub fn IsDialogMessage(&self,hWin:DWnd)->bool{
    unsafe{
      IsDialogMessage(hWin,self)
    }
  }
}


#[repr(C)]
pub struct INITCOMMONCONTROLSEX{
  pub dwSize: u32,
  pub dwICC:u32
}
