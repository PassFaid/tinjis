let f = ./functions.dhall
let microservices = ./microservices.dhall

in

[ f.mkDeployment microservices.antaeusInfo
, f.mkDeployment microservices.providerInfo
]
