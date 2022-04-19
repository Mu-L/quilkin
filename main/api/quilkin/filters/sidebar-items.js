initSidebarItems({"enum":[["Error","An error that occurred when attempting to create a [`Filter`] from a [`FilterFactory`]."]],"mod":[["capture",""],["compress",""],["concatenate_bytes",""],["debug",""],["drop",""],["firewall",""],["load_balancer",""],["local_rate_limit",""],["match",""],["pass",""],["prelude","Prelude containing all types and traits required to implement [`Filter`] and [`FilterFactory`]."],["token_router",""]],"struct":[["Capture",""],["Compress","Filter for compressing and decompressing packet data"],["ConcatenateBytes","The `ConcatenateBytes` filter’s job is to add a byte packet to either the beginning or end of each UDP packet that passes through. This is commonly used to provide an auth token to each packet, so they can be routed appropriately."],["ConvertProtoConfigError","An error representing failure to convert a filter’s protobuf configuration to its static representation."],["CreateFilterArgs","Arguments needed to create a new filter."],["Debug","Debug logs all incoming and outgoing packets"],["Drop","Always drops a packet, mostly useful in combination with other filters."],["FilterInstance","The value returned by [`FilterFactory::create_filter`]."],["FilterRegistry","Registry of all [`Filter`][crate::filters::Filter]s that can be applied in the system."],["FilterSet","A set of filters to be registered with a [`FilterRegistry`]."],["Firewall","Filter for allowing/blocking traffic by IP and port."],["LoadBalancer","Balances packets over the upstream endpoints."],["LocalRateLimit","A filter that implements rate limiting on packets based on the token-bucket algorithm.  Packets that violate the rate limit are dropped.  It only applies rate limiting on packets received from a downstream connection (processed through [`LocalRateLimit::read`]). Packets coming from upstream endpoints flow through the filter untouched."],["Match",""],["Pass","Allows a packet to pass through, mostly useful in combination with other filters."],["ReadContext","The input arguments to [`Filter::read`]."],["ReadResponse","The output of [`Filter::read`]."],["TokenRouter","Filter that only allows packets to be passed to Endpoints that have a matching connection_id to the token stored in the Filter’s dynamic metadata."],["WriteContext","The input arguments to [`Filter::write`]."],["WriteResponse","The output of [`Filter::write`]."]],"trait":[["Filter","Trait for routing and manipulating packets."],["FilterFactory","Provides the name and creation function for a given [`Filter`]."],["StaticFilter","Statically safe version of [`Filter`], if you’re writing a Rust filter, you should implement [`StaticFilter`] in addition to [`Filter`], as [`StaticFilter`] guarantees all of the required properties through the type system, allowing Quilkin take care of the virtual table boilerplate automatically at compile-time."]],"type":[["DynFilterFactory","An owned pointer to a dynamic [`FilterFactory`] instance."],["FilterMap","A map of [`FilterFactory::name`]s to [`DynFilterFactory`] values."]]});