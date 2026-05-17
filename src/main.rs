use std::process::Command;
use std::path::Path;



fn main() {
    println!("Hello, world!");

     // 引数の取得: %1 (PFM) と %2 (WAV) に相当
//    let args: Vec<String> = env::args().collect();
//    if args.len() < 3 {
//        println!("使用法: {} <入力.pfm> <入力.wav>", args);
//        return;
//    }

    let input_pfm = "ame_no_hi_ni_toshokan_he.pfm"; //&args[2];
    let input_wav = "ame_no_hi_ni_toshokan_he.wav"; //&args[3];
    
    // 出力ファイル名の生成（入力PFMのベース名を使用）
    let output_name = format!("{}.mov", Path::new(input_wav).file_stem().unwrap().to_str().unwrap());

    println!("エンコードを開始します: {} + {} -> {}", input_pfm, input_wav, output_name);

    // 検証済みFFmpegコマンドの構築 [4, 5]
    let status = Command::new("ffmpeg")
        .args(&[
            "-loop", "1",
            "-framerate", "60", // 60fps固定 [6, 7]
            "-i", input_pfm,   // MMDのHDR PFMソース [8]
            "-i", input_wav,   // Cubase等のPCMソース [3]
            "-c:v", "libx264",
            "-profile:v", "high", // ニコニコ/YouTube推奨 [9, 10]
            "-pix_fmt", "yuv420p", // ニコニコ動画必須仕様 [6, 9]
            // 高精度な色空間変換: sRGBガンマをBT.709に変換 [6]
            "-vf", "setparams=color_primaries=bt709:color_trc=iec61966-2-1:colorspace=bt709,colorspace=all=bt709:itrc=iec61966-2-1",
            "-color_primaries", "bt709",
            "-color_trc", "bt709",
            "-colorspace", "bt709",
            "-g", "30", // GOP長: フレームレートの半分 [10]
            "-bf", "2", // YouTube推奨Bフレーム数 [10]
            "-flags", "+cgop", // Closed GOP [9]
            "-b:v", "60M", // 目標ビットレート
            "-movflags", "+faststart", // Web再生最適化 [11]
            "-c:a", "copy", // PCM音源を無劣化コピー（.movコンテナでサポート）[7]
            "-shortest", // 音声終端で停止 [12]
            "-y", // 上書き許可 [13]
            &output_name,
        ])
        .status()
        .expect("FFmpegの起動に失敗しました。パスが通っているか確認してください。");

    if status.success() {
        println!("エンコードが正常に完了しました: {}", output_name);
    } else {
        eprintln!("FFmpegがエラーを返しました。");
    }
}
