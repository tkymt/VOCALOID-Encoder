@echo off
chcp 65001
setlocal

echo 1番目の引数は: %1
echo 2番目の引数は: %2

REM 1. FFmpegをインストールします。すでにインストール済みならアップグレードします。
winget install Gyan.FFmpeg

REM 2. エンコードを実行します。 (最小限のオプション)
ffmpeg -loop 1 -framerate 60 -i %1 -i %2 -c:v libx264 -pix_fmt yuv420p -profile:v high -g 30 -c:a copy -shortest -movflags +faststart output.mov

ECHO エンコードは、完了しました。

ECHO この黒い画面を閉じるには、スペースキーを押してください。

REM  なぜスペースキーかというと、連打したとしても人畜無害なので。
pause
endlocal
