NETWORK ?= tunnetwork
SUBNET ?= 10.200.1.0/24
IMAGE_NAME ?= closetool/example-container-route:v1.0.0

.PHONY: example-tun
example-tun:
	@echo "Running example-tun"
	sudo cargo run --example tun

.PHONY: example-tun-bridge
example-tun-bridge:
	docker network create --driver bridge --subnet $(SUBNET) $(NETWORK)
	docker build ./examples/tun-bridge -t $(IMAGE_NAME)
	docker run --rm -it -d --name client --network $(NETWORK) $(IMAGE_NAME)
	sudo cargo run --example tun-bridge

.PHONY: example-tun-bridge-clean
example-tun-bridge-clean:
	docker rm -f client
	docker network rm $(NETWORK)
