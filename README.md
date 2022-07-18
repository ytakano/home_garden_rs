# Rustで家庭菜園

温度・湿度・大気圧センサーには以下のBME280を利用した。I2Cで接続可能で、Raspberry Piからもすぐに利用できた。[BME280を利用するためのクレート](https://docs.rs/bme280/latest/bme280/index.html)もあり、今回はそちらを利用した。

<a href="https://www.amazon.co.jp/gp/product/B081GYFNF8?ie=UTF8&psc=1&linkCode=li2&tag=coredumped03-22&linkId=754efe0e6d4ac4ed0014b7430adb1bdc&language=ja_JP&ref_=as_li_ss_il" target="_blank"><img border="0" src="https://ws-fe.amazon-adsystem.com/widgets/q?_encoding=UTF8&ASIN=B081GYFNF8&Format=_SL160_&ID=AsinImage&MarketPlace=JP&ServiceVersion=20070822&WS=1&tag=coredumped03-22&language=ja_JP" ></a><img src="https://ir-jp.amazon-adsystem.com/e/ir?t=coredumped03-22&language=ja_JP&l=li2&o=9&a=B081GYFNF8" width="1" height="1" border="0" alt="" style="border:none !important; margin:0px !important;" />

以下が実際に接続した写真となる。右下の基板の左上にBME280がみえる。

![Rpi4_1](./fig/rpi4_1.jpeg)

裏側には以下のようにLEDを点灯している。LEDはRaspberry PIのGPIOからトランジスタで制御する。

![Rpi4_2](./fig/rpi4_2.jpeg)

白色LEDは青色のスペクトルがほとんどのため、昼光色やピンク緑のLEDも利用している。緑のスペクトルは葉っぱによってはじかれるが、高効率で少量吸収するらしいので少しだけ緑色のLEDを追加してある。

LEDの光は直進性が高いため、分散させるためにラメ入りのマニュキアをLEDに塗布している。

<a href="https://www.amazon.co.jp/%E3%81%A1%E3%81%B5%E3%82%8C-CHIFURE-%E3%81%A1%E3%81%B5%E3%82%8C-%E3%83%8D%E3%82%A4%E3%83%AB%E3%82%A8%E3%83%8A%E3%83%A1%E3%83%AB-003-%E3%82%B7%E3%83%AB%E3%83%90%E3%83%BC%E7%B3%BB%E3%83%A9%E3%83%A1/dp/B00YGNOW6A?__mk_ja_JP=%E3%82%AB%E3%82%BF%E3%82%AB%E3%83%8A&crid=V7ELK67PVTA6&keywords=%E3%83%9E%E3%83%8B%E3%82%AD%E3%83%A5%E3%82%A2+%E3%83%A9%E3%83%A1&qid=1658150440&sprefix=%E3%83%9E%E3%83%8B%E3%83%A5%E3%82%AD%E3%82%A2+%E3%83%A9%E3%83%A1%2Caps%2C229&sr=8-8&linkCode=li2&tag=coredumped03-22&linkId=c15235ebfc36be43733ab313e204414c&language=ja_JP&ref_=as_li_ss_il" target="_blank"><img border="0" src="https://ws-fe.amazon-adsystem.com/widgets/q?_encoding=UTF8&ASIN=B00YGNOW6A&Format=_SL160_&ID=AsinImage&MarketPlace=JP&ServiceVersion=20070822&WS=1&tag=coredumped03-22&language=ja_JP" ></a><img src="https://ir-jp.amazon-adsystem.com/e/ir?t=coredumped03-22&language=ja_JP&l=li2&o=9&a=B00YGNOW6A" width="1" height="1" border="0" alt="" style="border:none !important; margin:0px !important;" />

## 回路

回路は以下のとおり。LEDが多くてはんだ付けが大変だった。LEDの数はもう少し多くてもよいかも知れない。自分は地球に優しいので、鉛フリーはんだを利用したが、鉛フリーはんだはなかなか溶けないのでかなり苦労した。

はんだ付けや電子回路の初心者はオライリー・ジャパンの『Make Electronics』がおすすめ。普通の電子回路の教科書には載っていない実践的な内容が書かれている。電子回路の基礎的な部分は色々な教科書があるので一冊何かを買って、ダイオードとトランジスタの特性を学ぶのがよさそう。

<a href="https://www.amazon.co.jp/Make-Electronics-%E7%AC%AC2%E7%89%88-%E2%80%95%E4%BD%9C%E3%81%A3%E3%81%A6%E3%82%8F%E3%81%8B%E3%82%8B%E9%9B%BB%E6%B0%97%E3%81%A8%E9%9B%BB%E5%AD%90%E5%9B%9E%E8%B7%AF%E3%81%AE%E5%9F%BA%E7%A4%8E-PROJECTS/dp/4873118972?__mk_ja_JP=%E3%82%AB%E3%82%BF%E3%82%AB%E3%83%8A&crid=2DNMRHROUTWSS&keywords=%E3%82%AA%E3%83%A9%E3%82%A4%E3%83%AA%E3%83%BC+maker&qid=1658150707&sprefix=%E3%82%AA%E3%83%A9%E3%82%A4%E3%83%AA%E3%83%BC+make%2Caps%2C248&sr=8-10&linkCode=li2&tag=coredumped03-22&linkId=187c2b876b8dc70ffe74cc2406111775&language=ja_JP&ref_=as_li_ss_il" target="_blank"><img border="0" src="https://ws-fe.amazon-adsystem.com/widgets/q?_encoding=UTF8&ASIN=4873118972&Format=_SL160_&ID=AsinImage&MarketPlace=JP&ServiceVersion=20070822&WS=1&tag=coredumped03-22&language=ja_JP" ></a><img src="https://ir-jp.amazon-adsystem.com/e/ir?t=coredumped03-22&language=ja_JP&l=li2&o=9&a=4873118972" width="1" height="1" border="0" alt="" style="border:none !important; margin:0px !important;" />

ブルーバックスの『Raspberry Piで学ぶ電子工作』もけっこうよかったので、こちらもおすすめ。

<a href="https://www.amazon.co.jp/%E3%83%A9%E3%82%BA%E3%83%91%E3%82%A4%EF%BC%94%E5%AF%BE%E5%BF%9C-%E3%82%AB%E3%83%A9%E3%83%BC%E5%9B%B3%E8%A7%A3-%E6%9C%80%E6%96%B0-%EF%BC%B2%EF%BD%81%EF%BD%93%EF%BD%90%EF%BD%82%EF%BD%85%EF%BD%92%EF%BD%92%EF%BD%99-%EF%BC%B0%EF%BD%89%E3%81%A7%E5%AD%A6%E3%81%B6%E9%9B%BB%E5%AD%90%E5%B7%A5%E4%BD%9C-%E4%BD%9C%E3%82%8B%E3%80%81%E5%8B%95%E3%81%8B%E3%81%99%E3%80%81%E3%81%97%E3%81%8F%E3%81%BF%E3%81%8C%E3%82%8F%E3%81%8B%E3%82%8B%EF%BC%81-%E3%83%96%E3%83%AB%E3%83%BC%E3%83%90%E3%83%83%E3%82%AF%E3%82%B9-%E9%87%91%E4%B8%B8%E9%9A%86%E5%BF%97-ebook/dp/B089VKLLK8?__mk_ja_JP=%E3%82%AB%E3%82%BF%E3%82%AB%E3%83%8A&crid=2BQXEWYQ3NE4S&keywords=raspberry+pi+%E8%AC%9B%E8%AB%87%E7%A4%BE&qid=1658151089&sprefix=raspberry+pi+%E8%AC%9B%E8%AB%87%E7%A4%BE%2Caps%2C190&sr=8-4&linkCode=li2&tag=coredumped03-22&linkId=c74c69ebc8b5352aa62660a05fd9c542&language=ja_JP&ref_=as_li_ss_il" target="_blank"><img border="0" src="https://ws-fe.amazon-adsystem.com/widgets/q?_encoding=UTF8&ASIN=B089VKLLK8&Format=_SL160_&ID=AsinImage&MarketPlace=JP&ServiceVersion=20070822&WS=1&tag=coredumped03-22&language=ja_JP" ></a><img src="https://ir-jp.amazon-adsystem.com/e/ir?t=coredumped03-22&language=ja_JP&l=li2&o=9&a=B089VKLLK8" width="1" height="1" border="0" alt="" style="border:none !important; margin:0px !important;" />

### LED

![LED](./fig/circuit.svg)

### 温度・湿度・大気圧

![LED](./fig/bme280.svg)

## Database

データベースにはPostgreSQLを利用し、ORマッパにはdelselを利用している。
PostgreSQLは、`docker`フォルダ内の`docker-compose.yml`スクリプトから起動可能。
ファイル内に書かれているパスワードを適当に修正した方がよい。

```text
$ cd docker
$ mkdir data
$ docker-compose up -d
```
データベースを設定するために、
事前にdieselをインストールしておく必要あり。その際、依存ライブラリを`apt`でインストールしておかないと失敗するので注意。

```text
$ sudo apt install libsqlite3-dev libpq-dev default-libmysqlclient-dev
$ cargo install diesel_cli
```

dieselをインストール後、以下のように入力してデータベースを初期化する。

```text
$ echo DATABASE_URL=postgres://iot:db_password@localhost/iot > .env
$ diesel migration run
```

上記の`docker-compose.yml`を利用すると、adminerとGrafanaも同時に起動され、それぞれ、以下に接続することでWeb UIを操作可能。

- adminer: http://localhost:8080/
- Grafana: http://localhost:3000/

Grafanaへのログインは`admin/admin`で行う。
データベースへ接続するためには、データベースからPostgreSQLを選択し接続先ホストに`db`と設定する必要がある。
ユーザIDとパスワードは上記の`docker-compose.yml`で指定したとおりとなる。
つまり、上記の例だと、`iot/db_password`となる。
また、データベース名も`iot`となる。

Grafanaのパネルの時間軸を`datetime`として、温度、湿度、大気圧をグラフ化可能。データベース中のこれら情報に対応するキーは以下の通り。

- 温度: `temperature`
- 湿度: `relative_humidity`
- 大気圧: `atmospheric_pressure`

正しく設定すると、以下のように表示できる。

![Grafana](./fig/grafana.png)

## 写真撮影

今回はRaspi公式のカメラを利用して写真撮影を行う。
写真撮影は、libcameraというツールを用いると可能で、
これは以下のようにしてインストールできる。

```text
$ sudo apt install libcamera-apps
```

カメラを用いた撮影は以下のように行える。

```text
$ libcamera-jpeg -n -o test.jpg
```

libcameraは撮影後にプレビューをGUIで表示してくれるが、コマンドラインから実行するのには不要である。プレビューは`-n`オプションを指定すると無効にできる。

今回は、このコマンドをcronで定期的に実行するだけとする。自分は、``libcamera-jpeg -o `date "+%Y_%m%d_%H%M"`.jpg -n``というコマンドを実行するシェルスクリプトを作成しておき、それをcronで実行するようにした。
