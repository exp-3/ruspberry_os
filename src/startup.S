@ startup
  .global _start
  .align

_start:
  // stack pointerの設定
  ldr sp, =0x06400000

  // main呼び出し
  bl main

  // 終了
  b .
