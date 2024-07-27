# bms-generator (BMS自動生成くん)

```
BMSを
  自動で
    作るのだ！
```

## 説明

DJゆかり (sinri氏) が開発した地力向上プログラムの改良を目指すプロジェクト

(TODO: ニコニコ動画のサービス停止中のため、動画をあとで貼る)

## 動作環境

- Rust

- Cargo

## つかいかた

1. `keysound-gen` でキー音を生成する

    ```
    $ cargo run --bin keysound-gen -- path/to/bms/folder
    ```

2. `bms-generator` で、キー音と同一のディレクトリに譜面を生成する

    ```
    cargo run --bin bms-generator-cli -- path/to/bms/folder/chart.bms
    ```

3. 🎉 Party Time!!! 🕺

## CLI のつかいかた

`--help` オプションでも確認できるよ

```
bms-generator-cli [OPTIONS] <FILENAME>
```

`<FILENAME>`: 出力 BMS のファイル名

### オプション

`--bpm`: BPM (デフォルトは150)

`--bars`: 小節数 (デフォルトは16)

`--seed`: 乱数のシード (省略した場合は現在時刻をシードとして使用)

## 実装済み / 実装予定の機能

- [ ] 最低限の BMS 生成機能

- [ ] 縦連の許容度設定

- [ ] 軸の補正機能

- [ ] もっとリッチなキー音

- [ ] 好きなメロディをキー音にする機能

- [ ] GUI (Web UI 予定)
