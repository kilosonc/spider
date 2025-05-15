# Custom Flannel: A Rust-based Overlay Network Component for Kubernetes

## Overview

Custom Flannel is a Rust-based overlay network component for Kubernetes, inspired by Flannel. It provides basic IP address management (IPAM) and network communication between pods across different nodes.

## Components

1. **IPAM**: Manages the allocation of IP addresses to pods.
2. **Backend**: Handles network communication between pods.

## Usage

1. Deploy the overlay network using the provided script.
2. Configure your Kubernetes cluster to use this overlay network.

## Deployment

Run the following command to deploy the overlay network:

```sh
./deploy.sh
