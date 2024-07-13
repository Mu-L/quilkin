(function() {var type_impls = {
"xds":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Client%3CC%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/xds/client.rs.html#119-211\">source</a><a href=\"#impl-Client%3CC%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;C: <a class=\"trait\" href=\"xds/client/trait.ServiceClient.html\" title=\"trait xds::client::ServiceClient\">ServiceClient</a>&gt; <a class=\"struct\" href=\"xds/client/struct.Client.html\" title=\"struct xds::client::Client\">Client</a>&lt;C&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"method.connect\" class=\"method\"><a class=\"src rightside\" href=\"src/xds/client.rs.html#120\">source</a><h4 class=\"code-header\">pub async fn <a href=\"xds/client/struct.Client.html#tymethod.connect\" class=\"fn\">connect</a>(\n    identifier: <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.1/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>,\n    management_servers: <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"https://docs.rs/tonic/0.11.0/tonic/transport/channel/endpoint/struct.Endpoint.html\" title=\"struct tonic::transport::channel::endpoint::Endpoint\">Endpoint</a>&gt;\n) -&gt; <a class=\"type\" href=\"xds/type.Result.html\" title=\"type xds::Result\">Result</a>&lt;Self&gt;</h4></section></div></details>",0,"xds::client::AdsClient","xds::client::MdsClient"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-Client%3CC%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/xds/client.rs.html#109\">source</a><a href=\"#impl-Clone-for-Client%3CC%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;C: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"xds/client/trait.ServiceClient.html\" title=\"trait xds::client::ServiceClient\">ServiceClient</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"xds/client/struct.Client.html\" title=\"struct xds::client::Client\">Client</a>&lt;C&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xds/client.rs.html#109\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.1/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"xds/client/struct.Client.html\" title=\"struct xds::client::Client\">Client</a>&lt;C&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.77.1/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.1/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.1/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.77.1/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","xds::client::AdsClient","xds::client::MdsClient"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()