searchState.loadedDescShard("nmap_service_probes", 0, "nmap-service-probes\nExclude port on both TCP and UDP.\nA directive which requires a probe to be associated with, …\nPotential errors that can occur when constructing …\nExact matches should stop scanning after being successful.\nRepresents how to recognize services based on responses to …\nThe kind of match behaviour that should be used.\nA structured representation of a nmap-service-probes …\nAn error occurred while parsing the file, typically …\nThe port number or range of numbers.\nRepresents the set of ports and protocols used for …\nThe transport protocol of the service.\nA range of ports representing [start, end) (e.g. end is …\nCorresponds to how rare a given service is considered, and …\nA regular expression. These patterns are “Perl” or …\nRepresents the data needed to define a probe for a given …\nA single port.\nscanning continues after a softmatch success, but only on …\nTCP exclusion.\nUDP exclusion.\nEnables case insensitivity.\nPorts that should be excluded from scanning.\nProbes which should be used as fallbacks if no matches are …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nEnables including newlines in matches.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nMatch patterns for the response from a given service.\nThe name of the service.\nConstructs a new Regex from a given pattern.\nConstructs a new ServiceProbe with the required parameters.\nWhether to send a payload to ping or not (used to prevent …\nAccepts a nmap-service-probes document, returning …\nThe ports associated with a probe.\nThe transport protocol for the service.\nCorresponds to how rare a given service is considered, and …\nThe regex to match against the response from the service.\nThe name of the matching service.\nProbes for services.\nThe ports associated with a probe that require SSL.\nThe payload string to send to the service to ping it.\nOnly typically used with NULL probes, if a connection is …\nThe timeout duration on waiting for a response to a given …\ninfo on the specific version.")