# ⚙️ VoxTech ⛰️

## 概要
`VoxTech`はPure Rustで安全で高効率な3Dサンドボックスシミュレーションの実装を目標とするオープンソースプロジェクトです。

### 目標
Minecraftライクな連続自動地形生成システムを有する3D・FPS/TPS型のサンドボックス・アクション・シミュレーターとする。

### 要件
* Rust言語での実装・かつunsafeの利用は原則最小限に抑えること
* Windows/Linuxをターゲットとして、最低限のクロスプラットフォーム性を確保すること
* SIMDやマルチスレッド、キャッシュを意識したデータ構造とすること
  * 固定長配列や再帰的データ構造を活用し、データ格納効率とパフォーマンスを両立すること
  * 拡張Octree(64分木)をベースとしたマップ構造(Cell/Chunk/Sector/Region)をベースとした実装

## Link and Special thanks

### 🪟 ウィンドウ生成・OSイベント処理: [winit](https://github.com/rust-windowing/winit)

### ✒️ グラフィクス: [wgpu](https://github.com/gfx-rs/wgpu)

### 🖼️ 画像処理: [image](https://github.com/image-rs/image)

### 🔊 オーディオ: [rodio](https://github.com/RustAudio/rodio)

### 📈 線形代数: [nalgebra](https://github.com/dimforge/nalgebra)

### 📝 ロギング: [tracing](https://github.com/tokio-rs/tracing)

### 📃 シリアライズ／デシリアライズ: [serde](https://github.com/serde-rs/serde)

### ⛓️ 並行処理-ロッキング: [parking_lot](https://github.com/Amanieu/parking_lot)

### 📞 並行処理-チャネル: [crossbeam](https://github.com/crossbeam-rs/crossbeam)

### 📦 ハッシュテーブル: [hashbrown](https://github.com/rust-lang/hashbrown)

### その他多くのOSS/クレート
多大なる貢献に感謝しております！🥰