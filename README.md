# home_garden_rs

## 回路

### LED

![LED](./fig/circuit.svg)

### 温度・湿度・大気圧

## Database

```text
$ echo DATABASE_URL=postgres://iot:db_password@localhost/iot > .env
$ diesel setup
$ diesel migration run
```
