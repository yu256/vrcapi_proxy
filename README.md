# vrcapi_proxy

vrcapi_proxyは、Misskeyフォークの[akatsukey](https://github.com/yu256/akatsukey)のフロントエンドで使用するためのプロキシサーバーです。

メモリ使用量: 5MB程度(AArch64 MacOS / v1.0.0 時点)

データ通信量・クライアント側の負荷削減のためデータを加工して返します。

VRChat APIのトークンは定期的に失効するため、複数端末で同じトークンを使用するために任意の文字列で管理します。

## 使い方

### 既存のリリースが実行環境に対応している場合

1. [Releasesページ](https://github.com/yu256/vrcapi_proxy/releases)にアクセスします。
2. 環境に合ったリリースをダウンロードします。

### 対応していない場合

1. Rust環境をセットアップします。
2. `rustup default nightly` を実行して Nightly に切り替えます。
3. このリポジトリを `git clone https://github.com/yu256/vrcapi_proxy.git` でクローンします。
4. クローンしたリポジトリに移動し、`cargo build --release` を実行してバイナリを生成します。生成されたバイナリは `/target/release` ディレクトリに保存されます。

初回起動時（すぐに停止します）、ホームディレクトリの `/vrcapi_proxy` フォルダにjsonが生成されます。CORSの設定を行ってください。設定が完了したら再度起動します。

`localhost:8000`をlistenするので、Cloudflare Tunnelなりngrokなりで公開して使用してください。
