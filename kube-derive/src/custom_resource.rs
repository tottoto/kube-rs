// Generated by darling macros, out of our control
#![allow(clippy::manual_unwrap_or_default)]
use darling::{
    util::{parse_expr, Override},
    FromDeriveInput, FromMeta,
};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{ToTokens, TokenStreamExt as _};
use serde::Deserialize;
use syn::{parse_quote, Data, DeriveInput, Expr, Path, Visibility};

/// Values we can parse from #[kube(attrs)]
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(kube))]
struct KubeAttrs {
    group: String,
    version: String,
    kind: String,
    doc: Option<String>,
    #[darling(rename = "root")]
    kind_struct: Option<String>,
    /// lowercase plural of kind (inferred if omitted)
    plural: Option<String>,
    /// singular defaults to lowercased kind
    singular: Option<String>,
    #[darling(default)]
    namespaced: bool,
    #[darling(multiple, rename = "derive")]
    derives: Vec<String>,
    schema: Option<SchemaMode>,
    status: Option<Path>,
    #[darling(multiple, rename = "category")]
    categories: Vec<String>,
    #[darling(multiple, rename = "shortname")]
    shortnames: Vec<String>,
    #[darling(multiple, rename = "printcolumn")]
    printcolums: Vec<String>,
    #[darling(multiple)]
    selectable: Vec<String>,

    /// Customize the scale subresource, see [Kubernetes docs][1].
    ///
    /// [1]: https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/#scale-subresource
    scale: Option<Scale>,

    #[darling(default)]
    crates: Crates,
    #[darling(multiple, rename = "annotation")]
    annotations: Vec<KVTuple>,
    #[darling(multiple, rename = "label")]
    labels: Vec<KVTuple>,
    #[darling(multiple, rename = "validation", with = parse_expr::preserve_str_literal)]
    validations: Vec<Expr>,

    /// Sets the `storage` property to `true` or `false`.
    ///
    /// Defaults to `true`.
    #[darling(default = default_storage_arg)]
    storage: bool,

    /// Sets the `served` property to `true` or `false`.
    ///
    /// Defaults to `true`.
    #[darling(default = default_served_arg)]
    served: bool,

    /// Sets the `deprecated` and optionally the `deprecationWarning` property.
    ///
    /// See https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definition-versioning/#version-deprecation
    deprecated: Option<Override<String>>,
}

#[derive(Debug)]
struct KVTuple(String, String);

impl FromMeta for KVTuple {
    fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
        if items.len() == 2 {
            if let (
                darling::ast::NestedMeta::Lit(syn::Lit::Str(key)),
                darling::ast::NestedMeta::Lit(syn::Lit::Str(value)),
            ) = (&items[0], &items[1])
            {
                return Ok(KVTuple(key.value(), value.value()));
            }
        }

        Err(darling::Error::unsupported_format(
            "expected `\"key\", \"value\"` format",
        ))
    }
}

impl From<(&'static str, &'static str)> for KVTuple {
    fn from((key, value): (&'static str, &'static str)) -> Self {
        Self(key.to_string(), value.to_string())
    }
}

