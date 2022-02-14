### How to run

First session:

```bash
$ cargo run --manifest-path smart-socket/Cargo.toml \
    --example smart_socket_tcp
```

Second session:

```bash
$ cargo run --manifest-path thermometer/Cargo.toml \
    --example thermometer_udp \
    -- 127.0.0.1:11700 127.0.0.1:11701 "thermometer-on-the-wall" 20
```

Third session:

```
$ cargo run --manifest-path web/Cargo.toml
```

Fourth session:

```
$ curl -XPOST 'http://localhost:8080/room' \
    -H 'Content-Type: application/json' \
    -d '{"name": "bedroom"}'

$ curl -XPOST 'http://localhost:8080/room/bedroom/device' \
    -H 'Content-Type: application/json' \
    -d '{
        "name": "thermometer-on-the-wall",
        "device_type": "thermometer",
        "description":
        "some description"
    }'

$ curl 'http://localhost:8080/room/bedroom/device' -XPOST -H 'Content-Type: application/json' \
    -d '{
        "name": "socket-near-the-bed",
        "device_type": "socket",
        "description": "some description"
    }'

$ curl -XPUT 'http://localhost:8080/room/bedroom/socket/connect' \
    -H 'Content-Type: application/json' \
    -d '{"name": "socket-near-the-bed", "host": "127.0.0.1:10701"}'

$ curl -XPOST 'http://localhost:8080/room/bedroom/socket/switch' \
    -H 'Content-Type: application/json' \
    -d '{"name": "socket-near-the-bed"}'

$ curl -XPUT 'http://localhost:8080/room/bedroom/receiver' \
    -H 'Content-Type: application/json' \
    -d '{"address": "127.0.0.1:11701"}'

$ curl 'http://localhost:8080/rooms' | jq
[
  {
    "name": "bedroom",
    "devices": [
      {
        "name": "thermometer-on-the-wall",
        "description": "some description"
      },
      {
        "name": "socket-near-the-bed",
        "description": "some description"
      }
    ]
  }
]

$ curl 'http://localhost:8080/report' | jq
[
  {
    "room": "bedroom",
    "device": "thermometer-on-the-wall",
    "summary": "20°C"
  },
  {
    "room": "bedroom",
    "device": "socket-near-the-bed",
    "summary": "turned on (2W)"
  }
]
```
