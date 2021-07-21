# Filters

In most cases, we would like Quilkin to do some preprocessing of received packets before sending them off to their destination. Because this stage is entirely specific to the use case at hand and differs between Quilkin deployments, we must have a say over what tweaks to perform - this is where filters come in.

### Filters and Filter chain
A filter represents a step in the tweaking/decision-making process of how we would like to process our packets. For example, at some step, we might choose to append some metadata to every packet we receive before forwarding it while at a later step, choose not to forward packets that don't meet some criteria.

Quilkin lets us specify any number of filters and connect them in a sequence to form a packet processing pipeline similar to a <a href="https://en.wikipedia.org/wiki/Pipeline_(Unix)" target="_blank">Unix pipeline</a> - we call this pipeline a `Filter chain`. The combination of filters and filter chain allows us to add new functionality to fit every scenario without changing Quilkin's core.

As an example, say we would like to perform the following steps in our processing pipeline to the packets we receive.

* Append a predetermined byte to the packet.
* Compress the packet.
* Do not forward (drop) the packet if its compressed length is over 512 bytes.

We would create a filter corresponding to each step either by leveraging any [existing filters](#built-in-filters) that do what we want or [writing one ourselves](#writing-filters) and connect them to form the following filter chain:

```bash
append | compress | drop
```

When Quilkin consults our filter chain, it feeds the received packet into `append` and forwards the packet it receives (if any) from `drop` - i.e the output of `append` becomes the `input` into `compress` and so on in that order.

There are a few things we note here:

* Although we have in this example, a filter called `drop`, every filter in the filter chain has the same ability to *drop* or *update* a packet - if any filter drops a packet then no more work needs to be done regarding that packet so the next filter in the pipeline never has any knowledge that the dropped packet ever existed.

* The filter chain is consulted for every received packet, and its filters are traversed in reverse order for packets travelling in the opposite direction.
  A packet received downstream will be fed into `append` and the result from `drop` is forwarded upstream - a packet received upstream will be fed into `drop` and the result from `append` is forwarded downstream.

* Exactly one filter chain is specified and used to process all packets that flow through Quilkin.

**Metrics**

* `filter_read_duration_seconds` The duration it took for a `filter`'s
  `read` implementation to execute.
  * Labels
    * `filter` The name of the filter being executed.

* `filter_write_duration_seconds` The duration it took for a `filter`'s
  `write` implementation to execute.
  * Labels
    * `filter` The name of the filter being executed.

### Configuration Examples ###

```rust
# // Wrap this example within an async main function since the
# // local_rate_limit filter spawns a task on initialization
# #[tokio::main]
# async fn main() {
# let yaml = "
version: v1alpha1
static:
  filters:
    - name: quilkin.extensions.filters.debug.v1alpha1.Debug
      config:
        id: debug-1
    - name: quilkin.extensions.filters.local_rate_limit.v1alpha1.LocalRateLimit
      config:
        max_packets: 10
        period: 500ms
  endpoints:
    - address: 127.0.0.1:7001
# ";
# let config = quilkin::config::Config::from_reader(yaml.as_bytes()).unwrap();
# assert_eq!(config.source.get_static_filters().unwrap().len(), 2);
# quilkin::proxy::Builder::from(std::sync::Arc::new(config)).validate().unwrap();
}
```

We specify our filter chain in the `.filters` section of the proxy's configuration which has takes a sequence of [FilterConfig](#filter-config) objects. Each object describes all information necessary to create a single filter.

The above example creates a filter chain comprising a [Debug](debug.md) filter followed by a [Rate limiter](./local_rate_limit.md) filter - the effect is that every packet will be logged and the proxy will not forward more than 20 packets per second.

> The sequence determines the filter chain order so its ordering matters - the chain starts with the filter corresponding the first filter config and ends with the filter corresponding the last filter config in the sequence.

### Filter Dynamic Metadata

A filter within the filter chain can share data within another filter further along in the filter chain by propagating the desired data alongside the packet being processed.
This enables sharing dynamic information at runtime, e.g information about the current packet that might be useful to other filters that process that packet.

At packet processing time each packet is associated with _filter dynamic metadata_ (a set of key-value pairs). Each key is a unique string while value is an arbitrary value.
When a filter processes a packet, it can choose to consult the associated dynamic metadata for more information or itself add/update or remove key-values from the set.

As an example, the built-in [CaptureBytes] filter is one such filter that populates a packet's filter metadata.
[CaptureBytes] extracts information (a configurable byte sequence) from each packet and appends it to the packet's dynamic metadata for other filters to leverage.
On the other hand, the built-in [TokenRouter] filter selects what endpoint to route a packet by consulting the packet's dynamic metadata for a routing token.
Consequently, we can build a filter chain with a [CaptureBytes] filter preceeding a [TokenRouter] filter, both configured to write and read the same key in the dynamic metadata entry. The effect would be that packets are routed to upstream endpoints based on token information extracted from their contents.

#### Well Known Dynamic Metadata

The following metadata are currently used by Quilkin core and built-in filters.

| Name | Type | Description |
|------|------|-------------|
| `quilkin.dev/captured_bytes` | `Vec<u8>` | The default key under which the [CaptureBytes] filter puts the byte slices it extracts from each packet. |

### Built-in filters <a name="built-in-filters"></a>
Quilkin includes several filters out of the box.

| Filter                                    | Description                    |
| ----------------------------------------- | ------------------------------ |
| [Debug](debug.md)                | Logs every packet              |
| [LocalRateLimiter](./local_rate_limit.md) | Limit the frequency of packets. |
| [ConcatenateBytes](./concatenate_bytes.md) | Add authentication tokens to packets. |
| [CaptureBytes](capture_bytes.md) | Capture specific bytes from a packet and store them in [filter dynamic metadata](#filter-dynamic-metadata). |
| [TokenRouter](token_router.md) | Send packets to endpoints based on metadata. |
| [Compress](./compress.md) | Compress and decompress packets data. |

### FilterConfig <a name="filter-config"></a>
Represents configuration for a filter instance.

```yaml
properties:
  name:
    type: string
    description: |
      Identifies the type of filter to be created.
      This value is unique for every filter type - please consult the documentation for the particular filter for this value.

  config:
    type: object
    description: |
      The configuration value to be passed onto the created filter.
      This is passed as an object value since it is specific to the filter's type and is validated by the filter
      implementation. Please consult the documentation for the particular filter for its schema.

required: [ 'name', 'config' ]
```

[CaptureBytes]: ./capture_bytes.md
[TokenRouter]: ./token_router.md
