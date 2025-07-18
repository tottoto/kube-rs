## Examples of how to use kube

This directory contains a number of examples showcasing various capabilities of
the `kube` crates.

All examples can be executed with:

```sh
cargo run --example $name
```

All examples enable logging via `RUST_LOG`. To enable deeper logging of the `kube` crates you can do:

```sh
RUST_LOG=info,kube=debug cargo run --example $name
```

## kube focused api examples
For a basic overview of how to use the `Api` try:

```sh
cargo run --example job_api
cargo run --example pod_api
cargo run --example dynamic_api
cargo run --example dynamic_jsonpath
cargo run --example log_stream -- kafka-manager-7d4f4bd8dc-f6c44
```

## kubectl light example

The `kubectl` light example supports `get`, `delete`, and `watch` on arbitrary resources:

```sh
cargo run --example kubectl -- get nodes
cargo run --example kubectl -- get pods -lapp.kubernetes.io/name=prometheus -n monitoring
cargo run --example kubectl -- watch pods --all
cargo run --example kubectl -- edit pod metrics-server-86cbb8457f-8fct5
cargo run --example kubectl -- delete pod metrics-server-86cbb8457f-8fct5
cargo run --example kubectl -- apply -f configmapgen_controller_crd.yaml
```

Supported flags are `-lLABELSELECTOR`, `-nNAMESPACE`, `--all`, and `-oyaml`.

There are also two other examples that serve as simplistic analogues of `kubectl logs` and `kubectl events`:

```sh
# tail logs
cargo run --example log_stream -- prometheus-promstack-kube-prometheus-prometheus-0 -c prometheus -f --since=3600
# get events for an object
cargo run --example event_watcher -- --for=Pod/prometheus-promstack-kube-prometheus-prometheus-0
```

## kube admission controller example
Admission controllers are web servers, where the apiserver talks to you. You don't necessarily require a `kube::Client` (unless you additionally need to cross-reference with information not present in the apiserver admissionrequest).

You will need certificates, and a web server app deployed behind a `Service`, or as we do here: running locally with a private ip that your `k3d` cluster can reach;

```sh
export ADMISSION_PRIVATE_IP=192.168.1.163
./admission_setup.sh
kubectl apply -f admission_crd.yaml
cargo run --example admission_controller

# separate shell:
kubectl apply -f admission_ok.yaml # should succeed and add a label
# -> admittest.kube.rs/ok created
kubectl apply -f admission_reject.yaml # should fail
# -> Error from server: error when creating "admission_reject.yaml": admission webhook "foo-admission.default.svc" denied the request: Resource contained 'illegal' label
```

## kube-derive focused examples
How deriving `CustomResource` works in practice, and how it interacts with the [schemars](https://github.com/GREsau/schemars/) dependency.

```sh
cargo run --example crd_api
cargo run --example crd_derive
cargo run --example crd_derive_schema
cargo run --example crd_derive_no_schema --no-default-features --features=openssl-tls,latest
cargo run --example cert_check # showcases partial typing with Resource derive
```

The `no_schema` one opts out from the default `schema` feature from `kube-derive` (and thus the need for you to derive/impl `JsonSchema`).

**However**: without the `schema` feature, it's left **up to you to fill in a valid openapi v3 schema**, as schemas are **required** for [v1::CustomResourceDefinitions](https://docs.rs/k8s-openapi/latest/k8s_openapi/apiextensions_apiserver/pkg/apis/apiextensions/v1/struct.CustomResourceDefinition.html), and the generated crd will be rejected by the apiserver if it's missing. As the last example shows, you can do this directly without `schemars`.

Note that these examples also contain tests for CI, and are invoked with the same parameters, but using `cargo test` rather than `cargo run`.

## kube-runtime focused examples

### watchers
These example watch a single resource and does some basic filtering on the watchevent stream:

```sh
# watch unready pods in the current namespace
cargo run --example pod_watcher
# watch all event events
cargo run --example event_watcher
# watch deployments, configmaps, secrets in the current namespace
cargo run --example multi_watcher
# watch broken nodes and cross reference with events api
cargo run --example node_watcher
# watch arbitrary, untyped objects across all namespaces
cargo run --example dynamic_watcher
# watch arbitrary, typed config map objects, with error toleration
cargo run --example errorbounded_configmap_watcher
```

The `node_` and `pod_` watcher also allows using [Kubernetes 1.27 Streaming lists](https://kubernetes.io/docs/reference/using-api/api-concepts/#streaming-lists) via `WATCHLIST=1`:

```sh
WATCHLIST=1 RUST_LOG=info,kube=debug cargo run --example pod_watcher
```

### controllers
Main example requires you creating the custom resource first:

```sh
kubectl apply -f configmapgen_controller_crd.yaml
cargo run --example configmapgen_controller &
kubectl apply -f configmapgen_controller_object.yaml
```

and the finalizer example (reconciles a labelled subset of configmaps):

```sh
cargo run --example secret_syncer
kubectl apply -f secret_syncer_configmap.yaml
kubectl delete -f secret_syncer_configmap.yaml
```

the finalizer is resilient against controller downtime (try stopping the controller before deleting).

### reflectors
These examples watch resources plus log from its queryable store:

```sh
# Watch namespaced pods and print the current pod count every event
cargo run --example pod_reflector
# Watch nodes for applied events and current active nodes
cargo run --example node_reflector
# Watch namespaced secrets for applied events and print secret keys in a task
cargo run --example secret_reflector
# Watch namespaced foo crs for applied events and print store info in task
cargo run --example crd_reflector
```

The `crd_reflector` will just await changes. You can run `kubectl apply -f crd-baz.yaml`, or `kubectl delete -f crd-baz.yaml -n default`, or `kubectl edit foos baz -n default` to verify that the events are being picked up.

## openssl
Disable default features and enable `openssl-tls`:

```sh
cargo run --example pod_watcher --no-default-features --features=openssl-tls,latest,runtime
```
