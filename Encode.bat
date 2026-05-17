@echo off
chcp 65001
setlocal

echo 1番目の引数は: %1
echo 2番目の引数は: %2

REM 1. FFmpegをインストールします。すでにインストール済みならアップグレードします。
winget install Gyan.FFmpeg

REM 2. エンコードを実行します。 (最小限のオプション)
ffmpeg ^
-loop 1 ^
-framerate 60 ^
-i %1 -i %2 ^
-c:v libx264 ^
-profile:v high ^
-pix_fmt yuv420p ^
-vf "setparams=color_primaries=bt709:color_trc=iec61966-2-1:colorspace=bt709,colorspace=all=bt709:itrc=iec61966-2-1" ^
-color_primaries bt709 ^
-color_trc bt709 -colorspace bt709 ^
-g 30 ^
-bf 2 ^
-flags +cgop ^
-b:v 60M ^
-movflags +faststart ^
-c:a copy ^
-shortest output.mov

ECHO エンコードは、完了しました。

ECHO この黒い画面を閉じるには、スペースキーを押してください。

REM  なぜスペースキーかというと、連打したとしても人畜無害なので。
pause
endlocal
