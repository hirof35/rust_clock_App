Rust Windows Clock
Rustとeguiライブラリを使用して作成された、シンプルで高速なデスクトップ用クロックツールです。
ストップウォッチ機能とカウントダウンタイマー機能を備えています。

🛠 特徴
ストップウォッチ: 1/1000秒単位の精度で計測、一時停止、リセットが可能。

カウントダウンタイマー: スライダーで直感的に時間を設定でき、バックグラウンドでの正確な減算処理を実装。

即時レスポンス: request_repaint()による滑らかなUI更新。

マルチプラットフォーム: Rustの特性を活かし、Windows、macOS、Linuxで動作可能。

🚀 始め方
前提条件
Rust (cargo) がインストールされていること。

インストールと実行
リポジトリをクローンします。

Cargo.tomlに以下の依存関係が含まれていることを確認してください。

Ini, TOML
[dependencies]
eframe = "0.24" # または最新バージョン
プロジェクトのルートディレクトリで以下のコマンドを実行します。

Bash
cargo run --release
📖 使い方
ストップウォッチ (StopWatch)
「Start」で計測開始。

「Stop」で一時停止（経過時間は保持されます）。

「Reset」で 0.000s に戻します。

タイマー (Timer)
スライダーを動かして秒数（最大3600秒）を設定します。

「Start」ボタンを押すとカウントダウンが始まります。

0秒になると自動的に停止します。

🛠 技術スタック
Language: Rust

GUI Framework: eframe / egui

Time Management: std::time::Instant / std::time::Duration

📂 コード構造
ClockApp 構造体: アプリケーションの状態（開始時間、経過時間、タイマー設定値）を管理。

update 関数: 1フレームごとに呼び出され、UIの描画と時間の計算を同時に処理。

⚖️ ライセンス
MIT License
