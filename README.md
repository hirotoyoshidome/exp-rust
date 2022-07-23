# exp-rust

### Build

```
rustc main.rs
./main
```

### docker

```
docker build -t myrust .
docker run --rm -it -d -v $(pwd):/var/rust --name myrust myrust:latest
docker exec -it myrust bash
```
