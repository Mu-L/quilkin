var sourcesIndex = {};
sourcesIndex["agones"] = {"name":"","files":["lib.rs","pod.rs"]};
sourcesIndex["quilkin"] = {"name":"","dirs":[{"name":"cli","files":["generate_config_schema.rs","manage.rs","run.rs"]},{"name":"config","dirs":[{"name":"watch","dirs":[{"name":"agones","files":["crd.rs"]}],"files":["agones.rs","fs.rs"]}],"files":["builder.rs","config_type.rs","error.rs","metrics.rs","slot.rs","watch.rs"]},{"name":"endpoint","files":["address.rs","locality.rs"]},{"name":"filters","dirs":[{"name":"capture","files":["affix.rs","config.rs","metrics.rs","regex.rs"]},{"name":"compress","files":["compressor.rs","config.rs","metrics.rs"]},{"name":"concatenate_bytes","files":["config.rs"]},{"name":"firewall","files":["config.rs","metrics.rs"]},{"name":"load_balancer","files":["config.rs","endpoint_chooser.rs"]},{"name":"local_rate_limit","files":["metrics.rs"]},{"name":"match","files":["config.rs","metrics.rs"]},{"name":"token_router","files":["metrics.rs"]}],"files":["capture.rs","chain.rs","compress.rs","concatenate_bytes.rs","debug.rs","drop.rs","error.rs","factory.rs","firewall.rs","load_balancer.rs","local_rate_limit.rs","match.rs","metadata.rs","pass.rs","read.rs","registry.rs","set.rs","token_router.rs","write.rs"]},{"name":"proxy","dirs":[{"name":"sessions","files":["error.rs","metrics.rs","session.rs","session_manager.rs"]}],"files":["health.rs","server.rs","sessions.rs"]},{"name":"utils","files":["debug.rs","net.rs"]},{"name":"xds","files":["client.rs","metrics.rs","resource.rs","server.rs"]}],"files":["admin.rs","cli.rs","cluster.rs","config.rs","endpoint.rs","filters.rs","lib.rs","metadata.rs","metrics.rs","prost.rs","proxy.rs","test_utils.rs","ttl_map.rs","utils.rs","xds.rs"]};
sourcesIndex["quilkin_macros"] = {"name":"","files":["include.rs","lib.rs"]};
createSourceSidebar();
