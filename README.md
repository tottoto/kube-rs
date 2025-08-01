# kube-rs

[![Crates.io](https://img.shields.io/crates/v/kube.svg)](https://crates.io/crates/kube)
[![Rust 1.85](https://img.shields.io/badge/MSRV-1.85-dea584.svg)](https://github.com/rust-lang/rust/releases/tag/1.85.0)
[![Tested against Kubernetes v1.30 and above](https://img.shields.io/badge/MK8SV-v1.30-326ce5.svg)](https://kube.rs/kubernetes-version)
[![Best Practices](https://bestpractices.coreinfrastructure.org/projects/5413/badge)](https://bestpractices.coreinfrastructure.org/projects/5413)
[![Discord chat](https://img.shields.io/discord/500028886025895936.svg?logo=discord&style=plastic)](https://discord.gg/tokio)

A [Rust](https://rust-lang.org/) client for [Kubernetes](http://kubernetes.io) in the style of a more generic [client-go](https://github.com/kubernetes/client-go), a runtime abstraction inspired by [controller-runtime](https://github.com/kubernetes-sigs/controller-runtime), and a derive macro for [CRDs](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) inspired by [kubebuilder](https://book.kubebuilder.io/reference/generating-crd.html). Hosted by [CNCF](https://cncf.io/) as a [Sandbox Project](https://www.cncf.io/sandbox-projects/).

These crates build upon Kubernetes [apimachinery](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go) + [api concepts](https://kubernetes.io/docs/reference/using-api/api-concepts/) to enable generic abstractions. These abstractions allow Rust reinterpretations of reflectors, controllers, and custom resource interfaces, so that you can write applications easily.

## Installation

Select a version of `kube` along with the generated [k8s-openapi](https://github.com/Arnavion/k8s-openapi) structs at your chosen [Kubernetes version](https://kube.rs/kubernetes-version/):

```toml
[dependencies]
kube = { version = "1.1.0", features = ["runtime", "derive"] }
k8s-openapi = { version = "0.25.0", features = ["latest"] }
```

See [features](https://kube.rs/features/) for a quick overview of default-enabled / opt-in functionality.

## Upgrading

See [kube.rs/upgrading](https://kube.rs/upgrading/).
Noteworthy changes are highlighted in [releases](https://github.com/kube-rs/kube/releases), and archived in the [changelog](https://kube.rs/changelog/).

## Usage

See the **[examples directory](https://github.com/kube-rs/kube/blob/main/examples)** for how to use any of these crates.

- **[kube API Docs](https://docs.rs/kube/)**
- **[kube.rs](https://kube.rs)**

Official examples:

- [version-rs](https://github.com/kube-rs/version-rs): lightweight deployment `reflector` using axum
- [controller-rs](https://github.com/kube-rs/controller-rs): `Controller` of a crd inside actix

For real world projects see [ADOPTERS](https://kube.rs/adopters/).

## Api

The [`Api`](https://docs.rs/kube/latest/kube/struct.Api.html) is what interacts with Kubernetes resources, and is generic over [`Resource`](https://docs.rs/kube/latest/kube/trait.Resource.html):

```rust
use k8s_openapi::api::core::v1::Pod;
let pods: Api<Pod> = Api::default_namespaced(client);

let pod = pods.get("blog").await?;
println!("Got pod: {pod:?}");

let patch = json!({"spec": {
    "activeDeadlineSeconds": 5
}});
let pp = PatchParams::apply("kube");
let patched = pods.patch("blog", &pp, &Patch::Apply(patch)).await?;
assert_eq!(patched.spec.active_deadline_seconds, Some(5));

pods.delete("blog", &DeleteParams::default()).await?;
```

See the examples ending in `_api` examples for more detail.

## Custom Resource Definitions

Working with custom resources uses automatic code-generation via [proc_macros in kube-derive](https://docs.rs/kube/latest/kube/derive.CustomResource.html).

You need to add `#[derive(CustomResource, JsonSchema)]` and some `#[kube(attrs..)]` on a __spec__ struct:

```rust
#[derive(CustomResource, Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[kube(group = "kube.rs", version = "v1", kind = "Document", namespaced)]
pub struct DocumentSpec {
    title: String,
    content: String,
}
```

Then you can use the generated wrapper struct `Document` as a [`kube::Resource`](https://docs.rs/kube/*/kube/trait.Resource.html):

```rust
let docs: Api<Document> = Api::default_namespaced(client);
let d = Document::new("guide", DocumentSpec::default());
println!("doc: {:?}", d);
println!("crd: {:?}", serde_yaml::to_string(&Document::crd()));
```

There are a ton of kubebuilder-like instructions that you can annotate with here. See the [documentation](https://docs.rs/kube/latest/kube/derive.CustomResource.html) or the `crd_` prefixed [examples](https://github.com/kube-rs/kube/blob/main/examples) for more.

**NB:** `#[derive(CustomResource)]` requires the `derive` feature enabled on `kube`.

## Runtime

The `runtime` module exports the `kube_runtime` crate and contains higher level abstractions on top of the `Api` and `Resource` types so that you don't have to do all the `watch`/`resourceVersion`/storage book-keeping yourself.

### Watchers

A streaming interface (similar to informers) that presents [`watcher::Event`](https://docs.rs/kube/latest/kube/runtime/watcher/enum.Event.html)s and does automatic relists under the hood.

```rust
let api = Api::<Pod>::default_namespaced(client);
let stream = watcher(api, Config::default()).default_backoff().applied_objects();
```

This now gives a continual stream of events and you do not need to care about the watch having to restart, or connections dropping.

```rust
while let Some(event) = stream.try_next().await? {
    println!("Applied: {}", event.name_any());
}
```


Note the base items from a `watcher` stream are an abstraction above the native `WatchEvent` to allow for store buffering. If you are following along to "see what changed", you can use utilities from [`WatchStreamExt`](https://docs.rs/kube/latest/kube/runtime/trait.WatchStreamExt.html), such as `applied_objects` to get a more conventional stream.

## Reflectors

A `reflector` is a `watcher` with `Store` on `K`. It acts on all the `Event<K>` exposed by `watcher` to ensure that the state in the `Store` is as accurate as possible.

```rust
let nodes: Api<Node> = Api::all(client);
let lp = Config::default().labels("kubernetes.io/arch=amd64");
let (reader, writer) = reflector::store();
let rf = reflector(writer, watcher(nodes, lp));
```

At this point you can listen to the `reflector` as if it was a `watcher`, but you can also query the `reader` at any point.

### Controllers

A `Controller` is a `reflector` along with an arbitrary number of watchers that schedule events internally to send events through a reconciler:

```rust
Controller::new(root_kind_api, Config::default())
    .owns(child_kind_api, Config::default())
    .run(reconcile, error_policy, context)
    .for_each(|res| async move {
        match res {
            Ok(o) => info!("reconciled {:?}", o),
            Err(e) => warn!("reconcile failed: {}", Report::from(e)),
        }
    })
    .await;
```

Here `reconcile` and `error_policy` refer to functions you define. The first will be called when the root or child elements change, and the second when the `reconciler` returns an `Err`.

See the [controller guide](https://kube.rs/controllers/intro/) for how to write these.

## TLS

Uses [rustls](https://github.com/rustls/rustls) with `ring` provider (default) or `aws-lc-rs` provider (optional).

To switch [rustls providers](https://docs.rs/rustls/latest/rustls/crypto/struct.CryptoProvider.html), turn off `default-features` and enable the `aws-lc-rs` feature:

```toml
kube = { version = "1.1.0", default-features = false, features = ["client", "rustls-tls", "aws-lc-rs"] }
```

To switch to `openssl`, turn off `default-features`, and enable the `openssl-tls` feature:

```toml
kube = { version = "1.1.0", default-features = false, features = ["client", "openssl-tls"] }
```

This will pull in `openssl` and `hyper-openssl`. If `default-features` is left enabled, you will pull in two TLS stacks, and the default will remain as `rustls`.

## musl-libc

Kube will work with [distroless](https://github.com/kube-rs/controller-rs/blob/main/Dockerfile), [scratch](https://github.com/constellation-rs/constellation/blob/27dc89d0d0e34896fd37d638692e7dfe60a904fc/Dockerfile), and `alpine` (it's also possible to use alpine as a builder [with some caveats](https://github.com/kube-rs/kube/issues/331#issuecomment-715962188)).

## License

Apache 2.0 licensed. See LICENSE for details.
