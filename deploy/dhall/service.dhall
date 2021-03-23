let f = ./functions.dhall
let microservices = ./microservices.dhall

in

[ f.mkService microservices.antaeusInfo
, f.mkService microservices.providerInfo
]
