
NETWORK ?= tunnetwork
SUBNET ?= 10.0.199.0/24
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
	# docker network rm $(NETWORK)