impl ToTokens for KVTuple {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let (k, v) = (&self.0, &self.1);
        tokens.append_all(quote! { (#k, #v) });
    }
}

fn default_storage_arg() -> bool {
    // This defaults to true to be backwards compatible.
    true
}

fn default_served_arg() -> bool {
    // This defaults to true to be backwards compatible.
    true
}

#[derive(Debug, FromMeta)]
struct Crates {
    #[darling(default = "Self::default_kube")]
    kube: Path,
    #[darling(default = "Self::default_kube_core")]
    kube_core: Path,
    #[darling(default = "Self::default_k8s_openapi")]
    k8s_openapi: Path,
    #[darling(default = "Self::default_schemars")]
    schemars: Path,
    #[darling(default = "Self::default_serde")]
    serde: Path,
    #[darling(default = "Self::default_serde_json")]
    serde_json: Path,
    #[darling(default = "Self::default_std")]
    std: Path,
}

// Default is required when the subattribute isn't mentioned at all
// Delegate to darling rather than deriving, so that we can piggyback off the `#[darling(default)]` clauses
impl Default for Crates {
    fn default() -> Self {
        Self::from_list(&[]).unwrap()
    }
}

impl Crates {
    fn default_kube_core() -> Path {
        parse_quote! { ::kube::core } // by default must work well with people using facade crate
    }

    fn default_kube() -> Path {
        parse_quote! { ::kube }
    }

    fn default_k8s_openapi() -> Path {
        parse_quote! { ::k8s_openapi }
    }

    fn default_schemars() -> Path {
        parse_quote! { ::schemars }
    }

    fn default_serde() -> Path {
        parse_quote! { ::serde }
    }

    fn default_serde_json() -> Path {
        parse_quote! { ::serde_json }
    }

    fn default_std() -> Path {
        parse_quote! { ::std }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SchemaMode {
    Disabled,
    Manual,
    Derived,
}

impl SchemaMode {
    fn derive(self) -> bool {
        match self {
            SchemaMode::Disabled => false,
            SchemaMode::Manual => false,
            SchemaMode::Derived => true,
        }
    }

    fn use_in_crd(self) -> bool {
        match self {
            SchemaMode::Disabled => false,
            SchemaMode::Manual => true,
            SchemaMode::Derived => true,
        }
    }
}

impl FromMeta for SchemaMode {
    fn from_string(value: &str) -> darling::Result<Self> {
        match value {
            "disabled" => Ok(SchemaMode::Disabled),
            "manual" => Ok(SchemaMode::Manual),
            "derived" => Ok(SchemaMode::Derived),
            x => Err(darling::Error::unknown_value(x)),
        }
    }
}

/// This struct mirrors the fields of `k8s_openapi::CustomResourceSubresourceScale` to support
/// parsing from the `#[kube]` attribute.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Scale {
    pub(crate) label_selector_path: Option<String>,
    pub(crate) spec_replicas_path: String,
    pub(crate) status_replicas_path: String,
}

// This custom FromMeta implementation is needed for two reasons:
//
// - To enable backwards-compatibility. Up to version 0.97.0 it was only possible to set scale
//   subresource values as a JSON string.
// - To be able to declare the scale sub-resource as a list of typed fields. The from_list impl uses
//   the derived implementation as inspiration.
impl FromMeta for Scale {
    /// This is implemented for backwards-compatibility. It allows that the scale subresource can
    /// be deserialized from a JSON string.
    fn from_string(value: &str) -> darling::Result<Self> {
        serde_json::from_str(value).map_err(darling::Error::custom)
    }

    fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
        let mut errors = darling::Error::accumulator();

        let mut label_selector_path: (bool, Option<Option<String>>) = (false, None);
        let mut spec_replicas_path: (bool, Option<String>) = (false, None);
        let mut status_replicas_path: (bool, Option<String>) = (false, None);

        for item in items {
            match item {
                darling::ast::NestedMeta::Meta(meta) => {
                    let name = darling::util::path_to_string(meta.path());

                    match name.as_str() {
                        "label_selector_path" => {
                            if !label_selector_path.0 {
                                let path = errors.handle(darling::FromMeta::from_meta(meta));
                                label_selector_path = (true, Some(path))
                            } else {
                                errors.push(
                                    darling::Error::duplicate_field("label_selector_path").with_span(&meta),
                                );
                            }
                        }
                        "spec_replicas_path" => {
                            if !spec_replicas_path.0 {
                                let path = errors.handle(darling::FromMeta::from_meta(meta));
                                spec_replicas_path = (true, path)
                            } else {
                                errors.push(
                                    darling::Error::duplicate_field("spec_replicas_path").with_span(&meta),
                                );
                            }
                        }
                        "status_replicas_path" => {
                            if !status_replicas_path.0 {
                                let path = errors.handle(darling::FromMeta::from_meta(meta));
                                status_replicas_path = (true, path)
                            } else {
                                errors.push(
                                    darling::Error::duplicate_field("status_replicas_path").with_span(&meta),
                                );
                            }
                        }
                        other => errors.push(darling::Error::unknown_field(other)),
                    }
                }
                darling::ast::NestedMeta::Lit(lit) => {
                    errors.push(darling::Error::unsupported_format("literal").with_span(&lit.span()))
                }
            }
        }

        if !spec_replicas_path.0 && spec_replicas_path.1.is_none() {
            errors.push(darling::Error::missing_field("spec_replicas_path"));
        }

        if !status_replicas_path.0 && status_replicas_path.1.is_none() {
            errors.push(darling::Error::missing_field("status_replicas_path"));
        }

        errors.finish()?;

        Ok(Self {
            label_selector_path: label_selector_path.1.unwrap_or_default(),
            spec_replicas_path: spec_replicas_path.1.unwrap(),
            status_replicas_path: status_replicas_path.1.unwrap(),
        })
    }
}

impl Scale {
    fn to_tokens(&self, k8s_openapi: &Path) -> TokenStream {
        let apiext = quote! {
            #k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1
        };

        let label_selector_path = self
            .label_selector_path
            .as_ref()
            .map_or_else(|| quote! { None }, |p| quote! { Some(#p.into()) });
        let spec_replicas_path = &self.spec_replicas_path;
        let status_replicas_path = &self.status_replicas_path;

        quote! {
            #apiext::CustomResourceSubresourceScale {
                label_selector_path: #label_selector_path,
                spec_replicas_path: #spec_replicas_path.into(),
                status_replicas_path: #status_replicas_path.into()
            }
        }
    }
}

pub(crate) fn derive(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let derive_input: DeriveInput = match syn::parse2(input) {
        Err(err) => return err.to_compile_error(),
        Ok(di) => di,
    };
    // Limit derive to structs
    match derive_input.data {
        Data::Struct(_) | Data::Enum(_) => {}
        _ => {
            return syn::Error::new_spanned(
                &derive_input.ident,
                r#"Unions can not #[derive(CustomResource)]"#,
            )
            .to_compile_error()
        }
    }

    let kube_attrs = match KubeAttrs::from_derive_input(&derive_input) {
        Err(err) => return err.write_errors(),
        Ok(attrs) => attrs,
    };

    let KubeAttrs {
        group,
        kind,
        kind_struct,
        version,
        doc,
        namespaced,
        derives,
        schema: schema_mode,
        status,
        plural,
        singular,
        categories,
        shortnames,
        printcolums,
        selectable,
        scale,
        validations,
        storage,
        served,
        deprecated,
        crates:
            Crates {
                kube_core,
                kube,
                k8s_openapi,
                schemars,
                serde,
                serde_json,
                std,
            },
        annotations,
        labels,
    } = kube_attrs;

    let struct_name = kind_struct.unwrap_or_else(|| kind.clone());
    if derive_input.ident == struct_name {
        return syn::Error::new_spanned(
            derive_input.ident,
            r#"#[derive(CustomResource)] `kind = "..."` must not equal the struct name (this is generated)"#,
        )
        .to_compile_error();
    }
    let visibility = derive_input.vis;
    let ident = derive_input.ident;

    // 1. Create root object Foo and truncate name from FooSpec

    // Default visibility is `pub(crate)`
    // Default generics is no generics (makes little sense to re-use CRD kind?)
    // We enforce metadata + spec's existence (always there)
    // => No default impl
    let rootident = Ident::new(&struct_name, Span::call_site());
    let rootident_str = rootident.to_string();

    // if status set, also add that
    let StatusInformation {
        field: status_field,
        default: status_default,
        impl_hasstatus,
    } = process_status(&rootident, &status, &visibility, &kube_core);
    let has_status = status.is_some();
    let serialize_status = if has_status {
        quote! {
            if let Some(status) = &self.status {
                obj.serialize_field("status", &status)?;
            }
        }
    } else {
        quote! {}
    };
    let has_status_value = if has_status {
        quote! { self.status.is_some() }
    } else {
        quote! { false }
    };

    let mut derive_paths: Vec<Path> = vec![
        syn::parse_quote! { #serde::Deserialize },
        syn::parse_quote! { Clone },
        syn::parse_quote! { Debug },
    ];
    let mut has_default = false;
    for d in &derives {
        if d == "Default" {
            has_default = true; // overridden manually to avoid confusion
        } else {
            match syn::parse_str(d) {
                Err(err) => return err.to_compile_error(),
                Ok(d) => derive_paths.push(d),
            }
        }
    }

    // Enable schema generation by default as in v1 it is mandatory.
    let schema_mode = schema_mode.unwrap_or(SchemaMode::Derived);
    // We exclude fields `apiVersion`, `kind`, and `metadata` from our schema because
    // these are validated by the API server implicitly. Also, we can't generate the
    // schema for `metadata` (`ObjectMeta`) because it doesn't implement `JsonSchema`.
    let schemars_skip = schema_mode.derive().then_some(quote! { #[schemars(skip)] });
    if schema_mode.derive() && !validations.is_empty() {
        derive_paths.push(syn::parse_quote! { #kube::KubeSchema });
    } else if schema_mode.derive() {
        derive_paths.push(syn::parse_quote! { #schemars::JsonSchema });
    }

    let struct_rules: Option<Vec<TokenStream>> =
        (!validations.is_empty()).then(|| validations.iter().map(|r| quote! {validation = #r,}).collect());
    let struct_rules = struct_rules.map(|r| quote! { #[x_kube(#(#r)*)]});

    let meta_annotations = if !annotations.is_empty() {
        quote! { Some(std::collections::BTreeMap::from([#((#annotations.0.to_string(), #annotations.1.to_string()),)*])) }
    } else {
        quote! { None }
    };

    let meta_labels = if !labels.is_empty() {
        quote! { Some(std::collections::BTreeMap::from([#((#labels.0.to_string(), #labels.1.to_string()),)*])) }
    } else {
        quote! { None }
    };

    let docstr =
        doc.unwrap_or_else(|| format!(" Auto-generated derived type for {ident} via `CustomResource`"));
    let quoted_serde = Literal::string(&serde.to_token_stream().to_string());
    let schemars_attribute = generate_schemars_attribute(schema_mode, &schemars);

    let root_obj = quote! {
        #[doc = #docstr]
        #[automatically_derived]
        #[allow(missing_docs)]
        #[derive(#(#derive_paths),*)]
        #[serde(rename_all = "camelCase")]
        #[serde(crate = #quoted_serde)]
        #schemars_attribute
        #struct_rules
        #visibility struct #rootident {
            #schemars_skip
            #visibility metadata: #k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
            #visibility spec: #ident,
            #status_field
        }
        impl #rootident {
            /// Spec based constructor for derived custom resource
            pub fn new(name: &str, spec: #ident) -> Self {
                Self {
                    metadata: #k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
                        annotations: #meta_annotations,
                        labels: #meta_labels,
                        name: Some(name.to_string()),
                        ..Default::default()
                    },
                    spec: spec,
                    #status_default
                }
            }
        }
        impl #serde::Serialize for #rootident {
            fn serialize<S: #serde::Serializer>(&self, ser: S) -> #std::result::Result<S::Ok, S::Error> {
                use #serde::ser::SerializeStruct;
                let mut obj = ser.serialize_struct(#rootident_str, 4 + usize::from(#has_status_value))?;
                obj.serialize_field("apiVersion", &<#rootident as #kube_core::Resource>::api_version(&()))?;
                obj.serialize_field("kind", &<#rootident as #kube_core::Resource>::kind(&()))?;
                obj.serialize_field("metadata", &self.metadata)?;
                obj.serialize_field("spec", &self.spec)?;
                #serialize_status
                obj.end()
            }
        }
    };

    // 2. Implement Resource trait
    let name = singular.unwrap_or_else(|| kind.to_ascii_lowercase());
    let plural = plural.unwrap_or_else(|| to_plural(&name));
    let (scope, scope_quote) = if namespaced {
        ("Namespaced", quote! { #kube_core::NamespaceResourceScope })
    } else {
        ("Cluster", quote! { #kube_core::ClusterResourceScope })
    };

    let api_ver = format!("{group}/{version}");
    let impl_resource = quote! {
        impl #kube_core::Resource for #rootident {
            type DynamicType = ();
            type Scope = #scope_quote;

            fn group(_: &()) -> std::borrow::Cow<'_, str> {
               #group.into()
            }

            fn kind(_: &()) -> std::borrow::Cow<'_, str> {
                #kind.into()
            }

            fn version(_: &()) -> std::borrow::Cow<'_, str> {
                #version.into()
            }

            fn api_version(_: &()) -> std::borrow::Cow<'_, str> {
                #api_ver.into()
            }

            fn plural(_: &()) -> std::borrow::Cow<'_, str> {
                #plural.into()
            }

            fn meta(&self) -> &#k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
                &self.metadata
            }

            fn meta_mut(&mut self) -> &mut #k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
                &mut self.metadata
            }
        }
    };

    // 3. Implement Default if requested
    let impl_default = if has_default {
        quote! {
            impl Default for #rootident {
                fn default() -> Self {
                    Self {
                        metadata: #k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta::default(),
                        spec: Default::default(),
                        #status_default
                    }
                }
            }
        }
    } else {
        quote! {}
    };

    // 4. Implement CustomResource

    // Compute a bunch of crd props
    let printers = format!("[ {} ]", printcolums.join(",")); // hacksss
    let fields: Vec<String> = selectable
        .iter()
        .map(|s| format!(r#"{{ "jsonPath": "{s}" }}"#))
        .collect();
    let fields = format!("[ {} ]", fields.join(","));
    let scale = scale.map_or_else(
        || quote! { None },
        |s| {
            let scale = s.to_tokens(&k8s_openapi);
            quote! { Some(#scale) }
        },
    );

    // Ensure it generates for the correct CRD version (only v1 supported now)
    let apiext = quote! {
        #k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1
    };
    let extver = quote! {
        #kube_core::crd::v1
    };

    let shortnames_slice = {
        let names = shortnames
            .iter()
            .map(|name| quote! { #name, })
            .collect::<TokenStream>();
        quote! { &[#names] }
    };

    let categories_json = serde_json::to_string(&categories).unwrap();
    let short_json = serde_json::to_string(&shortnames).unwrap();
    let crd_meta_name = format!("{plural}.{group}");

    let mut crd_meta = TokenStream::new();
    crd_meta.extend(quote! { "name": #crd_meta_name });

    if !annotations.is_empty() {
        crd_meta.extend(quote! { , "annotations": #meta_annotations });
    }

    if !labels.is_empty() {
        crd_meta.extend(quote! { , "labels": #meta_labels });
    }

    let schemagen = if schema_mode.use_in_crd() {
        quote! {
            // Don't use definitions and don't include `$schema` because these are not allowed.
            let generate = #schemars::generate::SchemaSettings::openapi3()
                .with(|s| {
                    s.inline_subschemas = true;
                    s.meta_schema = None;
                })
                .with_transform(#schemars::transform::AddNullable::default())
                .with_transform(#kube_core::schema::StructuralSchemaRewriter)
                .into_generator();
            let schema = generate.into_root_schema_for::<Self>();
        }
    } else {
        // we could issue a compile time warning for this, but it would hit EVERY compile, which would be noisy
        // eprintln!("warning: kube-derive configured with manual schema generation");
        // users must manually set a valid schema in crd.spec.versions[*].schema - see examples: crd_derive_no_schema
        quote! {
            let schema: Option<#k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps> = None;
        }
    };

    let selectable = if !selectable.is_empty() {
        quote! { "selectableFields": fields, }
    } else {
        quote! {}
    };

    let deprecation = if let Some(deprecation) = deprecated {
        match deprecation {
            Override::Inherit => quote! { "deprecated": true, },
            Override::Explicit(warning) => quote! {
                "deprecated": true,
                "deprecationWarning": #warning,
            },
        }
    } else {
        quote! {}
    };

    // Known constraints that are hard to enforce elsewhere
    let compile_constraints = quote! {}; // all modern features rolled out atm.

    let jsondata = quote! {
        #schemagen

        let jsondata = #serde_json::json!({
            "metadata": {
                #crd_meta
            },
            "spec": {
                "group": #group,
                "scope": #scope,
                "names": {
                    "categories": categories,
                    "plural": #plural,
                    "singular": #name,
                    "kind": #kind,
                    "shortNames": shorts
                },
                "versions": [{
                    "name": #version,
                    "served": #served,
                    "storage": #storage,
                    #deprecation
                    "schema": {
                        "openAPIV3Schema": schema,
                    },
                    "additionalPrinterColumns": columns,
                    #selectable
                    "subresources": subres,
                }],
            }
        });
    };

    // Implement the CustomResourceExt trait to allow users writing generic logic on top of them
    let impl_crd = quote! {
        impl #extver::CustomResourceExt for #rootident {

            fn crd() -> #apiext::CustomResourceDefinition {
                let columns : Vec<#apiext::CustomResourceColumnDefinition> = #serde_json::from_str(#printers).expect("valid printer column json");
                #k8s_openapi::k8s_if_ge_1_30! {
                    let fields : Vec<#apiext::SelectableField> = #serde_json::from_str(#fields).expect("valid selectableField column json");
                }
                let scale: Option<#apiext::CustomResourceSubresourceScale> = #scale;
                let categories: Vec<String> = #serde_json::from_str(#categories_json).expect("valid categories");
                let shorts : Vec<String> = #serde_json::from_str(#short_json).expect("valid shortnames");
                let subres = if #has_status {
                    if let Some(s) = &scale {
                        #serde_json::json!({
                            "status": {},
                            "scale": scale
                        })
                    } else {
                        #serde_json::json!({"status": {} })
                    }
                } else {
                    #serde_json::json!({})
                };

                #jsondata
                #serde_json::from_value(jsondata)
                    .expect("valid custom resource from #[kube(attrs..)]")
            }

            fn crd_name() -> &'static str {
                #crd_meta_name
            }

            fn api_resource() -> #kube_core::dynamic::ApiResource {
                #kube_core::dynamic::ApiResource::erase::<Self>(&())
            }

            fn shortnames() -> &'static [&'static str] {
                #shortnames_slice
            }
        }
    };

    let impl_hasspec = generate_hasspec(&ident, &rootident, &kube_core);

    // Concat output
    quote! {
        #compile_constraints
        #root_obj
        #impl_resource
        #impl_default
        #impl_crd
        #impl_hasspec
        #impl_hasstatus
    }
}

/// This generates the code for the `#kube_core::object::HasSpec` trait implementation.
///
/// All CRDs have a spec so it is implemented for all of them.
///
/// # Arguments
///
/// * `ident`: The identity (name) of the spec struct
/// * `root ident`: The identity (name) of the main CRD struct (the one we generate in this macro)
/// * `kube_core`: The path stream for the analagous kube::core import location from users POV
fn generate_hasspec(spec_ident: &Ident, root_ident: &Ident, kube_core: &Path) -> TokenStream {
    quote! {
        impl #kube_core::object::HasSpec for #root_ident {
            type Spec = #spec_ident;

            fn spec(&self) -> &#spec_ident {
                &self.spec
            }

            fn spec_mut(&mut self) -> &mut #spec_ident {
                &mut self.spec
            }
        }
    }
}

fn generate_schemars_attribute(schema_mode: SchemaMode, schemars_path: &Path) -> Option<TokenStream> {
    schema_mode.derive().then(|| {
        let schemars_path = schemars_path.to_token_stream().to_string();
        quote! { #[schemars(crate = #schemars_path)] }
    })
}

struct StatusInformation {
    /// The code to be used for the field in the main struct
    field: TokenStream,
    /// The initialization code to use in a `Default` and `::new()` implementation
    default: TokenStream,
    /// The implementation code for the `HasStatus` trait
    impl_hasstatus: TokenStream,
}

/// This processes the `status` field of a CRD.
///
/// As it is optional some features will be turned on or off depending on whether it's available or not.
///
/// # Arguments
///
/// * `root ident`: The identity (name) of the main CRD struct (the one we generate in this macro)
/// * `status`: The optional name of the `status` struct to use
/// * `visibility`: Desired visibility of the generated field
/// * `kube_core`: The path stream for the analagous kube::core import location from users POV
///
/// returns: A `StatusInformation` struct
fn process_status(
    root_ident: &Ident,
    status: &Option<Path>,
    visibility: &Visibility,
    kube_core: &Path,
) -> StatusInformation {
    if let Some(pth) = &status {
        StatusInformation {
            field: quote! {
                #[serde(skip_serializing_if = "Option::is_none")]
                #visibility status: Option<#pth>,
            },
            default: quote! { status: None, },
            impl_hasstatus: quote! {
                impl #kube_core::object::HasStatus for #root_ident {

                    type Status = #pth;

                    fn status(&self) -> Option<&#pth> {
                        self.status.as_ref()
                    }

                    fn status_mut(&mut self) -> &mut Option<#pth> {
                        &mut self.status
                    }
                }
            },
        }
    } else {
        let empty_quote = quote! {};
        StatusInformation {
            field: empty_quote.clone(),
            default: empty_quote.clone(),
            impl_hasstatus: empty_quote,
        }
    }
}

// Simple pluralizer.
// Duplicating the code from kube (without special casing) because it's simple enough.
// Irregular plurals must be explicitly specified.
fn to_plural(word: &str) -> String {
    // Words ending in s, x, z, ch, sh will be pluralized with -es (eg. foxes).
    if word.ends_with('s')
        || word.ends_with('x')
        || word.ends_with('z')
        || word.ends_with("ch")
        || word.ends_with("sh")
    {
        return format!("{word}es");
    }

    // Words ending in y that are preceded by a consonant will be pluralized by
    // replacing y with -ies (eg. puppies).
    if word.ends_with('y') {
        if let Some(c) = word.chars().nth(word.len() - 2) {
            if !matches!(c, 'a' | 'e' | 'i' | 'o' | 'u') {
                // Remove 'y' and add `ies`
                let mut chars = word.chars();
                chars.next_back();
                return format!("{}ies", chars.as_str());
            }
        }
    }

    // All other words will have "s" added to the end (eg. days).
    format!("{word}s")
}

#[cfg(test)]
mod tests {
    use std::{env, fs};

    use super::*;

    #[test]
    fn test_parse_default() {
        let input = quote! {
            #[derive(CustomResource, Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
            #[kube(group = "clux.dev", version = "v1", kind = "Foo", namespaced)]
            struct FooSpec { foo: String }
        };
        let input = syn::parse2(input).unwrap();
        let kube_attrs = KubeAttrs::from_derive_input(&input).unwrap();
        assert_eq!(kube_attrs.group, "clux.dev".to_string());
        assert_eq!(kube_attrs.version, "v1".to_string());
        assert_eq!(kube_attrs.kind, "Foo".to_string());
        assert!(kube_attrs.namespaced);
    }

    #[test]
    fn test_derive_crd() {
        let path = env::current_dir().unwrap().join("tests").join("crd_enum_test.rs");
        let file = fs::File::open(path).unwrap();
        runtime_macros::emulate_derive_macro_expansion(file, &[("CustomResource", derive)]).unwrap();

        let path = env::current_dir()
            .unwrap()
            .join("tests")
            .join("crd_schema_test.rs");
        let file = fs::File::open(path).unwrap();
        runtime_macros::emulate_derive_macro_expansion(file, &[("CustomResource", derive)]).unwrap();
    }
}
