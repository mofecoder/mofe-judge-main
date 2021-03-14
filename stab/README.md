# stab-cafecoder-docker-rs

## cafe-docker-rs.yaml
openapi 形式で記述したAPI仕様

## Dockerfile
danielgtaylor/apisprout を利用しopenapi形式のyamlからスタブイメージを作成
portが8000なので.env に設定の必要あり



## 使い方
以下のコマンドでイメージを名前付きビルド
```
docker build ./ -t cafecoder
```

本体を起動

Responseを変更したい場合yamlのsample を編集