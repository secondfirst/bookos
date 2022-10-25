- Xiaoマイコンが認識されずにビルド済みデバッガを配置できないエラー
  - 余っているケーブルの導線をむき出しにして、XiaoのRSTと書いてあるふたつのポイントに両端を数回当ててショートさせて点灯状態とすることで、ファイルシステムが認識された。


- cargo build でデバイスが見つからないエラー
-> udev配下にデバイス定義をしていなかった。

- openocdでcfgが見つからないエラー
$ openocd -f interface/cmsis-dap.cfg -f target/atsame5x.cfg
Open On-Chip Debugger 0.11.0-rc2+dev-gcafa0b5 (2021-02-20-22:15)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
embedded:startup.tcl:26: Error: Can't find interface/cmsis-dap.cfg
in procedure 'script' 
at file "embedded:startup.tcl", line 26

-> 手動インストールしていたが、/usr配下に対象プログラムのscriptやmanなどを移動していなかった。

- Wio Terminalへ接続できない？

$ openocd -f interface/cmsis-dap.cfg -f target/atsame5x.cfg
Open On-Chip Debugger 0.11.0-rc2+dev-gcafa0b5 (2021-02-20-22:15)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "swd". To override use 'transport select <transport>'.
Info : Listening on port 6666 for tcl connections
Info : Listening on port 4444 for telnet connections
Info : Using CMSIS-DAPv2 interface with VID:PID=0x2886:0x802f, serial=0565E6F850583234352E3120FF021542
Info : CMSIS-DAP: SWD  Supported
Info : CMSIS-DAP: JTAG Supported
Info : CMSIS-DAP: FW Version = 2.0.0
Info : CMSIS-DAP: Serial# = 0
Info : CMSIS-DAP: Interface Initialised (SWD)
Info : SWCLK/TCK = 0 SWDIO/TMS = 1 TDI = 0 TDO = 0 nTRST = 0 nRESET = 0
Info : CMSIS-DAP: Interface ready
Info : clock speed 2000 kHz
Error: Error connecting DP: cannot read IDR
Info : DAP init failed

-> FPCケーブルのWio terminal側接続の誤り。
   0.5mmピッチ、10ピンのコネクタの位置が変わっていたので、差し直した。

-> ピンのコネクタ接続間違い
   FPCコネクタのケーブルをつなげる面の番号が合っていなかった(1, 2, 4, 6のピン)。

-> ハンダ付け不備による接触不良
   - デバッグサーバ起動に成功する場合があったが、すぐに落ちたことから接触不良と判断しハンダの付け見直しを行う。

-> 起動順序
   次の順序で起動するようにした。
   1.WIO Terminal のスイッチをいれてブートローダモード後にhf2コマンドでアップロード。
   2.XIAO を接続して起動
   3.openocd でXiao上のデバッグサーバを起動。