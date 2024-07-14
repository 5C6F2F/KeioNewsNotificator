# KeioNewsNotificator

慶應義塾高等学校ホームページのニュースが更新された際にLINEでお知らせします。

## 設定方法

1. [`config.toml`における`line_api_token`](https://github.com/5C6F2F/KeioNewsNotificator/blob/a148894159f3c205bf21cc59f61e145da6157f44/resources/config.toml#L3)の二重引用符内に[LINE Notify](https://notify-bot.line.me/my/)で登録したAPI Tokenを入力してください。 
セキュリティ的にどうなのかと思いますが、面倒くさいので構いません。

1. Task Scheduler等で定期的に実行するよう好みに設定してください。

## ビルドするとき

パスの関係で、デバッグ時には`debug_run.bat`を、リリースビルド時には`release_build.bat`を使うとやりやすいです。
